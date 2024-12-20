use crate::geometry::point2d::Point2D;
use crate::graphics::screen::ScreenParams;
use eframe::epaint::Shape;
use egui::Color32;

#[derive(Clone)]
pub struct Dot {
    pub point: Point2D,
    pub color: Color32,
}

impl Dot {
    pub fn new(point: Point2D, color: Color32) -> Self {
        Self { point, color }
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Self {
        Self {
            point: self.point.to_screen(screen_params),
            color: self.color,
        }
    }

    pub fn to_shape(&self) -> Shape {
        debug_assert!(self.point.converted_to_screen);

        Shape::circle_filled(self.point.to_pos2(), 5.0, self.color)
    }
}
