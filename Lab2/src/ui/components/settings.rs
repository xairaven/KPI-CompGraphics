use crate::context::Context;
use crate::models::model;
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
                context.grid = Default::default();
                context.model = Default::default();
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
                .striped(true)
                .min_col_width(25.0)
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

                    ui.end_row();
                })
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
                .min_col_width(50.0)
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
                });
        });

        ui.add_space(10.0);
    });
}
