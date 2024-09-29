use crate::models::point::Point;
use crate::models::screen_params::ScreenParams;
use eframe::emath::Pos2;

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
}
