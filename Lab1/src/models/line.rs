use crate::models::point::Point;
use crate::models::screen_params::ScreenParams;
use eframe::epaint::Stroke;
use egui::Shape;

pub struct Line {
    pub start: Point,
    pub end: Point,

    pub stroke: Stroke,
}

impl Line {
    pub fn new(start: Point, end: Point, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }
    pub fn new_plain(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
            stroke: Stroke::default(),
        }
    }

    pub fn to_shape(&self) -> Shape {
        Shape::line(vec![self.start.to_pos2(), self.end.to_pos2()], self.stroke)
    }

    pub fn to_screen_shape(&self, screen_params: ScreenParams) -> Shape {
        Shape::line(
            vec![
                self.start.to_screen(screen_params),
                self.end.to_screen(screen_params),
            ],
            self.stroke,
        )
    }

    pub fn length(&self) -> f32 {
        f32::sqrt((self.end.x - self.start.x).powf(2.0) + (self.end.y - self.start.y).powf(2.0))
    }
}
