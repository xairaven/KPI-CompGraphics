use crate::context::Context;
use crate::graphics::screen::{MAX_PX_PER_CM, MIN_PX_PER_CM};
use crate::projections::orthographic::OrthographicMode;
use crate::ui::components::canvas::Canvas;
use crate::ui::components::message_box::MessageBox;
use crate::ui::styles::colors;
use egui::{DragValue, Grid, RichText};

pub const SETTINGS_PANEL_WIDTH: f32 = 250.0;

#[derive(Default)]
pub struct Settings {
    error_box: Option<MessageBox>,
}

impl Settings {
    pub fn show_panel(&mut self, context: &mut Context, canvas: &mut Canvas, ui: &mut egui::Ui) {
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
                    self.reset_to_defaults(context, canvas);
                }
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

            ui.add_space(10.0);

            ui.collapsing("Surface Settings", |ui| {
                Grid::new("SurfaceSettings").num_columns(2).show(ui, |ui| {
                    ui.label("Radius: ");
                    ui.add(
                        DragValue::new(&mut context.surface.display_radius)
                            .speed(1)
                            .range(1.0..=f32::MAX)
                            .suffix(" cm"),
                    );
                    ui.end_row();

                    ui.label("Height: ");
                    ui.add(
                        DragValue::new(&mut context.surface.display_height)
                            .speed(1)
                            .range(1.0..=f32::MAX)
                            .suffix(" cm"),
                    );
                    ui.end_row();

                    ui.label("Mesh Density: ");
                    ui.add(
                        DragValue::new(&mut context.surface.display_mesh)
                            .speed(1)
                            .range(1.0..=f32::MAX)
                            .suffix(" cm"),
                    );
                    ui.end_row();
                });
            });

            ui.add_space(10.0);

            ui.collapsing("Texture Settings", |ui| {
                Grid::new("TextureStatus").num_columns(2).show(ui, |ui| {
                    ui.checkbox(&mut context.texture.is_enabled, "Enable");
                    ui.end_row();

                    ui.label("Status: ");
                    if context.texture.is_loaded() {
                        ui.label(RichText::new("Loaded!").color(colors::LIME));
                    } else {
                        ui.label(RichText::new("Not Loaded.").color(colors::RED));
                    }
                    ui.end_row();
                });
                context.texture.ensure_texture_loaded();

                ui.add_space(5.0);

                ui.vertical_centered_justified(|ui| {
                    if ui
                        .add_enabled(
                            !context.texture.is_loaded(),
                            egui::Button::new("Load Texture"),
                        )
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let loading_result = context.texture.load_texture(path);

                            if let Err(e) = loading_result {
                                let mut error_message =
                                    format!("Error occured while loading texture:\n{}\n", e);
                                if let Some(additional_info) = e.get_additional_info() {
                                    error_message.push_str(
                                        format!("\nAdditional Info:\n{}", additional_info).as_str(),
                                    );
                                }

                                self.error_box =
                                    Some(MessageBox::new("Error!".to_string(), error_message));
                            }
                        }
                    }
                });

                ui.vertical_centered_justified(|ui| {
                    if ui
                        .add_enabled(
                            context.texture.is_loaded(),
                            egui::Button::new("Unload Texture"),
                        )
                        .clicked()
                    {
                        context.texture.unload_texture();
                    }
                });

                ui.add_space(5.0);

                // Grid::new("TextureSettings")
                //     .num_columns(2)
                //     .show(ui, |ui| {});
            });
        });

        self.show_errors_if_some(ui);
    }

    fn show_errors_if_some(&mut self, ui: &egui::Ui) {
        if let Some(error) = &mut self.error_box {
            error.show(ui);

            if error.is_closed() {
                self.error_box = None;
            }
        }
    }

    fn reset_to_defaults(&self, context: &mut Context, canvas: &mut Canvas) {
        canvas.screen_params = Default::default();

        context.axes = Default::default();

        context.surface = Default::default();
        context.texture = Default::default();

        context.trimetric = Default::default();
        context.orthographic = Default::default();

        context.animation = Default::default();

        context.offset = Default::default();
        context.rotation = Default::default();
    }
}
