use crate::context::Context;
use crate::ui::components::canvas::Canvas;
use egui::DragValue;

pub const SETTINGS_PANEL_WIDTH: f32 = 250.0;

pub fn show_panel(context: &mut Context, canvas: &mut Canvas, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Settings");
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Pixels per Centimeter:");
            ui.add(
                DragValue::new(&mut canvas.screen_params.px_per_cm)
                    .speed(1)
                    .range(1..=100),
            );
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            if ui.button("Reset to Default Settings").clicked() {
                context.model = Default::default();
                canvas.screen_params = Default::default();
            }
        });

        ui.add_space(10.0);
    });
}
