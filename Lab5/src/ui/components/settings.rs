use crate::context::Context;
use crate::graphics::screen::{MAX_PX_PER_CM, MIN_PX_PER_CM};
use crate::ui::components::canvas::Canvas;
use egui::{DragValue, Grid};

pub const SETTINGS_PANEL_WIDTH: f32 = 250.0;
const GRID_COLUMN_MINIMAL_WIDTH: f32 = 50.0;

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

        ui.collapsing("Axis Rotation Settings", |ui| {
            Grid::new("AxisRotationSettings")
                .num_columns(2)
                .min_col_width(GRID_COLUMN_MINIMAL_WIDTH)
                .show(ui, |ui| {
                    ui.label("OX:");
                    ui.add(
                        DragValue::new(&mut context.trimetric.angle_deg_x)
                            .speed(1)
                            .range(-360..=360),
                    );

                    ui.end_row();

                    ui.label("OY:");
                    ui.add(
                        DragValue::new(&mut context.trimetric.angle_deg_y)
                            .speed(1)
                            .range(-360..=360),
                    );

                    ui.end_row();
                });
        });

        ui.add_space(10.0);

        ui.collapsing("Model Settings", |ui| {
            Grid::new("ModelSettings")
                .num_columns(2)
                .min_col_width(GRID_COLUMN_MINIMAL_WIDTH)
                .show(ui, |ui| {
                    ui.label("Radius:");
                    ui.add(
                        DragValue::new(&mut context.model.radius)
                            .speed(0.1)
                            .range(0.1..=20.0),
                    );

                    ui.end_row();

                    ui.label("Height:");
                    ui.add(
                        DragValue::new(&mut context.model.height)
                            .speed(0.1)
                            .range(0.1..=20.0),
                    );
                });
        });
    });
}

fn reset_to_defaults(context: &mut Context, canvas: &mut Canvas) {
    canvas.screen_params = Default::default();
    context.grid = Default::default();
    context.model = Default::default();
    context.trimetric = Default::default();
    context.orthographic = Default::default();
}
