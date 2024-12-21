use crate::context::Context;
use crate::fractal::examples::FractalExamples;
use crate::ui::components::canvas::Canvas;
use crate::ui::styles::colors;
use crate::ui::windows::ifs_settings::IfsSettingsWindow;
use crate::ui::windows::message::MessageWindow;
use crate::ui::windows::traits::window_ops::WindowOps;
use egui::{Button, DragValue, Grid, RichText};
use indoc::indoc;

pub struct Settings {
    pub panel_width: f32,

    error_window: Option<Box<dyn WindowOps>>,
    ifs_settings: Option<Box<dyn WindowOps>>,
    help_window: Option<Box<dyn WindowOps>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            panel_width: 250.0,
            ifs_settings: None,
            error_window: None,
            help_window: None,
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
            Grid::new("Fractal Status").num_columns(2).show(ui, |ui| {
                ui.label("Status: ");
                if context.fractal_state.is_initialized() {
                    ui.label(RichText::new("Initialized!").color(colors::LIME));
                } else {
                    ui.label(RichText::new("Not initialized.").color(colors::RED));
                }
                ui.end_row();
            });

            ui.add_space(10.0);

            Grid::new("Fractal Settings").num_columns(2).show(ui, |ui| {
                ui.label("Iterations: ");
                ui.add(
                    DragValue::new(&mut context.fractal_state.iterations)
                        .speed(1)
                        .range(0..=u32::MAX),
                );
                ui.end_row();

                ui.label("Dot Radius: ");
                ui.add(
                    DragValue::new(&mut context.fractal_state.radius_centimeters)
                        .speed(0.01)
                        .range(0.01..=5.0)
                        .suffix(" cm."),
                );
                ui.end_row();
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui.button("Enter Parameters").clicked() {
                    self.ifs_settings = Some(Box::new(IfsSettingsWindow::default()));
                }
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui
                    .add_enabled(context.fractal_state.is_initialized(), Button::new("Draw"))
                    .clicked()
                {
                    context.fractal_state.request_draw();
                }
            });
            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset Settings").clicked() {
                    context.fractal_state = Default::default();
                }
            });

            ui.add_space(10.0);

            ui.collapsing("Load from Example", |ui| {
                ui.vertical_centered_justified(|ui| {
                    for example in FractalExamples::iter() {
                        if ui.button(example.to_string()).clicked() {
                            if let Err(err) = context
                                .file_loader
                                .load_from_path(&mut context.fractal_state, example.path())
                            {
                                context.fractal_state = Default::default();
                                self.error_window = Some(Box::new(err.window()))
                            }
                        }
                    }
                });
            });

            ui.add_space(10.0);

            ui.collapsing("Load from File", |ui| {
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Open File...").clicked() {
                        if let Err(err) = context
                            .file_loader
                            .load_with_file_pick(&mut context.fractal_state)
                        {
                            context.fractal_state = Default::default();
                            self.error_window = Some(Box::new(err.window()))
                        }
                    }
                    if ui.button("Help").clicked() {
                        let message = indoc! {"
                            File format: CSV file with 7 numbers.
                            Numbers: A, B, D, E, C, F, Probability.

                            Example:
                            0,0,0,0.16,0,0,0.01
                            0.85,0.04,-0.04,0.85,0,1.6,0.85
                            0.2,-0.26,0.23,0.22,0,1.6,0.07
                            -0.15,0.28,0.26,0.24,0,0.44,0.07

                            You can find other examples in the 'project/examples' folder.
                        "};
                        self.help_window = Some(Box::new(
                            MessageWindow::default()
                                .with_message(message)
                                .with_name("Help ❓")
                                .with_height(500.0)
                                .with_width(300.0)
                                .with_collapsible(false),
                        ));
                    }
                });
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
        let windows: Vec<&mut Option<Box<dyn WindowOps>>> = vec![
            &mut self.error_window,
            &mut self.help_window,
            &mut self.ifs_settings,
        ];

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
        context.file_loader = Default::default();
        context.fractal_state = Default::default();
        canvas.screen_params = Default::default();
    }
}
