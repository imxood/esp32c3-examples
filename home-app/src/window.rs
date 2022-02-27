use std::{
    collections::{vec_deque::IterMut, VecDeque},
    mem::{discriminant, swap},
    sync::Arc,
};

use epi::*;
use parking_lot::RwLock;
use winit::window::Window;

use crate::{
    resource::{defines::APP_NAME, fonts, icons::Icons},
    ui::device_page::DevicePage,
};

pub struct MainWindow {
    window_handle: Option<Arc<RwLock<Window>>>,
    pages: Pages,
    ui_enabled: bool,
}

impl MainWindow {
    pub fn new() -> Self {
        let window = Self {
            window_handle: None,
            pages: Pages::default(),
            ui_enabled: true,
        };
        window
    }

    pub fn set_window_handle(&mut self, window_handle: Arc<RwLock<Window>>) {
        self.window_handle = Some(window_handle);
    }

    /// 从开头添加一个页面
    /// 显示每次 update 时, 绘制第一个页面
    pub fn add_page(&mut self, page: Page) {
        self.pages.add(page);
    }

    /// 弹出最后添加的页面
    pub fn remove_page(&mut self, id: Option<usize>) {
        self.pages.remove(id);
    }

    /// 根据索引重写页面
    pub fn modify_page(&mut self, id: usize, new_page: Page) -> Option<Page> {
        self.pages.modify(id, new_page)
    }
}

impl epi::App for MainWindow {
    fn name(&self) -> &str {
        APP_NAME
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        tracing::info!("app setup");
        if let Some(window) = &self.window_handle {
            // 设置图标
            window.read().set_window_icon(Some(Icons::logo().unwrap()));
        }

        ctx.set_fonts(fonts::chinese());

        // 添加第一个页面
        if self.window_handle.is_some() {
            let mut page = Page::default();
            page.add(Box::new(DevicePage::new(
                self.window_handle.as_ref().unwrap().clone(),
            )));
            self.add_page(page);
        }
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        for page in self.pages.iter_mut() {
            if let Some(action) = page.draw(ctx, frame) {
                match action {
                    PageAction::None => {}
                    PageAction::AddPage(page) => {
                        self.add_page(page);
                        break;
                    }
                    PageAction::RemovePage(id) => {
                        self.remove_page(id);
                        break;
                    }
                    PageAction::ModifyPage(index, p) => {
                        self.modify_page(index, p);
                        break;
                    }
                    PageAction::OpenWindow(ui_enabled) => {
                        self.ui_enabled = ui_enabled;
                    }
                    PageAction::CloseWindow => {
                        self.ui_enabled = true;
                    }
                }
            }
        }
    }
}

pub enum PageAction {
    None,
    // 添加一个新页面
    AddPage(Page),
    // 移除特定索引的页面
    RemovePage(Option<usize>),
    // 重写指定页面
    ModifyPage(usize, Page),
    /// 弹出窗口(允许背景ui)
    OpenWindow(bool),
    /// 关闭窗口
    CloseWindow,
}

impl PartialEq for PageAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::AddPage(_), Self::AddPage(_)) => true,
            (Self::ModifyPage(_, _), Self::ModifyPage(_, _)) => true,
            (Self::OpenWindow(_), Self::OpenWindow(_)) => true,
            _ => discriminant(self) == discriminant(other),
        }
    }
}

/// 一个基本页面, 在一个 Page 中可以嵌套多个 BasePage
/// 多个 BasePage 可以组合成 一个 Page
pub trait BasePage {
    /// 设置 base_page 的 id, 用于识别这个 base_page
    fn set_id(&mut self, id: usize) {}

    /// 获取 base_page id
    fn get_id(&self) -> usize {
        0
    }

    /// 关联 parent page 的 id
    fn set_pid(&mut self, id: usize);

    /// 得到关联的 parent page 的 id
    fn get_pid(&self) -> usize;

    /// 标题栏
    fn title_bar(&mut self, ctx: &egui::Context, frame: &Frame);

    /// 页面内容
    fn content(&mut self, ctx: &egui::Context, frame: &Frame) -> PageAction;

    /// 状态栏
    fn status_bar(&mut self, ctx: &egui::Context, frame: &Frame) {}

    /// 更新数据
    fn update(&mut self) {}

    /// 绘制
    fn draw(&mut self, ctx: &egui::Context, frame: &Frame) -> PageAction {
        self.title_bar(ctx, frame);
        self.status_bar(ctx, frame);
        let res = self.content(ctx, frame);
        res
    }
}

/// 一个页面, 是路由的基本单位
#[derive(Default)]
pub struct Page {
    id: usize,
    pid: usize,
    base_pages: VecDeque<Box<dyn BasePage>>,
}

impl Page {
    pub fn draw(&mut self, ctx: &egui::Context, frame: &Frame) -> Option<PageAction> {
        for page in self.base_pages.iter_mut() {
            let action = page.draw(ctx, frame);
            if !matches!(action, PageAction::None) {
                return Some(action);
            }
        }
        None
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn set_pid(&mut self, pid: usize) {
        self.pid = pid;
    }

    // 添加一个新的 BasePage
    pub fn add(&mut self, mut base_page: Box<dyn BasePage>) {
        base_page.set_pid(self.id);
        base_page.set_id(self.base_pages.len());
        self.base_pages.push_front(base_page);
    }

    // 移除特定索引的 BasePage, 如果索引为 None, 则移除最后一个
    pub fn remove(&mut self, id: Option<usize>) -> Option<Box<dyn BasePage>> {
        if let Some(id) = id {
            return self.base_pages.remove(id);
        }
        self.base_pages.pop_front()
    }
}

/// 页面列表
#[derive(Default)]
pub struct Pages(VecDeque<Page>);

impl Pages {
    /// 添加一个新页面到显示列表
    pub fn add(&mut self, mut page: Page) {
        page.set_id(self.0.len());
        self.0.push_front(page);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn modify(&mut self, id: usize, mut new_page: Page) -> Option<Page> {
        if let Some(page) = self.0.get_mut(id) {
            let id = page.get_id();
            swap(page, &mut new_page);
            page.set_id(id);
            return Some(new_page);
        }
        None
    }

    /// 移除特定索引的 BasePage, 如果索引为 None, 则移除最后一个
    pub fn remove(&mut self, id: Option<usize>) -> Option<Page> {
        if let Some(id) = id {
            return self.0.remove(id);
        }
        self.0.pop_front()
    }

    pub fn iter_mut(&mut self) -> IterMut<Page> {
        self.0.iter_mut()
    }
}

/// 顶部标题栏
pub trait TitleBar {
    fn draw(&mut self, ctx: &egui::Context, frame: &Frame);
}

/// 底部状态栏
pub trait StatusBar {
    fn draw(&mut self, ctx: &egui::Context, frame: &Frame);
}
