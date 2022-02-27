use crate::resource::{defines::icons::ICON_LOGO, error::Result};

use egui_extras::RetainedImage;
use winit::window::Icon;

pub struct Icons {
    pub maximize: RetainedImage,
    pub minus: RetainedImage,
    pub x: RetainedImage,
    pub menu: RetainedImage,
    pub list: RetainedImage,
    pub settings: RetainedImage,
}

impl Default for Icons {
    fn default() -> Self {
        Self {
            maximize: Self::maximize(),
            minus: Self::minus(),
            x: Self::x(),
            menu: Self::menu(),
            list: Self::list(),
            settings: Self::settings(),
        }
    }
}

impl Icons {
    pub fn logo() -> Result<Icon> {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::load_from_memory(ICON_LOGO)?.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height)?;
        Ok(icon)
    }

    pub fn maximize() -> RetainedImage {
        let icon = include_bytes!("icons/maximize.svg");
        RetainedImage::from_svg_bytes("icon_maximize", icon).unwrap()
    }

    pub fn minus() -> RetainedImage {
        let icon = include_bytes!("icons/minus.svg");
        RetainedImage::from_svg_bytes("icon_minus", icon).unwrap()
    }

    pub fn x() -> RetainedImage {
        let icon = include_bytes!("icons/x.svg");
        RetainedImage::from_svg_bytes("icon_x", icon).unwrap()
    }

    pub fn menu() -> RetainedImage {
        let icon = include_bytes!("icons/menu.svg");
        RetainedImage::from_svg_bytes("icon_menu", icon).unwrap()
    }

    pub fn list() -> RetainedImage {
        let icon = include_bytes!("icons/list.svg");
        RetainedImage::from_svg_bytes("icon_list", icon).unwrap()
    }

    pub fn settings() -> RetainedImage {
        let icon = include_bytes!("icons/settings.svg");
        RetainedImage::from_svg_bytes("icon_settings", icon).unwrap()
    }
}
