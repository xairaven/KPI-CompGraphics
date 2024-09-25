use crate::ui::app_model::AppModel;
use egui::DragValue;

pub fn show_panel(app: &mut AppModel, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Settings");
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Pixels per Centimeter:");
            ui.add(
                DragValue::new(&mut app.canvas.px_per_cm)
                    .speed(1)
                    .range(1..=100),
            );
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            if ui.button("Set Default Figure").clicked() {
                // TODO: Set default figure
            }
        });

        ui.add_space(10.0);
        ui.separator();

        ui.label("Transformations:");
    });
}
