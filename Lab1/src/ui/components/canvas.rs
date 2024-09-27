use crate::context::Context;
use egui::{Color32, Frame, Response, Sense};

pub const INIT_X: f32 = 50.0;
pub const INIT_Y: f32 = 50.0;

pub struct Canvas {
    pub px_per_cm: f32,
}

impl Default for Canvas {
    fn default() -> Self {
        Self { px_per_cm: 20.0 }
    }
}

impl Canvas {
    pub fn draw(&mut self, ui: &mut egui::Ui, context: &Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::hover());
        let canvas_height = response.rect.max.y;

        let grid_shapes = context.grid.shape(canvas_height, self.px_per_cm);
        painter.extend(grid_shapes);

        let model_shapes = context.model.shape(canvas_height, self.px_per_cm);
        painter.extend(model_shapes);

        response
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui, context: &Context) {
        Frame::canvas(ui.style())
            .fill(Color32::from_rgb(255, 255, 255))
            .show(ui, |ui| {
                self.draw(ui, context);
            });
    }
}
