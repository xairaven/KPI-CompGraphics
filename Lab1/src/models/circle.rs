use crate::models::point::Point;
use crate::models::screen_params::ScreenParams;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Shape};

pub const DOT_STANDARD_RADIUS: f32 = 2.5;

pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            center: Point::default(),
            radius: 0.0,
        }
    }
}

impl Circle {
    pub fn set_radius(&self, radius: f32) -> Self {
        Self {
            center: self.center,
            radius,
        }
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Self {
        let (x, y) = screen_params.convert_xy(self.center.x, self.center.y);
        Self {
            center: Point::new(x, y),
            radius: self.radius,
        }
    }

    pub fn to_screen_pos2(&self, screen_params: ScreenParams) -> Pos2 {
        let (x, y) = screen_params.convert_xy(self.center.x, self.center.y);
        Pos2::from([x, y])
    }

    pub fn shape_dot(circle: Circle, color: Color32, screen_params: ScreenParams) -> Shape {
        let screen_point = circle.to_screen_pos2(screen_params);

        let radius = screen_params.convert_single(circle.radius);

        if circle.center.x == 0.0 && circle.center.y == 0.0 {
            return Shape::circle_filled(screen_point, radius, Color32::from_white_alpha(0));
        }

        Shape::circle_filled(screen_point, radius, color)
    }
}
