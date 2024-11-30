use crate::context::Context;
use crate::model::examples::FractalExamples;
use crate::ui::components::canvas::Canvas;
use crate::ui::windows::message::MessageWindow;
use egui::{vec2, DragValue, Grid, RichText};
use indoc::indoc;

pub struct Settings {
    pub panel_width: f32,

    error_window: Option<MessageWindow>,
    loading_help_window: Option<MessageWindow>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            panel_width: 250.0,
            error_window: None,
            loading_help_window: None,
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
                ui.label("Axiom:");
                ui.text_edit_singleline(&mut context.fractal_view_model.axiom);
                ui.end_row();

                ui.label("Angle:");
                ui.add(
                    DragValue::new(&mut context.fractal_view_model.angle)
                        .speed(1)
                        .range(0..=360)
                        .suffix("°"),
                );
                ui.end_row();

                ui.label("Initial Angle:");
                ui.add(
                    DragValue::new(&mut context.fractal_view_model.initial_angle)
                        .speed(1)
                        .range(0..=360)
                        .suffix("°"),
                );
                ui.end_row();

                ui.label("Iterations:");
                ui.add(
                    DragValue::new(&mut context.fractal_view_model.iterations)
                        .speed(1)
                        .range(1..=usize::MAX),
                );
                ui.end_row();

                ui.label("Length:");
                ui.add(
                    DragValue::new(&mut context.fractal_view_model.length)
                        .speed(0.01)
                        .range(0.01..=f32::MAX)
                        .suffix(" cm."),
                );
                ui.end_row();

                ui.label("Color:");
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    &mut context.fractal_view_model.color,
                    egui::color_picker::Alpha::Opaque,
                );
                ui.end_row();
            });

            ui.add_space(10.0);
            ui.label("Rules:");
            ui.add_space(5.0);
            let mut rule_removed: (bool, usize) = (false, 0);
            for (rule_index, rule_line) in context.fractal_view_model.rules.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.add_sized(vec2(200.0, 12.5), egui::TextEdit::singleline(rule_line))
                        .on_hover_text("Format:\nSymbol -> Rule\n\nFor Example:\nX -> X+YF+");
                    if ui.button("Remove").clicked() {
                        rule_removed = (true, rule_index);
                    }
                });
            }
            let (is_rule_removed, removed_rule_index) = rule_removed;
            if is_rule_removed {
                context.fractal_view_model.rules.remove(removed_rule_index);
            }
            ui.vertical_centered_justified(|ui| {
                if ui.button("Add Rule").clicked() {
                    context.fractal_view_model.rules.push(String::new());
                }
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui.button("Draw").clicked() {
                    if let Err(err) = context.fractal_view_model.request_draw() {
                        context.fractal_view_model.reset_fractal_settings();
                        self.error_window = Some(err.window())
                    }
                }
            });

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset Settings").clicked() {
                    context.fractal_view_model = Default::default();
                }
            });

            ui.add_space(10.0);

            ui.collapsing("Load from Example", |ui| {
                ui.vertical_centered_justified(|ui| {
                    for example in FractalExamples::iter() {
                        if ui.button(example.to_string()).clicked() {
                            if let Err(err) = context
                                .loader
                                .load_from_path(&mut context.fractal_view_model, example.path())
                            {
                                context.fractal_view_model.reset_fractal_settings();
                                self.error_window = Some(err.window())
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
                            .loader
                            .load_with_file_pick(&mut context.fractal_view_model)
                        {
                            context.fractal_view_model.reset_fractal_settings();
                            self.error_window = Some(err.window())
                        }
                    }
                    if ui.button("Help").clicked() {
                        let message = indoc! {"
                            File format:

                            Axiom = ...
                            Angle = ...
                            Initial Angle = ...
                            Iterations = ...
                            SymbolForRule1 -> ...
                            SymbolForRule2 -> ...
                        
                            ---
                        
                            Example:
                        
                            Axiom = FX
                            Angle = 90
                            Iterations = 5
                            X -> X+YF+
                            Y -> -FX-Y
                        "};
                        self.loading_help_window = Some(
                            MessageWindow::default()
                                .with_message(message)
                                .with_name("Help ❓")
                                .with_height(500.0)
                                .with_width(300.0)
                                .with_collapsible(false),
                        )
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

        self.show_windows_if_opened(ui);
    }

    fn show_windows_if_opened(&mut self, ui: &mut egui::Ui) {
        let windows = vec![&mut self.loading_help_window, &mut self.error_window];

        for window_option in windows {
            if let Some(window) = window_option {
                window.show(ui);

                if window.is_closed() {
                    *window_option = None;
                }
            }
        }
    }

    fn reset_to_defaults(&self, context: &mut Context, canvas: &mut Canvas) {
        context.grid = Default::default();
        context.fractal_view_model = Default::default();
        context.loader = Default::default();

        canvas.screen_params = Default::default();
    }
}
