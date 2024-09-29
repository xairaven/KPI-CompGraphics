use crate::context::Context;
use crate::models::line::Line;
use crate::models::screen_params::ScreenParams;
use eframe::epaint::Shape;
use egui::{Color32, Frame, Response, Sense};

#[derive(Default)]
pub struct Canvas {
    pub screen_params: ScreenParams,
}

impl Canvas {
    pub fn draw(&mut self, ui: &mut egui::Ui, context: &mut Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::hover());
        self.screen_params.canvas_height = response.rect.max.y;

        // ALL TRANSFORMATIONS
        // Resizing
        context.resize.update_values(&mut context.model);

        // Get model lines
        let model_lines: Vec<Line> = context.model.lines();

        // Get grid lines
        let grid_lines: Vec<Line> = context.grid.lines();

        // Euclidean Offset
        let mut model_shadow: Vec<Line> = Line::color_shadow(&model_lines);
        let model_lines: Vec<Line> = context.euclidean.process_offset(model_lines);
        if context.euclidean.offset_applied {
            context.euclidean.apply_offset(&mut context.model)
        }

        // Euclidean Rotation
        let model_lines: Vec<Line> = context.euclidean.process_rotation(model_lines);
        if context.euclidean.offset_x != 0.0 || context.euclidean.offset_y != 0.0 {
            model_shadow = context.euclidean.process_rotation(model_shadow);
        }

        // Affine
        let model_lines: Vec<Line> = context.affine.process_affine(model_lines);
        if context.euclidean.offset_x != 0.0 || context.euclidean.offset_y != 0.0 {
            model_shadow = context.affine.process_affine(model_shadow);
        }
        let grid_lines: Vec<Line> = context.affine.process_affine(grid_lines);

        // DRAWING
        // Draw grid
        let grid_shapes: Vec<Shape> = grid_lines
            .iter()
            .map(|line| line.to_screen_shape(self.screen_params))
            .collect();
        painter.extend(grid_shapes);

        // Draw model
        let model_shapes: Vec<Shape> = model_lines
            .iter()
            .map(|line| line.to_screen_shape(self.screen_params))
            .collect();
        painter.extend(model_shapes);

        // Draw shadow model
        if context.euclidean.offset_x != 0.0 || context.euclidean.offset_y != 0.0 {
            let shapes: Vec<Shape> = model_shadow
                .iter()
                .map(|line| line.to_screen_shape(self.screen_params))
                .collect();
            painter.extend(shapes);
        }

        // Draw Euclidean Rotation Dot
        // TODO: AFFINE DRAWING
        let rotation_dot = context.euclidean.shape_rotation_dot(self.screen_params);
        painter.add(rotation_dot);

        response
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        Frame::canvas(ui.style())
            .fill(Color32::from_rgb(255, 255, 255))
            .show(ui, |ui| {
                ui.input(|i| {
                    let delta = i.smooth_scroll_delta.y;
                    self.screen_params.px_per_cm += delta * 0.05;
                });
                self.draw(ui, context);
            });
    }
}
