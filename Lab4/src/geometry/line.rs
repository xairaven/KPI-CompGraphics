use crate::graphics::screen::ScreenParams;
use crate::traits::positionable::Positionable;
use eframe::epaint::Shape;
use egui::Stroke;

#[derive(Debug, Default, Clone, Copy)]
pub struct Line<T: Positionable + Clone + Copy> {
    pub start: T,
    pub end: T,

    pub stroke: Stroke,
}

impl<T: Positionable + Clone + Copy> Line<T> {
    pub fn new(start: T, end: T, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }

    pub fn transparent(start: T, end: T) -> Self {
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
        f32::sqrt(
            (self.end.x() - self.start.x()).powf(2.0) + (self.end.y() - self.start.y()).powf(2.0),
        )
    }
}
