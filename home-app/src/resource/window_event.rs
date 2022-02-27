/// 窗口事件
#[derive(Debug)]
pub enum WindowEvent {
    /// 应用离开
    Exit,
    /// 全屏
    Fullscreen,
    /// 最大化
    Maximize,
    /// 最小化
    Minimize,
    /// 拖拽窗口
    DragWindow,
}
