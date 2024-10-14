use crate::context::Context;
use crate::models::{grid, model};
use crate::operations::curve_point;
use crate::operations::curve_point::Direction;
use crate::ui::components::canvas::Canvas;
use crate::ui::styles::colors;
use egui::style::HandleShape;
use egui::{DragValue, Grid, RichText, Slider};

pub const SETTINGS_PANEL_WIDTH: f32 = 250.0;
pub const GRID_SIZE: f32 = 200.0;
pub const MAX_SCROLL_OFFSET: f32 = GRID_SIZE;

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
                    .range(10..=150),
            );
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Scroll");

                Grid::new("ScrollGrid")
                    .min_col_width(100.0)
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Horizontal:");
                        ui.add(
                            Slider::new(
                                &mut canvas.screen_params.offset_x,
                                -MAX_SCROLL_OFFSET..=MAX_SCROLL_OFFSET,
                            )
                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 }),
                        );
                        ui.end_row();

                        ui.label("Vertical:");
                        ui.add(
                            Slider::new(
                                &mut canvas.screen_params.offset_y,
                                -MAX_SCROLL_OFFSET..=MAX_SCROLL_OFFSET,
                            )
                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 }),
                        );
                        ui.end_row();
                    });
            });
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            if ui.button("Reset to Default Settings").clicked() {
                canvas.screen_params = Default::default();
                context.animation_settings = Default::default();
                context.curve_point = Default::default();
                context.curve_props = Default::default();
                context.grid = Default::default();
                context.model = Default::default();
                context.offset = Default::default();
                context.rotation = Default::default();
            }
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Parameters").strong());
        });
        ui.add_space(5.0);

        ui.group(|ui| {
            Grid::new("ParametersGrid")
                .num_columns(6)
                .min_col_width(35.0)
                .show(ui, |ui| {
                    ui.label("A:");
                    ui.add(
                        DragValue::new(&mut context.model.a)
                            .speed(0.05)
                            .range(-model::PARAMETERS_MAX..=model::PARAMETERS_MAX),
                    );

                    ui.label("B:");
                    ui.add(
                        DragValue::new(&mut context.model.b)
                            .speed(0.05)
                            .range(-model::PARAMETERS_MAX..=model::PARAMETERS_MAX),
                    );

                    ui.label("C:");
                    ui.add(
                        DragValue::new(&mut context.model.c)
                            .speed(0.05)
                            .range(-model::PARAMETERS_MAX..=model::PARAMETERS_MAX),
                    );
                });
        });

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

                        context.curve_point = Default::default();
                    }
                });
            });
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Curve Point").color(colors::PINK));
        });
        ui.add_space(5.0);
        ui.group(|ui| {
            Grid::new("CurvePointGrid")
                .num_columns(3)
                .min_col_width(40.0)
                .spacing([40.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Status: ");

                    if context.curve_point.is_visible {
                        ui.label(RichText::new("Visible").color(colors::LIME));
                    } else {
                        ui.label(RichText::new("Hidden").color(colors::RED));
                    };

                    if context.curve_point.is_running {
                        ui.label(RichText::new("Running").color(colors::LIME));
                    } else {
                        ui.label(RichText::new("Stopped").color(colors::RED));
                    };

                    ui.end_row();

                    if ui.button("Show / Hide").clicked() {
                        context.curve_point.is_visible = !context.curve_point.is_visible;

                        if !context.curve_point.is_visible {
                            context.curve_point = Default::default()
                        } else {
                            context.animation_settings = Default::default();
                        }
                    }

                    if ui.button("⏪").clicked() && context.curve_point.is_visible {
                        context.curve_point.is_running = !context.curve_point.is_running;
                        context.curve_point.direction = Direction::Left;
                    }
                    if ui.button("⏩").clicked() && context.curve_point.is_visible {
                        context.curve_point.is_running = !context.curve_point.is_running;
                        context.curve_point.direction = Direction::Right;
                    }

                    ui.end_row();

                    ui.label("Coordinates: ");
                    ui.label(format!("X: {:.2}", context.curve_point.dot.center.x));
                    ui.label(format!("Y: {:.2}", context.curve_point.dot.center.y));

                    ui.end_row();

                    ui.label("Curvature Radius: ");
                    ui.label(format!("{:.2} cm.", context.curve_point.curvature_radius));

                    ui.end_row();

                    ui.label("Speed: ");
                    ui.add(
                        DragValue::new(&mut context.curve_point.speed)
                            .speed(1)
                            .range(curve_point::MIN_SPEED..=curve_point::MAX_SPEED),
                    );
                });
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Properties").strong());
        });
        ui.add_space(5.0);
        ui.label(format!(
            "Curve Length: {:.2} cm.",
            context.curve_props.length
        ));
        ui.checkbox(
            &mut context.curve_props.is_tangent_enabled,
            RichText::new("Tangent").color(colors::BLUE),
        );
        if !context.curve_point.is_visible {
            context.curve_props.is_tangent_enabled = false;
        }
        ui.checkbox(
            &mut context.curve_props.is_normal_enabled,
            RichText::new("Normal").color(colors::AQUA),
        );
        if !context.curve_point.is_visible {
            context.curve_props.is_normal_enabled = false;
        }
        ui.checkbox(
            &mut context.curve_props.is_inflection_enabled,
            RichText::new("Inflection points").color(colors::ORANGE),
        );

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Euclidean Transformations").strong());
        });

        ui.add_space(5.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Offset");
            });

            ui.add_space(5.0);

            Grid::new("OffsetGrid")
                .num_columns(5)
                .min_col_width(30.0)
                .show(ui, |ui| {
                    ui.label("X: ");
                    ui.add(
                        DragValue::new(&mut context.offset.x)
                            .speed(1)
                            .range(-grid::DEFAULT_TICKS..=grid::DEFAULT_TICKS),
                    );

                    ui.label("Y: ");
                    ui.add(
                        DragValue::new(&mut context.offset.y)
                            .speed(1)
                            .range(-grid::DEFAULT_TICKS..=grid::DEFAULT_TICKS),
                    );

                    if ui.button("\t\tReset\t\t").clicked() {
                        context.offset = Default::default();
                    }
                });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Rotation").color(colors::GREEN));
            });

            ui.add_space(5.0);

            Grid::new("RotationGrid")
                .num_columns(6)
                .min_col_width(35.0)
                .show(ui, |ui| {
                    ui.label("X: ");
                    ui.add(
                        DragValue::new(&mut context.rotation.x)
                            .speed(1)
                            .range(-grid::DEFAULT_TICKS..=grid::DEFAULT_TICKS),
                    );

                    ui.label("Y: ");
                    ui.add(
                        DragValue::new(&mut context.rotation.y)
                            .speed(1)
                            .range(-grid::DEFAULT_TICKS..=grid::DEFAULT_TICKS),
                    );

                    ui.label("Angle: ");
                    ui.add(
                        DragValue::new(&mut context.rotation.angle)
                            .speed(1)
                            .range(-360..=360),
                    );
                });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.button("\t\tReset\t\t").clicked() {
                    context.rotation = Default::default();
                }
            });
        });
        ui.add_space(10.0);
    });
}
