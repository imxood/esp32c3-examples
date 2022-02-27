use bevy::{prelude::*, window::WindowId, winit::WinitWindows};
use bevy_egui::{EguiContext, EguiSettings};

use crate::{
    resource::{error::Result, fonts::Fonts, icons::Icons},
    ui::ui_state::UiState,
};

/// egui环境初始化
pub fn setup(
    mut ui_state: ResMut<UiState>,
    mut egui_ctx: ResMut<EguiContext>,
    mut egui_settings: ResMut<EguiSettings>,
    mut windows: ResMut<Windows>,
    winit_windows: Res<WinitWindows>,
) {
    if let Some(window) = windows.get_primary_mut() {
        window.set_maximized(ui_state.maximized);
        window.set_mode(ui_state.window_type.get());
    }

    egui_settings.scale_factor = ui_state.scale_factor;

    let ctx = egui_ctx.ctx_mut();

    ctx.set_fonts(Fonts::chinese());

    ctx.set_style(ui_state.theme.blue_style_clone());

    if let Err(e) = init_icon(winit_windows) {
        ui_state.fatal_error = Some(e.to_string());
    }
}

/// 设置应用图标
pub fn init_icon(windows: Res<WinitWindows>) -> Result<()> {
    let primary = windows.get_window(WindowId::primary()).unwrap();

    let icon = Icons::logo()?;

    primary.set_window_icon(Some(icon));

    Ok(())
}
