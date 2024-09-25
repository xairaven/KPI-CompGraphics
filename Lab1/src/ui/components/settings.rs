use crate::ui::app_model::AppModel;
use crate::utils::egui::label_centered_with_drag;
use egui::{DragValue, Grid};

pub const MAX_RESIZING: u32 = 300;

pub fn show_panel(app: &mut AppModel, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Settings");
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Pixels per Centimeter:");
            ui.add(
                DragValue::new(&mut app.canvas.px_per_cm)
                    .speed(1)
                    .range(1..=100),
            );
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            if ui.button("Set Default Figure").clicked() {
                set_default_figure(app);
            }
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Length");
            });

            ui.add_space(5.0);

            Grid::new("LengthTransformationsGrid")
                .min_col_width(60.0)
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    label_centered_with_drag(
                        ui,
                        "AB",
                        &mut app.resize.length_ab,
                        1,
                        1..=MAX_RESIZING,
                    );
                    label_centered_with_drag(
                        ui,
                        "BC",
                        &mut app.resize.length_bc,
                        1,
                        1..=MAX_RESIZING,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "CD",
                        &mut app.resize.length_cd,
                        1,
                        1..=MAX_RESIZING,
                    );
                    label_centered_with_drag(
                        ui,
                        "DE",
                        &mut app.resize.length_de,
                        1,
                        1..=MAX_RESIZING,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "EF",
                        &mut app.resize.length_ef,
                        1,
                        1..=MAX_RESIZING,
                    );
                    label_centered_with_drag(
                        ui,
                        "FG",
                        &mut app.resize.length_fg,
                        1,
                        1..=MAX_RESIZING,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "GH",
                        &mut app.resize.length_gh,
                        1,
                        1..=MAX_RESIZING,
                    );
                    label_centered_with_drag(
                        ui,
                        "AH",
                        &mut app.resize.length_ah,
                        1,
                        1..=MAX_RESIZING,
                    );
                    ui.end_row();
                });
        });
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Radius");
            });

            ui.add_space(5.0);

            Grid::new("RadiusTransformationsGrid")
                .min_col_width(30.0)
                .num_columns(4)
                .show(ui, |ui| {
                    label_centered_with_drag(
                        ui,
                        "I",
                        &mut app.resize.radius_i,
                        1,
                        1..=100,
                    );
                    label_centered_with_drag(
                        ui,
                        "J",
                        &mut app.resize.radius_j,
                        1,
                        1..=100,
                    );
                    ui.end_row();
                });
        });
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Euclidean Transformations");
            });

            ui.add_space(5.0);

            Grid::new("EuclideanTransformationsGrid")
                .num_columns(2)
                .striped(true)
                .min_col_width(125.0)
                .show(ui, |ui| {
                    label_centered_with_drag(
                        ui,
                        "X Offset:",
                        &mut app.euclidean.offset_x,
                        1,
                        0..=100,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "Y Offset:",
                        &mut app.euclidean.offset_y,
                        1,
                        0..=100,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "Angle Rotation:",
                        &mut app.euclidean.rotation_angle,
                        1,
                        0..=100,
                    );
                    ui.end_row();
                });
        });
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Affine Transformations");
            });

            ui.add_space(5.0);

            Grid::new("AffineTransformationsGrid")
                .min_col_width(60.0)
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    label_centered_with_drag(ui, "Xx", &mut app.affine.xx, 1, 0..=100);
                    label_centered_with_drag(ui, "Xy", &mut app.affine.xy, 1, 0..=100);
                    ui.end_row();

                    label_centered_with_drag(ui, "Yx", &mut app.affine.yx, 1, 0..=100);
                    label_centered_with_drag(ui, "Yy", &mut app.affine.yy, 1, 0..=100);
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "0x",
                        &mut app.affine.zero_x,
                        1,
                        0..=100,
                    );
                    label_centered_with_drag(
                        ui,
                        "0y",
                        &mut app.affine.zero_y,
                        1,
                        0..=100,
                    );
                    ui.end_row();
                });
        });
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Projective Transformations");
            });

            ui.add_space(5.0);

            Grid::new("ProjectiveTransformationsGrid")
                .striped(true)
                .num_columns(6)
                .show(ui, |ui| {
                    label_centered_with_drag(
                        ui,
                        "Xx",
                        &mut app.projective.xx,
                        1,
                        0..=100,
                    );
                    label_centered_with_drag(
                        ui,
                        "Xy",
                        &mut app.projective.xy,
                        1,
                        0..=100,
                    );
                    label_centered_with_drag(
                        ui,
                        "wX",
                        &mut app.projective.wx,
                        1,
                        0..=100,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "Yx",
                        &mut app.projective.yx,
                        1,
                        0..=100,
                    );
                    label_centered_with_drag(
                        ui,
                        "Yy",
                        &mut app.projective.yy,
                        1,
                        0..=100,
                    );
                    label_centered_with_drag(
                        ui,
                        "wY",
                        &mut app.projective.wy,
                        1,
                        0..=100,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "0x",
                        &mut app.projective.zero_x,
                        1,
                        0..=100,
                    );
                    label_centered_with_drag(
                        ui,
                        "0y",
                        &mut app.projective.zero_y,
                        1,
                        0..=100,
                    );
                    label_centered_with_drag(
                        ui,
                        "w0",
                        &mut app.projective.w_zero,
                        1,
                        0..=100,
                    );
                    ui.end_row();
                });
        });
    });
}

fn set_default_figure(app: &mut AppModel) {
    app.affine = Default::default();
    app.euclidean = Default::default();
    app.projective = Default::default();
    app.resize = Default::default();
}
