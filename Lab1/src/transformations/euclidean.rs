use crate::math::angle::Angle;
use crate::models::circle::Circle;
use crate::models::line::Line;
use crate::models::model::Model;
use crate::models::point::Point;
use egui::Color32;
use nalgebra::Matrix3;

pub const ROTATION_DOT_COLOR: Color32 = Color32::from_rgb(255, 0, 0);

pub struct Euclidean {
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub rotation_angle: f32,

    pub offset_x: f32,
    pub offset_y: f32,

    pub offset_applied: bool,
    pub rotation_applied: bool,
}

impl Default for Euclidean {
    fn default() -> Self {
        Self {
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_angle: 0.0,
            rotation_applied: false,

            offset_x: 0.0,
            offset_y: 0.0,
            offset_applied: false,
        }
    }
}

impl Euclidean {
    pub fn process_rotation(&self, model: Vec<Line>) -> Vec<Line> {
        if self.rotation_angle == 0.0 {
            return model;
        };

        model
            .iter()
            .map(|line| {
                let start = self.rotate(line.start);
                let end = self.rotate(line.end);
                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    pub fn apply_offset(&mut self, model: &mut Model) {
        let offset_point = Point {
            x: self.offset_x,
            y: self.offset_y,
        };

        let points = model.points_mut();

        for point in points {
            *point = *point + offset_point;
        }

        self.offset_x = 0.0;
        self.offset_y = 0.0;
        self.offset_applied = false;
    }

    pub fn apply_rotation(&mut self, model: &mut Model) {
        let points = model.points_mut();
        for point in points {
            *point = self.rotate(*point);
        }

        self.rotation_x = 0.0;
        self.rotation_y = 0.0;
        self.rotation_angle = 0.0;
        self.rotation_applied = false;
    }

    pub fn process_offset(&self, model: Vec<Line>) -> Vec<Line> {
        if self.offset_x == 0.0 && self.offset_y == 0.0 {
            return model;
        }

        model
            .iter()
            .map(|line| {
                let start = self.offset(line.start);
                let end = self.offset(line.end);
                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    pub fn rotate(&self, point: Point) -> Point {
        let point_vector = point.to_vector();
        let matrix = self.get_rotation_matrix();

        let answer = point_vector * matrix;

        Point::new(answer.x, answer.y)
    }

    pub fn offset(&self, point: Point) -> Point {
        Point {
            x: point.x + self.offset_x,
            y: point.y + self.offset_y,
        }
    }

    fn get_rotation_matrix(&self) -> Matrix3<f32> {
        let angle = Angle::from_degree(self.rotation_angle).radian();
        let rotation_point = Point::new(self.rotation_x, self.rotation_y);

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

    pub fn rotation_dot(&self) -> Circle {
        Circle {
            center: Point::new(self.rotation_x, self.rotation_y),
            ..Default::default()
        }
    }
}
