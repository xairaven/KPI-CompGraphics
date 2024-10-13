use crate::models::point::Point;
use crate::models::screen::ScreenParams;
use crate::ui::styles::colors;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Shape};

pub const STANDARD_RADIUS: f32 = 0.15;

#[derive(Debug, Clone, Copy)]
pub struct Dot {
    pub center: Point,
    pub radius: f32,
}

impl Default for Dot {
    fn default() -> Self {
        Self {
            center: Point::default(),
            radius: STANDARD_RADIUS,
        }
    }
}

impl Dot {
    pub fn from_point(point: &Point) -> Self {
        debug_assert!(!point.converted_to_screen);

        Self {
            center: *point,
            radius: STANDARD_RADIUS,
        }
    }

    pub fn set_radius(&self, radius: f32) -> Self {
        assert!(radius >= 0.0);

        Self {
            center: self.center,
            radius,
        }
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Self {
        Self {
            center: self.center.to_screen(screen_params),
            radius: screen_params.convert_single(self.radius),
        }
    }

    pub fn to_pos2(&self) -> Pos2 {
        Pos2::from([self.center.x, self.center.y])
    }

    pub fn to_shape(&self, color: Color32) -> Shape {
        Shape::circle_filled(self.center.to_pos2(), self.radius, color)
    }

    pub fn to_transparent_shape(&self) -> Shape {
        Shape::circle_filled(self.center.to_pos2(), self.radius, colors::transparent())
    }
}
