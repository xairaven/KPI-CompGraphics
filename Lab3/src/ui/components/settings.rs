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

        ui.collapsing("Grid Settings", |ui| {
            ui.checkbox(&mut context.grid.is_enabled, "Enable Grid");
            ui.checkbox(&mut canvas.screen_params.is_dragging_offset_enabled, "Enable Drag & Offset");

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
                    canvas.screen_params.grid_unit_length = 1.0;
                    canvas.screen_params.offset = (0.0, 0.0);
                    canvas.screen_params.is_dragging_offset_enabled = true;
                }
            });
        });

        ui.add_space(10.0);

        ui.collapsing("Bezier Skeleton Settings", |ui| {
            ui.checkbox(&mut context.model.are_tooltips_enabled, "Tooltips")
                .on_hover_text("If you hover the cursor over a control or defining point with 'Tooltips' mode enabled, you will be able to see its number and coordinates.");

            ui.add_space(10.0);

            Grid::new("PointStrokes").num_columns(2).show(ui, |ui| {
                ui.label("Skeleton Stroke:");
                ui.add(&mut context.model.skeleton_stroke);

                ui.end_row();

                ui.label("Outline:");
                ui.add(&mut context.model.outline);

                ui.end_row();

                ui.label("Fill - Control Point:");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut context.model.fill_control,
                    egui::color_picker::Alpha::Opaque,
                );

                ui.end_row();

                ui.label("Fill - Defining Point:");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut context.model.fill_defining,
                    egui::color_picker::Alpha::Opaque,
                );

                ui.end_row();

                ui.label("Radius:");
                ui.add(
                    DragValue::new(&mut context.model.radius)
                        .speed(0.1)
                        .range(0.1..=2.0),
                );
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.button("Reset Settings").clicked() {
                    context.model = Default::default();
                }
            });
        });

        ui.add_space(10.0);

        ui.collapsing("Model Settings", |ui| {
            Grid::new("PointStrokes").num_columns(2).show(ui, |ui| {
                ui.label("Stroke:");
                ui.add(&mut context.model.model_stroke);

                ui.end_row();

                ui.label("Bezier step:");
                ui.add(DragValue::new(&mut context.model.bezier_step)
                           .speed(0.01)
                           .range(0.01..=0.5));
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.button("Reset Settings").clicked() {
                    context.model = Default::default();
                }
            });
        });

        ui.add_space(10.0);

        ui.checkbox(&mut context.model.is_skeleton_enabled, "Display Skeleton");

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Animation").strong());
        });
        ui.add_space(5.0);
        ui.group(|ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Status: ");
                if context.animation_settings.is_running {
                    ui.label(RichText::new("Running!").color(colors::LIME));
                } else {
                    ui.label(RichText::new("Stopped.").color(colors::RED));
                };

                ui.vertical_centered(|ui| {
                    if ui.button("Start / Stop").clicked() {
                        context.animation_settings.is_running =
                            !context.animation_settings.is_running;
                    }
                });
            });
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Euclidean Transformations").strong());
        });

        ui.add_space(5.0);

        ui.checkbox(&mut context.euclidean_offset.is_enabled,
                    RichText::new("Enable Offset").color(colors::BLUE));

        ui.add_space(5.0);

        Grid::new("OffsetGrid").num_columns(2).show(ui, |ui| {
            ui.label("Offset X: ");
            ui.add(
                DragValue::new(&mut context.euclidean_offset.x).speed(0.1)
            );

            ui.end_row();

            ui.label("Offset Y: ");
            ui.add(
                DragValue::new(&mut context.euclidean_offset.y).speed(0.1)
            );
        });

        ui.add_space(5.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset").clicked() {
                context.euclidean_offset = Default::default();
            }
        });

        ui.add_space(5.0);
        ui.separator();
        ui.add_space(5.0);

        ui.checkbox(&mut context.euclidean_rotation.is_enabled,
                    RichText::new("Enable Rotation").color(colors::YELLOW));

        ui.add_space(5.0);

        Grid::new("RotationGrid").num_columns(2).show(ui, |ui| {
            ui.label("Rotation X: ");
            ui.add(
                DragValue::new(&mut context.euclidean_rotation.x).speed(0.1)
            );

            ui.end_row();

            ui.label("Rotation Y: ");
            ui.add(
                DragValue::new(&mut context.euclidean_rotation.y).speed(0.1)
            );

            ui.end_row();

            ui.label("Rotation Angle: ");
            ui.add(
                DragValue::new(&mut context.euclidean_rotation.angle)
                    .speed(1)
                    .range(-360..=360),
            );
        });

        ui.add_space(5.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset").clicked() {
                context.euclidean_rotation = Default::default();
            }
        });
    });
}

fn reset_to_defaults(context: &mut Context, canvas: &mut Canvas) {
    canvas.screen_params = Default::default();
    context.grid = Default::default();
    context.model = Default::default();
    context.animation_settings = Default::default();
    context.euclidean_offset = Default::default();
    context.euclidean_rotation = Default::default();
}
