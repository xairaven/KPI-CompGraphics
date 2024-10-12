use crate::context::Context;
use crate::ui::components::canvas::Canvas;
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
                    .range(1..=100),
            );
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            if ui.button("Reset to Default Settings").clicked() {
                context.model = Default::default();
                canvas.screen_params = Default::default();
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
                            .range(0..=10),
                    );

                    ui.label("B:");
                    ui.add(
                        DragValue::new(&mut context.model.b)
                            .speed(0.05)
                            .range(-10..=10),
                    );

                    ui.label("C:");
                    ui.add(
                        DragValue::new(&mut context.model.c)
                            .speed(0.05)
                            .range(-10..=10),
                    );

                    ui.end_row();
                })
        });

        ui.add_space(10.0);
    });
}
