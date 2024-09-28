use crate::models::point::Point;
use crate::models::screen_params::ScreenParams;
use egui::{Color32, Shape};

pub const ROTATION_DOT_RADIUS: f32 = 5.0;

pub struct Euclidean {
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_angle: f32,

    rotation_dot_color: Color32,

    pub offset_x: f32,
    pub offset_y: f32,
}

impl Default for Euclidean {
    fn default() -> Self {
        Self {
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_angle: 0.0,

            rotation_dot_color: Color32::from_rgb(255, 0, 0),

            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

impl Euclidean {
    pub fn rotation_dot_shape(&self, screen_params: ScreenParams) -> Shape {
        let rotation_center = Point {
            x: self.rotation_x,
            y: self.rotation_y,
        }
        .to_screen(screen_params);

        if self.rotation_x == 0.0 && self.rotation_y == 0.0 {
            return Shape::circle_filled(
                rotation_center,
                ROTATION_DOT_RADIUS,
                Color32::from_white_alpha(0),
            );
        }

        Shape::circle_filled(
            rotation_center,
            ROTATION_DOT_RADIUS,
            self.rotation_dot_color,
        )
    }
}
