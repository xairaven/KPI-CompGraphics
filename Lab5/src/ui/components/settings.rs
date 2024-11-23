use crate::context::Context;
use crate::graphics::screen::{MAX_PX_PER_CM, MIN_PX_PER_CM};
use crate::projections::orthographic::OrthographicMode;
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
                    .range(MIN_PX_PER_CM..=MAX_PX_PER_CM)
                    .suffix(" px"),
            );
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            if ui.button("Reset to Default Settings").clicked() {
                reset_to_defaults(context, canvas);
            }
        });

        ui.add_space(10.0);

        ui.collapsing("Drag & Offset", |ui| {
            Grid::new("DragAndOffsetSettings")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.checkbox(
                        &mut canvas.screen_params.is_dragging_offset_enabled,
                        "Enable",
                    );
                    if ui.button("Reset Settings").clicked() {
                        canvas.screen_params.offset = Default::default();
                        canvas.screen_params.is_dragging_offset_enabled = true;
                    }
                    ui.end_row();

                    ui.label("X:");
                    ui.label(format!("{:.2}", canvas.screen_params.offset.0));
                    ui.end_row();
                    ui.label("Y:");
                    ui.label(format!("{:.2}", -canvas.screen_params.offset.1));
                    ui.end_row();
                });
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
                        .range(1.0..=10.0)
                        .suffix(" cm"),
                );
                ui.end_row();

                ui.label("Axes Length:");
                ui.add(
                    DragValue::new(&mut context.axes.axis_length)
                        .speed(1)
                        .range(1..=u32::MAX)
                        .suffix(" cm"),
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
                        .range(-360..=360)
                        .suffix("°"),
                );
                ui.end_row();

                ui.label("OY:");
                ui.add(
                    DragValue::new(&mut context.trimetric.angle_deg_y)
                        .speed(1)
                        .range(-360..=360)
                        .suffix("°"),
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
            ui.vertical_centered(|ui| {
                if ui.button("Reset Rotation").clicked() {
                    context.trimetric = Default::default();
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
                        .range(0.1..=f32::MAX)
                        .suffix(" cm"),
                );
                ui.end_row();

                ui.label("Thickness:");
                ui.add(
                    DragValue::new(&mut context.model.thickness)
                        .speed(0.1)
                        .range(0.1..=f32::MAX)
                        .suffix(" cm"),
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

        ui.add_space(10.0);

        ui.collapsing("Animation", |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label("Status: ");
                if context.animation.is_running {
                    ui.label(RichText::new("Running!").color(colors::LIME));
                } else {
                    ui.label(RichText::new("Stopped.").color(colors::RED));
                }

                ui.vertical_centered(|ui| {
                    if ui.button("Start / Stop").clicked() {
                        context.animation.checkout_status();
                    }
                });
            });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Animation Parameters").color(colors::WHITE));
            });

            ui.add_space(5.0);

            ui.checkbox(&mut context.animation.use_radius, "Use Radius");
            if context.animation.use_radius {
                Grid::new("AnimationRadiusParameters")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Minimum: ");
                        ui.add(
                            DragValue::new(&mut context.animation.min_radius)
                                .speed(0.1)
                                .range(0.1..=f32::MAX)
                                .suffix(" cm"),
                        );
                        ui.end_row();

                        ui.label("Maximum: ");
                        ui.add(
                            DragValue::new(&mut context.animation.max_radius)
                                .speed(0.1)
                                .range(0.1..=f32::MAX)
                                .suffix(" cm"),
                        );
                        ui.end_row();

                        ui.label("Step: ");
                        ui.add(
                            DragValue::new(&mut context.animation.step_radius)
                                .speed(0.01)
                                .range(0.01..=f32::MAX)
                                .suffix(" cm"),
                        );
                        ui.end_row();
                    });
            }

            ui.add_space(5.0);

            ui.checkbox(&mut context.animation.use_rotation, "Use Rotation");
            if context.animation.use_rotation {
                Grid::new("AnimationRotationParameters")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Step, OX: ");
                        ui.add(
                            DragValue::new(&mut context.animation.step_x)
                                .speed(1)
                                .range(-360..=360)
                                .suffix("°"),
                        );
                        ui.end_row();

                        ui.label("Step, OY: ");
                        ui.add(
                            DragValue::new(&mut context.animation.step_y)
                                .speed(1)
                                .range(-360..=360)
                                .suffix("°"),
                        );
                        ui.end_row();

                        ui.label("Step, OZ: ");
                        ui.add(
                            DragValue::new(&mut context.animation.step_z)
                                .speed(1)
                                .range(-360..=360)
                                .suffix("°"),
                        );
                        ui.end_row();
                    });
            }
        });

        ui.add_space(10.0);

        ui.collapsing("Euclidean Offset", |ui| {
            Grid::new("OffsetSettings").num_columns(2).show(ui, |ui| {
                ui.label("Offset X: ");
                ui.add(
                    DragValue::new(&mut context.offset.x)
                        .speed(0.1)
                        .suffix(" cm"),
                );
                ui.end_row();

                ui.label("Offset Y: ");
                ui.add(
                    DragValue::new(&mut context.offset.y)
                        .speed(0.1)
                        .suffix(" cm"),
                );
                ui.end_row();

                ui.label("Offset Z: ");
                ui.add(
                    DragValue::new(&mut context.offset.z)
                        .speed(0.1)
                        .suffix(" cm"),
                );
                ui.end_row();
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui.button("Apply").clicked() {
                    context.offset.is_applied = true;
                }
            });

            ui.vertical_centered_justified(|ui| {
                if ui.button("Clear Fields").clicked() {
                    context.offset.x = 0.0;
                    context.offset.y = 0.0;
                    context.offset.z = 0.0;
                }
            });

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset Position").clicked() {
                    context.offset.reset_position();
                }
            });
        });

        ui.add_space(10.0);

        ui.collapsing("Euclidean Rotation", |ui| {
            Grid::new("RotationSettings").num_columns(2).show(ui, |ui| {
                ui.label("OX Angle: ");
                ui.add(
                    DragValue::new(&mut context.rotation.angle_deg_x)
                        .speed(1)
                        .range(-360..=360)
                        .suffix("°"),
                );
                ui.end_row();

                ui.label("OY Angle: ");
                ui.add(
                    DragValue::new(&mut context.rotation.angle_deg_y)
                        .speed(1)
                        .range(-360..=360)
                        .suffix("°"),
                );
                ui.end_row();

                ui.label("OZ Angle: ");
                ui.add(
                    DragValue::new(&mut context.rotation.angle_deg_z)
                        .speed(1)
                        .range(-360..=360)
                        .suffix("°"),
                );
                ui.end_row();
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui.button("Apply").clicked() {
                    context.rotation.is_applied = true;
                }
            });

            ui.vertical_centered_justified(|ui| {
                if ui.button("Clear Fields").clicked() {
                    context.rotation.angle_deg_x = 0.0;
                    context.rotation.angle_deg_y = 0.0;
                    context.rotation.angle_deg_z = 0.0;
                }
            });

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset Position").clicked() {
                    context.rotation.reset_position();
                }
            });
        });

        ui.add_space(10.0);

        ui.collapsing("Orthographic Projections", |ui| {
            ui.checkbox(&mut context.orthographic.is_enabled, "Enable");

            ui.add_space(5.0);

            Grid::new("SelectOrthographicProjection")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.radio_value(
                        &mut context.orthographic.mode,
                        OrthographicMode::XOnZero,
                        "X = 0",
                    );
                    ui.radio_value(
                        &mut context.orthographic.mode,
                        OrthographicMode::XOnP,
                        "X = P",
                    );
                    ui.end_row();

                    ui.radio_value(
                        &mut context.orthographic.mode,
                        OrthographicMode::YOnZero,
                        "Y = 0",
                    );
                    ui.radio_value(
                        &mut context.orthographic.mode,
                        OrthographicMode::YOnP,
                        "Y = P",
                    );
                    ui.end_row();

                    ui.radio_value(
                        &mut context.orthographic.mode,
                        OrthographicMode::ZOnZero,
                        "Z = 0",
                    );
                    ui.radio_value(
                        &mut context.orthographic.mode,
                        OrthographicMode::ZOnP,
                        "Z = P",
                    );
                    ui.end_row();
                });

            match &context.orthographic.mode {
                OrthographicMode::XOnP | OrthographicMode::YOnP | OrthographicMode::ZOnP => {
                    ui.add_space(5.0);

                    ui.horizontal_wrapped(|ui| {
                        ui.label("P: ");
                        ui.add(
                            DragValue::new(&mut context.orthographic.p)
                                .speed(0.1)
                                .suffix(" cm"),
                        );
                    });
                },
                _ => {},
            }
        });
    });
}

fn reset_to_defaults(context: &mut Context, canvas: &mut Canvas) {
    canvas.screen_params = Default::default();

    context.axes = Default::default();
    context.model = Default::default();

    context.trimetric = Default::default();
    context.orthographic = Default::default();

    context.animation = Default::default();

    context.offset = Default::default();
    context.rotation = Default::default();
}
