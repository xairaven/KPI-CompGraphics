use crate::context::Context;
use crate::ui::windows::message::MessageWindow;
use crate::ui::windows::traits::window_ops::WindowOps;
use egui::{Button, DragValue, Grid};

pub struct IfsSettingsWindow {
    name: String,
    is_open: bool,
    collapsible: bool,
    resizable: bool,

    width: f32,
    height: f32,

    error_window: Option<MessageWindow>,
}

impl Default for IfsSettingsWindow {
    fn default() -> Self {
        Self {
            name: "IFS Settings".to_string(),
            is_open: true,
            collapsible: true,
            resizable: true,

            width: 450.0,
            height: 250.0,

            error_window: None,
        }
    }
}

impl WindowOps for IfsSettingsWindow {
    fn show(&mut self, ui: &egui::Ui, context: &mut Context) {
        let mut to_close = false;

        egui::Window::new(&self.name)
            .open(&mut self.is_open)
            .min_width(self.width)
            .min_height(self.height)
            .collapsible(self.collapsible)
            .resizable(self.resizable)
            .show(ui.ctx(), |ui| {
                egui::ScrollArea::vertical()
                    .max_height(self.height - 30.0)
                    .show(ui, |ui| {
                        let mut rule_removed: (bool, usize) = (false, 0);
                        Grid::new("SystemGrid")
                            .num_columns(8)
                            .striped(true)
                            .show(ui, |ui| {
                                for (index_system, system) in
                                    context.fractal_view.systems.iter_mut().enumerate()
                                {
                                    for element in &mut system[0..=5] {
                                        ui.add(
                                            DragValue::new(element)
                                                .speed(0.01)
                                                .range(0.00..=f32::MAX),
                                        );
                                    }

                                    ui.add(
                                        DragValue::new(&mut system[6])
                                            .speed(0.01)
                                            .range(0.01..=1.0),
                                    );

                                    if ui.button("Remove").clicked() {
                                        rule_removed = (true, index_system);
                                    }

                                    ui.end_row();
                                }
                            });
                        let (is_rule_removed, removed_rule_index) = rule_removed;
                        if is_rule_removed {
                            context.fractal_view.systems.remove(removed_rule_index);
                        }
                    });

                ui.add_space(10.0);

                ui.vertical_centered_justified(|ui| {
                    if ui.button("Add System").clicked() {
                        context.fractal_view.add_system();
                    }
                });

                ui.add_space(10.0);

                ui.columns(2, |columns| {
                    columns[0].vertical_centered(|ui| {
                        if ui
                            .add_sized([self.width / 2.0 - 15.0, 20.0], Button::new("Save"))
                            .clicked()
                        {
                            let initialization_result = context.fractal_view.initialize();

                            match initialization_result {
                                Ok(_) => {
                                    to_close = true;
                                },
                                Err(error) => {
                                    self.error_window = Some(error.window());
                                },
                            }
                        }
                    });
                    columns[1].vertical_centered(|ui| {
                        if ui
                            .add_sized([self.width / 2.0 - 15.0, 20.0], Button::new("Close"))
                            .clicked()
                        {
                            to_close = true;
                        }
                    });
                });
            });

        self.show_existing_errors(ui, context);

        if to_close {
            self.close();
        }
    }

    fn is_closed(&self) -> bool {
        !self.is_open
    }
}

impl IfsSettingsWindow {
    fn show_existing_errors(&mut self, ui: &egui::Ui, context: &mut Context) {
        let windows = vec![&mut self.error_window];

        for window_option in windows {
            if let Some(window) = window_option {
                window.show(ui, context);

                if window.is_closed() {
                    *window_option = None;
                }
            }
        }
    }

    fn close(&mut self) {
        self.is_open = false;
    }
}
