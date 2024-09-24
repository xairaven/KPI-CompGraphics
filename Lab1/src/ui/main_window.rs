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
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Settings");
                });

                ui.vertical_centered(|ui| {
                    if ui.button("Set Default Figure").clicked() {
                        // TODO: Set default figure
                    }
                });

                ui.label(egui::RichText::new("SomeText"));
            });
        });

    CentralPanel::default().show_inside(ui, |ui| {
        app.canvas.show(ui);
    });
}
