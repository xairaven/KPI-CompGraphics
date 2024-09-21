use crate::ui::app::Application;
use eframe::{NativeOptions, Theme};

pub const LAB_TITLE: &str = "Lab 1";

pub fn start() -> eframe::Result {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(format!("Computer Graphics: {LAB_TITLE}"))
            .with_inner_size([1200.0, 700.0])
            .with_min_inner_size([900.0, 550.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../../assets/icon-256.png")[..],
                )
                .expect("Failed to load icon"),
            ),
        default_theme: Theme::Light,
        follow_system_theme: false,
        ..Default::default()
    };

    eframe::run_native(
        LAB_TITLE,
        native_options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )
}
