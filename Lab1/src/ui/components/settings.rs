use crate::ui::app_model::AppModel;

pub fn show_panel(app: &mut AppModel, ui: &mut egui::Ui) {
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

        app.canvas.px_per_cm = 35.0;
        app.grid.ticks = 10;
    });
}
