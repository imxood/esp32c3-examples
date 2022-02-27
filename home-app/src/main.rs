use std::{iter, sync::Arc};

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit::epi::{window_builder, EpiIntegration, Persistence};
use epi::egui::vec2;
use epi::{egui, App, NativeOptions};
use parking_lot::RwLock;
// use tracing_log::LogTracer;
use window::MainWindow;
use winit::dpi::PhysicalPosition;
use winit::event::*;
use winit::event_loop::ControlFlow;

mod resource;
mod ui;
mod window;

pub enum CustomEvent {
    RequestRedraw,
}

pub struct EventProxy(std::sync::Mutex<winit::event_loop::EventLoopProxy<CustomEvent>>);

impl epi::backend::RepaintSignal for EventProxy {
    fn request_repaint(&self) {
        self.0
            .lock()
            .unwrap()
            .send_event(CustomEvent::RequestRedraw)
            .ok();
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "INFO");

    // 在linux系统上, 使用gl驱动, 默认的Vulkan驱动会在屏幕关闭后 出现程序"Timeout"退出(2022-0405)
    if cfg!(target_os = "linux") {
        std::env::set_var("WGPU_BACKEND", "gl");
    }

    tracing_subscriber::fmt::init();

    // LogTracer::builder()
    //     .with_max_level(log::LevelFilter::Trace)
    //     .init()
    //     .expect("初始化 tracing 失败");

    let native_options = NativeOptions {
        decorated: false,
        initial_window_size: Some(vec2(1240.0, 720.0)),
        ..Default::default()
    };

    let mut app = MainWindow::new();

    let persistence = Persistence::from_app_name(app.name());
    let window_settings = persistence.load_window_settings();
    let window_builder = window_builder(&native_options, &window_settings).with_title(app.name());
    let event_loop = winit::event_loop::EventLoop::with_user_event();

    let event_proxy =
        std::sync::Arc::new(EventProxy(std::sync::Mutex::new(event_loop.create_proxy())));

    // 设置窗口居中
    let window = Arc::new(RwLock::new(window_builder.build(&event_loop).unwrap()));
    {
        let window = window.read();
        if let Some(monitor) = window.current_monitor() {
            let window_size = window.outer_size();
            let screen_size = monitor.size();
            let init_pos = PhysicalPosition::new(
                (screen_size.width - window_size.width) / 2,
                (screen_size.height - window_size.height) / 2,
            );
            window.set_outer_position(init_pos);
        }
    }

    app.set_window_handle(window.clone());

    let instance = wgpu::Instance::new(wgpu::Backends::all());

    let surface = unsafe { instance.create_surface(&*window.read()) };

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .unwrap();

    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ))
    .unwrap();

    let size = window.read().inner_size();
    let surface_format = surface.get_preferred_format(&adapter).unwrap();
    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &surface_config);

    let mut egui_rpass = RenderPass::new(&device, surface_format, 1);

    let persistence = Persistence::from_app_name(app.name());

    let mut integration = EpiIntegration::new(
        "egui_glow",
        4096,
        &*window.read(),
        event_proxy,
        persistence,
        Box::new(app),
    );

    let mut is_focused = true;

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            if !is_focused {
                // On Mac, a minimized Window uses up all CPU: https://github.com/emilk/egui/issues/325
                // We can't know if we are minimized: https://github.com/rust-windowing/winit/issues/208
                // But we know if we are focused (in foreground). When minimized, we are not focused.
                // However, a user may want an egui with an animation in the background,
                // so we still need to repaint quite fast.
                std::thread::sleep(std::time::Duration::from_millis(10));
            }

            let output_frame = match surface.get_current_texture() {
                Ok(frame) => frame,
                Err(wgpu::SurfaceError::Outdated) => {
                    // This error occurs when the app is minimized on Windows.
                    // Silently return here to prevent spamming the console with:
                    // "The underlying surface has changed, and therefore the swap chain must be updated"
                    return;
                }
                Err(e) => {
                    tracing::error!("Dropped frame with error: {}", e);
                    return;
                }
            };
            let output_view = output_frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let egui::FullOutput {
                platform_output,
                needs_repaint,
                textures_delta,
                shapes,
            } = integration.update(&*window.read());

            integration.handle_platform_output(&*window.read(), platform_output);

            let clipped_meshes = integration.egui_ctx.tessellate(shapes);

            // paint:
            {
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("encoder"),
                });

                // Upload all resources for the GPU.
                let screen_descriptor = ScreenDescriptor {
                    physical_width: surface_config.width,
                    physical_height: surface_config.height,
                    scale_factor: window.read().scale_factor() as f32,
                };

                egui_rpass
                    .add_textures(&device, &queue, &textures_delta)
                    .unwrap();
                egui_rpass.remove_textures(textures_delta).unwrap();
                egui_rpass.update_buffers(&device, &queue, &clipped_meshes, &screen_descriptor);

                // Record all render passes.
                egui_rpass
                    .execute(
                        &mut encoder,
                        &output_view,
                        &clipped_meshes,
                        &screen_descriptor,
                        Some(wgpu::Color::BLACK),
                    )
                    .unwrap();
                // Submit the commands.
                queue.submit(iter::once(encoder.finish()));

                // Redraw egui
                output_frame.present();
            }
            {
                *control_flow = if integration.should_quit() {
                    winit::event_loop::ControlFlow::Exit
                } else if needs_repaint {
                    window.read().request_redraw();
                    winit::event_loop::ControlFlow::Poll
                } else {
                    winit::event_loop::ControlFlow::Wait
                };
            }
            integration.maybe_autosave(&*window.read());
        };
        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            Event::RedrawRequested(_) if !cfg!(windows) => redraw(),

            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(size) => {
                        // Resize with 0 width and height is used by winit to signal a minimize event on Windows.
                        // See: https://github.com/rust-windowing/winit/issues/208
                        // This solves an issue where the app would panic when minimizing on Windows.
                        if size.width > 0 && size.height > 0 {
                            surface_config.width = size.width;
                            surface_config.height = size.height;
                            surface.configure(&device, &surface_config);
                        }
                    }
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Focused(new_focused) => {
                        is_focused = new_focused;
                    }
                    _ => {}
                }

                integration.on_event(&event);
                if integration.should_quit() {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }

                window.read().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }
            Event::LoopDestroyed => {
                integration.on_exit(&*window.read());
            }
            Event::UserEvent(CustomEvent::RequestRedraw) => {
                window.read().request_redraw();
            }
            _ => (),
        }
    });
}
