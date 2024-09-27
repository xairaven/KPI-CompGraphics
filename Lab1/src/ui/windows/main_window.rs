use crate::ui::app_model::AppModel;
use crate::ui::components;
use egui::{CentralPanel, SidePanel};

pub const SETTINGS_PANEL_WIDTH: f32 = 250.0;

pub fn show(app: &mut AppModel, ui: &mut egui::Ui, _ctx: &egui::Context) {
    SidePanel::right(components::NAME_SETTINGS_PANEL)
        .resizable(false)
        .default_width(SETTINGS_PANEL_WIDTH)
        .show_separator_line(true)
        .show_inside(ui, |ui| {
            components::settings::show_panel(app, ui);
        });

    CentralPanel::default().show_inside(ui, |ui| {
        app.canvas.update(ui);
    });
}
