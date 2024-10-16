use crate::context::Context;
use crate::graphics::screen::{MAX_PX_PER_CM, MIN_PX_PER_CM};
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
                    .range(MIN_PX_PER_CM..=MAX_PX_PER_CM),
            );
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            if ui.button("Reset to Default Settings").clicked() {
                reset_to_defaults(context, canvas);
            }
        });
    });
}

fn reset_to_defaults(context: &mut Context, canvas: &mut Canvas) {
    canvas.screen_params = Default::default();
    context.grid = Default::default();
}
