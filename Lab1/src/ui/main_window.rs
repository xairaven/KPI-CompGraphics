use crate::ui::app::Application;
use egui::{Color32, Frame};

pub const ELEMENT_NAME_SETTINGS_PANEL: &str = "Settings_Panel";

pub fn show(app: &mut Application, ui: &mut egui::Ui, ctx: &egui::Context) {
    egui::SidePanel::right(ELEMENT_NAME_SETTINGS_PANEL)
        .resizable(false)
        .default_width(250.0)
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

    Frame::canvas(ui.style())
        .fill(Color32::from_rgb(255, 255, 255))
        .show(ui, |ui| {});
}
