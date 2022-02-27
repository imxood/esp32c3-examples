use bevy::prelude::*;
use bevy_egui::{
    egui::{style::Margin, CentralPanel, Color32, Frame, TopBottomPanel},
    EguiContext,
};

use crate::{
    resource::window_event::WindowEvent,
    ui::{titlebar_ui::Titlebar, ui_state::UiState},
};

pub fn ui(
    mut egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut player_event: EventWriter<WindowEvent>,
) {
    let ctx = egui_ctx.ctx_mut();
    let ui_state = &mut *ui_state;

    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        Titlebar::show(ctx, ui, ui_state, &mut player_event)
    });

    // 设置背景
    let frame = Frame {
        margin: Margin::symmetric(0.0, 6.0),
        fill: Color32::from_rgb(42, 56, 115),
        ..Default::default()
    };

    TopBottomPanel::bottom("bottom_panel")
        .frame(frame)
        .show(ctx, |_ui| {});

    // 设置背景
    let frame = Frame {
        margin: Margin::symmetric(0., 0.),
        ..Default::default()
    };

    CentralPanel::default().frame(frame).show(ctx, |_ui| {});
}
