use crate::context::Context;
use egui::{emath, vec2, Color32, Frame, Pos2, Rect, Response, Sense, Stroke, Vec2};

pub struct Canvas {
    pub lines: Vec<Vec<Pos2>>,
    pub px_per_cm: f32,

    pub axis_stroke: Stroke,
    pub grid_stroke: Stroke,
    pub model_stroke: Stroke,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            px_per_cm: 35.0,

            axis_stroke: Stroke::new(1.8, Color32::from_rgb(0, 0, 0)),
            grid_stroke: Stroke::new(0.8, Color32::from_rgb(150, 150, 150)),
            model_stroke: Stroke::new(2.0, Color32::from_rgb(0, 0, 0)),
        }
    }
}

impl Canvas {
    pub fn draw(&mut self, ui: &mut egui::Ui, context: &Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::hover());
        let canvas_height = response.rect.max.y;

        let positions: [Pos2; 2] = [Pos2::from([0.0, 0.0]), Pos2::from([100.0, 100.0])];

        let transformed_points: Vec<Pos2> = positions
            .into_iter()
            .map(|p| Self::inverse(p, canvas_height))
            .collect();
        let lines = vec![egui::Shape::line(transformed_points, self.model_stroke)];

        painter.extend(lines);

        response
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui, context: &Context) {
        Frame::canvas(ui.style())
            .fill(Color32::from_rgb(255, 255, 255))
            .show(ui, |ui| {
                self.draw(ui, context);
            });
    }

    fn inverse(pos: Pos2, max_y: f32) -> Pos2 {
        let init_x: f32 = 50.0;
        let init_y: f32 = 100.0;

        Pos2::from([pos.x + init_x, max_y - pos.y - init_y])
    }
}
