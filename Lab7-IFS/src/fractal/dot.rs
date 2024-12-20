use crate::geometry::point2d::Point2D;
use crate::graphics::screen::ScreenParams;
use eframe::epaint::Shape;
use egui::Color32;

const DEFAULT_RADIUS_CENTIMETERS: f32 = 0.01;

#[derive(Clone)]
pub struct Dot {
    pub point: Point2D,
    pub color: Color32,
    pub radius: f32,
}

impl Default for Dot {
    fn default() -> Self {
        Self {
            point: Point2D::new(0.0, 0.0),
            color: Default::default(),
            radius: DEFAULT_RADIUS_CENTIMETERS,
        }
    }
}

impl Dot {
    pub fn with_center(mut self, point: Point2D) -> Self {
        self.point = point;
        self
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn with_radius_centimeters(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Self {
        Self {
            point: self.point.to_screen(screen_params),
            color: self.color,
            radius: screen_params.value_cm_to_px(self.radius),
        }
    }

    pub fn to_shape(&self) -> Shape {
        debug_assert!(self.point.converted_to_screen);

        Shape::circle_filled(self.point.to_pos2(), self.radius, self.color)
    }
}
