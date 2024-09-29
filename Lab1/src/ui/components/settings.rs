use crate::context::Context;
use crate::ui::components::canvas::Canvas;
use crate::utils::egui::label_centered_with_drag;
use egui::style::HandleShape;
use egui::{Color32, DragValue, Grid, RichText, Slider};

pub const MAX_RESIZING: u32 = 300;
pub const MAX_ROTATION_DOT_CORDS: u32 = 200;

pub const GRID_SIZE: f32 = 200.0;

pub const MAX_P_SCROLL_OFFSET: f32 = GRID_SIZE;
pub const MAX_N_SCROLL_OFFSET: f32 = -1.0 * GRID_SIZE;

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
                reset_to_defaults(context, canvas);
            }
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Scroll");

                Grid::new("LengthTransformationsGrid")
                    .min_col_width(100.0)
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Horizontal:");
                        ui.add(
                            Slider::new(
                                &mut canvas.screen_params.offset_x,
                                MAX_N_SCROLL_OFFSET..=MAX_P_SCROLL_OFFSET,
                            )
                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 }),
                        );
                        ui.end_row();

                        ui.label("Vertical:");
                        ui.add(
                            Slider::new(
                                &mut canvas.screen_params.offset_y,
                                MAX_N_SCROLL_OFFSET..=MAX_P_SCROLL_OFFSET,
                            )
                            .handle_shape(HandleShape::Rect { aspect_ratio: 0.5 }),
                        );
                        ui.end_row();
                    });
            });
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
                        &mut context.resize.length_ab,
                        1,
                        1..=MAX_RESIZING,
                    );
                    label_centered_with_drag(
                        ui,
                        "BC",
                        &mut context.resize.length_bc,
                        1,
                        1..=MAX_RESIZING,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "CD",
                        &mut context.resize.length_cd,
                        1,
                        1..=MAX_RESIZING,
                    );
                    label_centered_with_drag(
                        ui,
                        "DE",
                        &mut context.resize.length_de,
                        1,
                        1..=MAX_RESIZING,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "EF",
                        &mut context.resize.length_ef,
                        1,
                        1..=MAX_RESIZING,
                    );
                    label_centered_with_drag(
                        ui,
                        "FG",
                        &mut context.resize.length_fg,
                        1,
                        1..=MAX_RESIZING,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "GH",
                        &mut context.resize.length_gh,
                        1,
                        1..=MAX_RESIZING,
                    );
                    label_centered_with_drag(
                        ui,
                        "AH",
                        &mut context.resize.length_ah,
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
                .min_col_width(50.0)
                .num_columns(4)
                .show(ui, |ui| {
                    label_centered_with_drag(ui, "I:", &mut context.resize.radius_i, 1, 1..=100);
                    label_centered_with_drag(ui, "J:", &mut context.resize.radius_j, 1, 1..=100);
                    ui.end_row();
                });
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Euclidean Transformations").strong());
        });
        ui.add_space(5.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Rotation").color(Color32::from_rgb(255, 0, 0)));
            });

            ui.add_space(5.0);

            Grid::new("RotationGrid")
                .num_columns(2)
                .striped(true)
                .min_col_width(125.0)
                .show(ui, |ui| {
                    label_centered_with_drag(
                        ui,
                        "X:",
                        &mut context.euclidean.rotation_x,
                        1,
                        0..=MAX_ROTATION_DOT_CORDS,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "Y:",
                        &mut context.euclidean.rotation_y,
                        1,
                        0..=MAX_ROTATION_DOT_CORDS,
                    );
                    ui.end_row();

                    label_centered_with_drag(
                        ui,
                        "Angle Rotation:",
                        &mut context.euclidean.rotation_angle,
                        1,
                        0..=360,
                    );
                    ui.end_row();
                });
        });
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Offset");
            });

            ui.add_space(5.0);

            Grid::new("OffsetGrid")
                .num_columns(4)
                .striped(true)
                .min_col_width(50.0)
                .show(ui, |ui| {
                    label_centered_with_drag(
                        ui,
                        "X:",
                        &mut context.euclidean.offset_x,
                        1,
                        -1.0 * GRID_SIZE..=GRID_SIZE,
                    );

                    label_centered_with_drag(
                        ui,
                        "Y:",
                        &mut context.euclidean.offset_y,
                        1,
                        -1.0 * GRID_SIZE..=GRID_SIZE,
                    );
                    ui.end_row();
                });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.button("\t\tApply\t\t").clicked() {
                    // TODO: OFFSET APPLY
                }
            });
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Affine Transformations").strong());
        });
        ui.add_space(5.0);

        ui.group(|ui| {
            Grid::new("AffineTransformationsGrid")
                .min_col_width(60.0)
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    label_centered_with_drag(ui, "Xx", &mut context.affine.xx, 1, 0..=100);
                    label_centered_with_drag(ui, "Xy", &mut context.affine.xy, 1, 0..=100);
                    ui.end_row();

                    label_centered_with_drag(ui, "Yx", &mut context.affine.yx, 1, 0..=100);
                    label_centered_with_drag(ui, "Yy", &mut context.affine.yy, 1, 0..=100);
                    ui.end_row();

                    label_centered_with_drag(ui, "0x", &mut context.affine.zero_x, 1, 0..=100);
                    label_centered_with_drag(ui, "0y", &mut context.affine.zero_y, 1, 0..=100);
                    ui.end_row();
                });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Scaling");
            });

            ui.add_space(5.0);

            Grid::new("ScalingTransformationsGrid")
                .min_col_width(50.0)
                .num_columns(4)
                .show(ui, |ui| {
                    label_centered_with_drag(ui, "Mx:", &mut context.affine.scaling_x, 1, -10..=10);
                    label_centered_with_drag(ui, "My:", &mut context.affine.scaling_y, 1, -10..=10);
                    ui.end_row();
                });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label("Point symmetry");
            });

            ui.add_space(5.0);

            Grid::new("PointSymmetryTransformationsGrid")
                .min_col_width(50.0)
                .num_columns(4)
                .show(ui, |ui| {
                    label_centered_with_drag(ui, "X:", &mut context.affine.symmetry_x, 1, -10..=10);
                    label_centered_with_drag(ui, "Y:", &mut context.affine.symmetry_y, 1, -10..=10);
                    ui.end_row();
                });

            ui.add_space(10.0);

            ui.vertical_centered(|ui| {
                if ui.button("\t\tApply\t\t").clicked() {
                    // TODO: POINT SYMMETRY APPLY
                }
            });
        });

        ui.add_space(10.0);

        ui.vertical_centered(|ui| {
            ui.label(RichText::new("Projective Transformations").strong());
        });
        ui.add_space(5.0);

        ui.group(|ui| {
            Grid::new("ProjectiveTransformationsGrid")
                .striped(true)
                .num_columns(6)
                .show(ui, |ui| {
                    label_centered_with_drag(ui, "Xx", &mut context.projective.xx, 1, 0..=100);
                    label_centered_with_drag(ui, "Xy", &mut context.projective.xy, 1, 0..=100);
                    label_centered_with_drag(ui, "wX", &mut context.projective.wx, 1, 0..=100);
                    ui.end_row();

                    label_centered_with_drag(ui, "Yx", &mut context.projective.yx, 1, 0..=100);
                    label_centered_with_drag(ui, "Yy", &mut context.projective.yy, 1, 0..=100);
                    label_centered_with_drag(ui, "wY", &mut context.projective.wy, 1, 0..=100);
                    ui.end_row();

                    label_centered_with_drag(ui, "0x", &mut context.projective.zero_x, 1, 0..=100);
                    label_centered_with_drag(ui, "0y", &mut context.projective.zero_y, 1, 0..=100);
                    label_centered_with_drag(ui, "w0", &mut context.projective.w_zero, 1, 0..=100);
                    ui.end_row();
                });
        });
    });
}

fn reset_to_defaults(context: &mut Context, canvas: &mut Canvas) {
    context.model = Default::default();

    context.affine = Default::default();
    context.euclidean = Default::default();
    context.projective = Default::default();
    context.resize = Default::default();

    canvas.screen_params = Default::default();
}
