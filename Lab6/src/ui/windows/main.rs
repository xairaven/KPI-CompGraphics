use crate::ui::app::App;
use crate::ui::components;
use egui::{CentralPanel, SidePanel};

pub fn show(app: &mut App, ui: &mut egui::Ui, _ctx: &egui::Context) {
    SidePanel::right("CANVAS_PANEL")
        .resizable(false)
        .default_width(components::settings::SETTINGS_PANEL_WIDTH)
        .show_separator_line(true)
        .show_inside(ui, |ui| {
            app.settings
                .show_panel(&mut app.context, &mut app.canvas, ui);
        });

    CentralPanel::default().show_inside(ui, |ui| {
        app.canvas.show_content(&mut app.context, ui);
    });
}
