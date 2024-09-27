use crate::models::point::Point;
use eframe::epaint::Stroke;
use egui::Shape;

pub struct Line {
    start: Point,
    end: Point,

    stroke: Stroke,
}

impl Line {
    pub fn new(start: Point, end: Point, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }

    pub fn to_shape(&self) -> Shape {
        Shape::line(vec![self.start.to_pos2(), self.end.to_pos2()], self.stroke)
    }

    pub fn to_screen_shape(&self, canvas_height: f32, px_per_cm: f32) -> Shape {
        Shape::line(
            vec![
                self.start.to_screen(canvas_height, px_per_cm),
                self.end.to_screen(canvas_height, px_per_cm),
            ],
            self.stroke,
        )
    }
}
