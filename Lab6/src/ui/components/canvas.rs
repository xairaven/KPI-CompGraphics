use crate::context::Context;
use crate::geometry::line2d::Line2D;
use crate::geometry::point2d::Point2D;
use crate::graphics::screen::ScreenParams;
use crate::ui::styles::colors;
use eframe::epaint::Shape;
use egui::{Frame, Response, Sense};

#[derive(Default)]
pub struct Canvas {
    pub screen_params: ScreenParams,

    pub lines: Vec<Line2D>,
}

impl Canvas {
    pub fn process(&mut self, context: &mut Context, _ui: &mut egui::Ui) {
        let mut converted_lines: Vec<Line2D> = vec![];

        // Axes processing
        if context.axes.are_enabled {
            let axes = context.axes.lines();

            axes.iter().for_each(|line3d| {
                let axis2d = line3d.to_line2d(&context.trimetric);
                converted_lines.push(axis2d);
            });
        }

        // Surface
        let surface = context.surface.generate(self.screen_params);

        // Surface to 2D
        surface.iter().for_each(|line3d| {
            let line = line3d.to_line2d(&context.trimetric);
            converted_lines.push(line);
        });

        // Passing all lines to draw() method
        self.lines = converted_lines;
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::click_and_drag());
        self.screen_params.canvas_center = Point2D::from_pos2(response.rect.center());

        let line_shapes: Vec<Shape> = self
            .lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(line_shapes);

        // Check for dragging
        self.screen_params.update_offset_on_drag(ui, &response);

        response
    }

    pub fn show_content(&mut self, context: &mut Context, ui: &mut egui::Ui) {
        Frame::canvas(ui.style())
            .fill(colors::WHITE)
            .show(ui, |ui| {
                ui.input(|i| {
                    let delta = i.smooth_scroll_delta.y;
                    self.screen_params.px_per_cm += delta * 0.1;
                });
                self.process(context, ui);
                self.draw(ui);
            });
    }
}
