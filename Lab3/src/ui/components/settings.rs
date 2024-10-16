use crate::context::Context;
use crate::graphics::screen::{MAX_PX_PER_CM, MIN_PX_PER_CM};
use crate::ui::components::canvas::Canvas;
use egui::{DragValue, Grid, RichText};

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

        ui.add_space(10.0);

        ui.collapsing("Grid Settings", |ui| {
            ui.checkbox(&mut context.grid.is_enabled, "Enable Grid");

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label("Unit Length:");
                ui.add(
                    DragValue::new(&mut canvas.screen_params.grid_unit_length)
                        .speed(1)
                        .range(1.0..=10.0),
                );
            });

            ui.add_space(10.0);

            ui.label(RichText::new("Strokes:").strong());
            Grid::new("GridStrokes").num_columns(2).show(ui, |ui| {
                ui.label("Axis X:");
                ui.add(&mut context.grid.axis_x_stroke);

                ui.end_row();

                ui.label("Axis Y:");
                ui.add(&mut context.grid.axis_y_stroke);

                ui.end_row();

                ui.label("Grid:");
                ui.add(&mut context.grid.grid_stroke);
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.button("Reset Settings").clicked() {
                    context.grid = Default::default();
                }
            })
        });

        ui.add_space(10.0);
    });
}

fn reset_to_defaults(context: &mut Context, canvas: &mut Canvas) {
    canvas.screen_params = Default::default();
    context.grid = Default::default();
}
