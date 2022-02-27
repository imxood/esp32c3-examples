use epi::egui::{FontData, FontDefinitions, FontFamily};

use crate::resource::defines::fonts::FONT_CHINESE;

pub fn chinese() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let chinese_data = FontData::from_static(FONT_CHINESE);
    fonts.font_data.insert("chinese".into(), chinese_data);

    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "chinese".into());

    tracing::info!("FontFamily::Proportional:");
    for font in fonts
        .families
        .get(&FontFamily::Proportional)
        .unwrap()
        .iter()
    {
        tracing::info!("\t{}", font);
    }

    fonts
}
