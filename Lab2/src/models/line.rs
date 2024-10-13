use crate::models::point::Point;
use crate::models::screen::ScreenParams;
use eframe::epaint::{Shape, Stroke};

#[derive(Debug, Default, Clone, Copy)]
pub struct Line {
    pub start: Point,
    pub end: Point,

    pub stroke: Stroke,
}

impl Line {
    pub fn new(start: Point, end: Point, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }
    pub fn transparent(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
            stroke: Stroke::default(),
        }
    }

    pub fn is_transparent(&self) -> bool {
        self.stroke == Stroke::default()
    }

    pub fn to_shape(&self) -> Shape {
        Shape::line(vec![self.start.to_pos2(), self.end.to_pos2()], self.stroke)
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Self {
        Self {
            start: self.start.to_screen(screen_params),
            end: self.end.to_screen(screen_params),
            stroke: self.stroke,
        }
    }

    pub fn length(&self) -> f32 {
        f32::sqrt((self.end.x - self.start.x).powf(2.0) + (self.end.y - self.start.y).powf(2.0))
    }
}
