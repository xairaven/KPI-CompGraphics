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
    pub fn process(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        let axes = context.axes.lines(self.screen_params);
        let model = context.model.lines(self.screen_params);

        // Converting lines to 2D (with z=0)
        let mut lines2d: Vec<Line2D> = vec![];

        axes.iter().for_each(|line3d| {
            let axis2d = line3d.to_line2d(&context.trimetric);
            lines2d.push(axis2d);
        });
        model.iter().for_each(|line3d| {
            let line = line3d.to_line2d(&context.trimetric);
            lines2d.push(line);
        });

        // Projection to z=0.
        // TODO

        // Passing all lines to draw method
        self.lines = lines2d;
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, context: &mut Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::click_and_drag());
        self.screen_params.canvas_center = Point2D::from_pos2(response.rect.center());

        let line_shapes: Vec<Shape> = self
            .lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(line_shapes);

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
                self.process(ui, context);
                self.draw(ui, context);
            });
    }
}
