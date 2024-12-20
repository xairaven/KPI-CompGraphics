use crate::context::Context;
use crate::ui::components::canvas::Canvas;
use crate::ui::styles::colors;
use crate::ui::windows::ifs_settings::IfsSettingsWindow;
use crate::ui::windows::traits::window_ops::WindowOps;
use egui::{Button, DragValue, Grid, RichText};

pub struct Settings {
    pub panel_width: f32,

    ifs_settings: Option<Box<dyn WindowOps>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            panel_width: 250.0,
            ifs_settings: None,
        }
    }
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
                        .range(5.0..=100.0)
                        .suffix(" cm."),
                );
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset Offset").clicked() {
                    canvas.screen_params.offset = Default::default();
                }
            });

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset to Default Settings").clicked() {
                    self.reset_to_defaults(context, canvas);
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Fractal Settings").strong());
            });
            ui.add_space(10.0);
            Grid::new("Fractal Settings").num_columns(2).show(ui, |ui| {
                ui.label("Status: ");
                if context.fractal_view.initialized {
                    ui.label(RichText::new("Initialized!").color(colors::LIME));
                } else {
                    ui.label(RichText::new("Not initialized.").color(colors::RED));
                }
                ui.end_row();
            });

            ui.vertical_centered_justified(|ui| {
                if ui.button("Enter Parameters").clicked() {
                    self.ifs_settings = Some(Box::new(IfsSettingsWindow::default()));
                }
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui
                    .add_enabled(context.fractal_view.initialized, Button::new("Draw"))
                    .clicked()
                {
                    todo!()
                }
            });
            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset Settings").clicked() {
                    context.fractal_view = Default::default();
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.collapsing("Grid Settings", |ui| {
                ui.checkbox(&mut context.grid.is_enabled, "Enable Grid");
                ui.checkbox(
                    &mut canvas.screen_params.is_dragging_offset_enabled,
                    "Enable Drag & Offset",
                );

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Unit Length:");
                    ui.add(
                        DragValue::new(&mut canvas.screen_params.unit_length)
                            .speed(1)
                            .range(1.0..=f32::MAX)
                            .suffix(" cm."),
                    );
                });

                ui.add_space(10.0);

                ui.label(RichText::new("Colors:").strong());
                Grid::new("GridColors").num_columns(2).show(ui, |ui| {
                    ui.label("Axis X:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut context.grid.axis_x_color,
                        egui::color_picker::Alpha::Opaque,
                    );

                    ui.end_row();

                    ui.label("Axis Y:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut context.grid.axis_y_color,
                        egui::color_picker::Alpha::Opaque,
                    );

                    ui.end_row();

                    ui.label("Grid:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut context.grid.grid_color,
                        egui::color_picker::Alpha::Opaque,
                    );
                });

                ui.add_space(10.0);

                ui.vertical_centered(|ui| {
                    if ui.button("Reset Settings").clicked() {
                        context.grid = Default::default();
                        canvas.screen_params.is_dragging_offset_enabled = true;
                        canvas.screen_params.offset = (0.0, 0.0);
                        canvas.screen_params.unit_length = 1.0;
                    }
                });
            });
        });

        self.show_windows_if_opened(ui, context);
    }

    fn show_windows_if_opened(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        let windows: Vec<&mut Option<Box<dyn WindowOps>>> = vec![&mut self.ifs_settings];

        for window_option in windows {
            if let Some(window) = window_option {
                window.show(ui, context);

                if window.is_closed() {
                    *window_option = None;
                }
            }
        }
    }

    fn reset_to_defaults(&mut self, context: &mut Context, canvas: &mut Canvas) {
        context.grid = Default::default();
        context.fractal_view = Default::default();
        canvas.screen_params = Default::default();
    }
}
