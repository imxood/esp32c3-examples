use crate::{resource::defines::APP_NAME, window::TitleBar};

use epi::egui::{self, Align2, Context, Direction, Layout, ScrollArea, Sense};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use winit::window::{Fullscreen, Window};

use super::ui_state::UiState;

pub struct MainTitlebar {
    pub style_ui_open: bool,
    pub window_handle: Arc<RwLock<Window>>,
}

impl MainTitlebar {
    pub fn new(window_handle: Arc<RwLock<Window>>) -> Self {
        Self {
            style_ui_open: true,
            window_handle,
        }
    }
}

impl TitleBar for MainTitlebar {
    fn draw(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::TopBottomPanel::top("device_page_title_bar")
            // .min_height(25.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // 设置 标题栏 的样式
                    // ui.set_style(ui_state.theme.blue_titlebar_style_clone());

                    ui.with_layout(egui::Layout::left_to_right(), |ui| {
                        // ui.menu_button("样式", |ui| {
                        //     ui_state.titlebar.trigger_style_ui();
                        //     ui.close_menu();
                        // });
                    });
                    ui.with_layout(Layout::right_to_left(), |ui| {
                        let window = self.window_handle.read();
                        // 关闭窗口
                        if ui.button("✖").clicked() {
                            frame.quit();
                        }
                        // 最大化
                        if ui.button("⛶").clicked() {
                            let mut fullscreen = Option::<Fullscreen>::None;
                            if let Some(fullscreen_) = window.fullscreen() {
                                fullscreen = None;
                            } else {
                                fullscreen = Some(Fullscreen::Borderless(None));
                            }
                            window.set_fullscreen(fullscreen);
                        }
                        // 最小化
                        if ui.button("➖").clicked() {
                            window.set_minimized(true);
                        }

                        // ui.label(format!("{:.1}", ui_state.fps));

                        // 设置
                        // if ui.button("⛭").clicked() {
                        //     ui_state.setting_window.trigger_show();
                        // }

                        // 标题
                        let (title_rect, res) =
                            ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());

                        ui.allocate_ui_at_rect(title_rect, |ui| {
                            ui.with_layout(
                                Layout::centered_and_justified(Direction::LeftToRight),
                                |ui| {
                                    ui.label(APP_NAME);
                                },
                            );
                        });

                        // 双击, 如果是最大化
                        if res.double_clicked() {
                            // 如果已经是全屏状态, 就退出全屏, 并不需要反转最大化(不然看着很难受)
                            if window.fullscreen().is_some() {
                                window.set_fullscreen(None);
                            } else {
                                window.set_maximized(!window.is_maximized());
                            }
                        } else if res.dragged() {
                            // 当拖动时, 如果不判断drag_delta, 直接进行 drag_window, 会导致 double_clicked 无法触发
                            let delta = res.drag_delta();
                            if delta.x != 0.0 && delta.y != 0.0 {
                                window.drag_window().ok();
                            }
                        }
                    });
                });
            });
    }
}

impl MainTitlebar {
    pub fn trigger_style_ui(&mut self) {
        self.style_ui_open = !self.style_ui_open;
    }

    pub fn style_ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("style_ui")
            .collapsible(false)
            .open(&mut self.style_ui_open)
            .anchor(Align2::CENTER_CENTER, [0.0, -30.0])
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ctx.style_ui(ui);
                });
            });
    }
}

#[derive(Serialize, Deserialize)]
pub enum WindowType {
    /// Creates a window that uses the given size
    Windowed,
    /// Creates a borderless window that uses the full size of the screen
    BorderlessFullscreen,
    /// Creates a fullscreen window that will render at desktop resolution. The app will use the closest supported size
    /// from the given size and scale it to fit the screen.
    SizedFullscreen,
    /// Creates a fullscreen window that uses the maximum supported size
    Fullscreen,
}
