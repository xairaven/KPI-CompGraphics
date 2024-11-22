use crate::context::Context;
use crate::graphics::screen::{MAX_PX_PER_CM, MIN_PX_PER_CM};
use crate::ui::components::canvas::Canvas;
use crate::ui::styles::colors;
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

        ui.collapsing("Axis Settings", |ui| {
            Grid::new("AxisSettings").num_columns(2).show(ui, |ui| {
                ui.checkbox(&mut context.axes.are_enabled, "Enable Axes");
                ui.end_row();

                ui.label("Unit Length:");
                ui.add(
                    DragValue::new(&mut canvas.screen_params.unit_length)
                        .speed(1)
                        .range(1.0..=10.0),
                );
                ui.end_row();

                ui.label("Axes Length:");
                ui.add(
                    DragValue::new(&mut context.axes.axis_length)
                        .speed(1)
                        .range(1..=30),
                );
                ui.end_row();
            });

            ui.add_space(10.0);

            ui.label(RichText::new("Rotation").color(colors::WHITE));
            ui.add_space(5.0);
            Grid::new("AxisRotation").num_columns(2).show(ui, |ui| {
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

            ui.add_space(10.0);

            ui.label(RichText::new("Stroke").color(colors::WHITE));
            ui.add_space(5.0);
            Grid::new("AxisStrokes").num_columns(2).show(ui, |ui| {
                ui.label("Axis X:");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut context.axes.color_x,
                    egui::color_picker::Alpha::Opaque,
                );
                ui.end_row();

                ui.label("Axis Y:");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut context.axes.color_y,
                    egui::color_picker::Alpha::Opaque,
                );
                ui.end_row();

                ui.label("Axis Z:");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut context.axes.color_z,
                    egui::color_picker::Alpha::Opaque,
                );
                ui.end_row();
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.button("Reset Settings").clicked() {
                    context.axes = Default::default();
                    canvas.screen_params.unit_length = 1.0;
                }
            });
        });

        ui.add_space(10.0);

        ui.collapsing("Model Settings", |ui| {
            Grid::new("ModelSettings").num_columns(2).show(ui, |ui| {
                ui.label("Radius:");
                ui.add(
                    DragValue::new(&mut context.model.radius)
                        .speed(0.1)
                        .range(0.1..=20.0),
                );
                ui.end_row();

                ui.label("Thickness:");
                ui.add(
                    DragValue::new(&mut context.model.thickness)
                        .speed(0.1)
                        .range(0.1..=20.0),
                );
                ui.end_row();
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.button("Reset Settings").clicked() {
                    context.model = Default::default();
                }
            });
        });
    });
}

fn reset_to_defaults(context: &mut Context, canvas: &mut Canvas) {
    context.axes = Default::default();
    context.model = Default::default();

    canvas.screen_params = Default::default();

    context.orthographic = Default::default();
    context.trimetric = Default::default();
}
