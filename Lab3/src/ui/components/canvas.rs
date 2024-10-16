use crate::context::Context;
use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::graphics::screen::{Resolution, ScreenParams};
use crate::ui::styles::colors;
use eframe::epaint::Shape;
use egui::{Frame, Response, Sense};

#[derive(Default)]
pub struct Canvas {
    pub screen_params: ScreenParams,

    pub grid_lines: Vec<Line>,
}

impl Canvas {
    pub fn process(&mut self, _ui: &mut egui::Ui, context: &mut Context) {
        // Creating grid:
        self.grid_lines = context.grid.lines(self.screen_params);
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, _context: &mut Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::hover());
        self.screen_params.canvas_center = Point::from_pos2(response.rect.center());
        self.screen_params.resolution = Resolution::from(response.rect.max.x, response.rect.max.y);

        // Draw grid:
        let grid_shapes: Vec<Shape> = self
            .grid_lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(grid_shapes);

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
