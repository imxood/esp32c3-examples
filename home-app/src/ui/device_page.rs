use epi::egui;
use parking_lot::RwLock;
use winit::window::Window;

use std::sync::Arc;

use crate::window::{BasePage, Page, PageAction, StatusBar, TitleBar};

use super::titlebar::MainTitlebar;

pub struct DevicePage {
    id: usize,
    pid: usize,
    open_test_window: bool,
    title_bar: MainTitlebar,
    status_bar: DeviceListBar,
    window_handle: Arc<RwLock<Window>>,
}

impl DevicePage {
    pub fn new(window_handle: Arc<RwLock<Window>>) -> Self {
        let title_bar = MainTitlebar::new(window_handle.clone());
        Self {
            id: 0,
            pid: 0,
            open_test_window: false,
            title_bar,
            status_bar: DeviceListBar::default(),
            window_handle,
        }
    }
}

impl BasePage for DevicePage {
    fn title_bar(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        self.title_bar.draw(ctx, frame);
    }

    fn status_bar(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        self.status_bar.draw(ctx, frame);
    }

    fn content(&mut self, ctx: &egui::Context, frame: &epi::Frame) -> PageAction {
        let mut res = PageAction::None;
        egui::CentralPanel::default().show(ctx, |ui| {
            tracing::debug!(
                "ui.available_rect_before_wrap(): {:?}",
                ui.available_rect_before_wrap()
            );

            ui.label("mqtt设备");

            ui.horizontal(|ui| {
                if ui.button("弹出窗口").clicked() {
                    self.open_test_window = true;
                }

                if self.open_test_window {
                    egui::Window::new("测试").show(ui.ctx(), |ui| {
                        ui.label("hello");
                        if ui.button("关闭窗口").clicked() {
                            self.open_test_window = false;
                        }
                    });
                    if !self.open_test_window {
                        res = PageAction::CloseWindow;
                        return;
                    }
                    res = PageAction::OpenWindow(false);
                    return;
                }
            });
        });

        res
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn get_id(&self) -> usize {
        todo!()
    }

    fn set_pid(&mut self, pid: usize) {
        self.pid = pid;
    }

    fn get_pid(&self) -> usize {
        self.pid
    }
}

#[derive(Default, Clone)]
struct DeviceListBar {}

impl StatusBar for DeviceListBar {
    fn draw(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        // .min_height(25.0)
        egui::TopBottomPanel::bottom("device_page_status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("状态栏");
                if ui.button("点击").clicked() {
                    ui.label("点击了");
                }
            });
        });
    }
}
