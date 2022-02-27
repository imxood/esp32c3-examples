use bevy::{
    app::AppExit,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::{WindowId, WindowMode},
    winit::WinitWindows,
};

use crate::{resource::window_event::WindowEvent, ui::ui_state::UiState};

pub fn update_window_event(
    mut ui_state: ResMut<UiState>,
    mut windows: ResMut<Windows>,
    mut exit: EventWriter<AppExit>,
    mut window_events: EventReader<WindowEvent>,
    winit_windows: Res<WinitWindows>,
    diagnostics: Res<Diagnostics>,
) {
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.average() {
            ui_state.fps = fps_avg;
        }
    }

    /*
        窗口控制
    */
    for event in window_events.iter() {
        match event {
            WindowEvent::Exit => {
                exit.send(AppExit);
            }
            WindowEvent::Fullscreen => {
                if let Some(window) = windows.get_primary_mut() {
                    let window_mode = window.mode();
                    if window_mode == WindowMode::Fullscreen {
                        ui_state.window_type.set(WindowMode::Windowed);
                    } else {
                        ui_state.window_type.set(WindowMode::Fullscreen);
                    }
                    window.set_mode(ui_state.window_type.get());
                }
            }
            WindowEvent::Maximize => {
                if let Some(window) = winit_windows.get_window(WindowId::primary()) {
                    ui_state.maximized = !ui_state.maximized;
                    window.set_maximized(ui_state.maximized);
                }
            }
            WindowEvent::Minimize => {
                if let Some(window) = windows.get_primary_mut() {
                    window.set_minimized(true);
                }
            }
            WindowEvent::DragWindow => {
                if let Some(window) = winit_windows.get_window(WindowId::primary()) {
                    window.drag_window().ok();
                }
            }
        }
    }
}
