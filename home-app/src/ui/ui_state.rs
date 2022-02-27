use serde::{Deserialize, Serialize};

use crate::resource::theme::Theme;

#[derive(Serialize, Deserialize)]
pub struct UiState {
    pub theme: Theme,
    pub maximized: bool,
    pub scale_factor: f64,
    pub fps: f64,
    pub fatal_error: Option<String>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            maximized: false,
            scale_factor: 1.25,
            theme: Theme::default(),
            fps: 0.0,
            fatal_error: None,
        }
    }
}
