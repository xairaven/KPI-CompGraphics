use crate::context::Context;
use crate::models::line::Line;
use crate::models::point::Point;
use crate::models::screen::ScreenParams;
use eframe::epaint::{Color32, Shape};
use egui::{Frame, Response, Sense};

#[derive(Default)]
pub struct Canvas {
    pub screen_params: ScreenParams,

    pub grid_lines: Vec<Line>,
    pub model_lines: Vec<Line>,
}

impl Canvas {
    pub fn process(&mut self, context: &mut Context) {
        // Creating grid:
        self.grid_lines = context.grid.lines();

        // Creating model:
        let lines: Vec<Line> = vec![];
        self.model_lines = lines;
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::hover());
        self.screen_params.canvas_center = Point::from_pos2(response.rect.center());

        // Draw grid:
        let grid_shapes: Vec<Shape> = self
            .grid_lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(grid_shapes);

        // Draw model:
        let model_shapes: Vec<Shape> = self
            .model_lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(model_shapes);

        response
    }

    pub fn show_content(&mut self, context: &mut Context, ui: &mut egui::Ui) {
        Frame::canvas(ui.style())
            .fill(Color32::from_rgb(255, 255, 255))
            .show(ui, |ui| {
                self.process(context);
                self.draw(ui);
            });
    }
}
