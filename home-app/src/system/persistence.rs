use crate::{data::storage::persistence::Persistence, ui::ui_state::UiState};

use bevy::prelude::*;

pub fn persistence_system(ui_state: Res<UiState>, mut persistence: ResMut<Persistence>) {
    persistence.set_value("app_data", &*ui_state);
    persistence.maybe_autosave();
}
