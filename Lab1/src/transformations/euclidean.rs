use crate::math::angle::Angle;
use crate::models::line::Line;
use crate::models::point::Point;
use crate::models::screen_params::ScreenParams;
use egui::{Color32, Shape};
use nalgebra::Matrix3;

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
    pub fn process_rotation(&self, model: Vec<Line>, screen_params: ScreenParams) -> Vec<Line> {
        if self.rotation_x == 0.0 && self.rotation_y == 0.0 && self.rotation_angle == 0.0 {
            return model;
        };

        model
            .iter()
            .map(|line| {
                let start = self.rotate(line.start, screen_params);
                let end = self.rotate(line.end, screen_params);
                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    pub fn rotate(&self, point: Point, screen_params: ScreenParams) -> Point {
        // Already in screen coordinates
        let point_vector = point.to_vector();

        // Getting matrix. Coordinates are converting to screen
        let matrix = self.get_rotation_matrix(screen_params);

        let answer = point_vector * matrix;

        Point::new(answer.x, answer.y)
    }

    fn get_rotation_matrix(&self, screen_params: ScreenParams) -> Matrix3<f32> {
        let angle = Angle::from_degree(self.rotation_angle).radian();
        let rotation_point = Point::new(self.rotation_x, self.rotation_y).to_screen(screen_params);

        let m11 = f32::cos(angle);
        let m12 = f32::sin(angle);
        let m21 = -1.0 * f32::sin(angle);
        let m22 = f32::cos(angle);
        let m31 =
            -1.0 * rotation_point.x * (f32::cos(angle) - 1.0) + rotation_point.y * f32::sin(angle);
        let m32 =
            -1.0 * rotation_point.x * f32::sin(angle) - rotation_point.y * (f32::cos(angle) - 1.0);

        Matrix3::new(m11, m12, 0.0, m21, m22, 0.0, m31, m32, 1.0)
    }

    pub fn shape_rotation_dot(&self, screen_params: ScreenParams) -> Shape {
        let rotation_center = Point {
            x: self.rotation_x,
            y: self.rotation_y,
        }
        .to_screen_pos2(screen_params);

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
