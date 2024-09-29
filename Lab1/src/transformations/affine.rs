use crate::models::circle::Circle;
use crate::models::line::Line;
use crate::models::point::Point;
use crate::models::screen_params::ScreenParams;
use eframe::epaint::{Color32, Shape};
use nalgebra::Matrix3;

pub const SYMMETRY_DOT_COLOR: Color32 = Color32::from_rgb(255, 0, 255);

pub struct Affine {
    pub xx: f32,
    pub xy: f32,
    pub yx: f32,
    pub yy: f32,
    pub zero_x: f32,
    pub zero_y: f32,

    pub scaling_x: f32,
    pub scaling_y: f32,

    pub symmetry_x: f32,
    pub symmetry_y: f32,
}

impl Default for Affine {
    fn default() -> Self {
        Self {
            xx: 1.0,
            xy: 0.0,
            yx: 0.0,
            yy: 1.0,
            zero_x: 0.0,
            zero_y: 0.0,

            scaling_x: 1.0,
            scaling_y: 1.0,

            symmetry_x: 0.0,
            symmetry_y: 0.0,
        }
    }
}

impl Affine {
    pub fn is_affine_default(&self) -> bool {
        self.xx == 1.0
            && self.xy == 0.0
            && self.yx == 0.0
            && self.yy == 1.0
            && self.zero_x == 0.0
            && self.zero_y == 0.0
    }

    pub fn affine_convert_line(&self, lines: Vec<Line>) -> Vec<Line> {
        if self.is_affine_default() {
            return lines;
        };

        lines
            .iter()
            .map(|line| {
                let start = self.affine_convert_point(line.start);
                let end = self.affine_convert_point(line.end);
                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    pub fn affine_convert_circle(&self, circle: Circle) -> Circle {
        if self.is_affine_default() {
            return circle;
        }

        let point_vector = circle.center.to_vector();
        let matrix = self.get_affine_matrix();

        let answer = point_vector * matrix;

        let mut radius = (self.xx + self.yy) / 2.0;
        if radius < 2.5 {
            radius = 2.5;
        }

        Circle {
            center: Point::new(answer.x, answer.y),
            radius,
        }
    }

    fn affine_convert_point(&self, point: Point) -> Point {
        let point_vector = point.to_vector();
        let matrix = self.get_affine_matrix();

        let answer = point_vector * matrix;

        Point::new(answer.x, answer.y)
    }

    pub fn scaling_convert_line(&self, lines: Vec<Line>) -> Vec<Line> {
        if self.scaling_x == 1.0 && self.scaling_y == 1.0 {
            return lines;
        };

        lines
            .iter()
            .map(|line| {
                let start = self.scale_convert_point(line.start);
                let end = self.scale_convert_point(line.end);
                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    pub fn scaling_convert_circle(&self, circle: Circle) -> Circle {
        if self.scaling_x == 1.0 && self.scaling_y == 1.0 {
            return circle;
        }

        let point_vector = circle.center.to_vector();
        let matrix = self.get_scale_matrix();

        let answer = point_vector * matrix;

        let mut radius = (self.scaling_x + self.scaling_y) / 2.0;
        if radius < 2.5 {
            radius = 2.5;
        }

        Circle {
            center: Point::new(answer.x, answer.y),
            radius,
        }
    }

    fn scale_convert_point(&self, point: Point) -> Point {
        let point_vector = point.to_vector();
        let matrix = self.get_scale_matrix();

        let answer = point_vector * matrix;

        Point::new(answer.x, answer.y)
    }

    pub fn symmetry_convert_line(&self, lines: Vec<Line>) -> Vec<Line> {
        if self.symmetry_x == 0.0 && self.symmetry_y == 0.0 {
            return lines;
        };

        lines
            .iter()
            .map(|line| {
                let start = self.symmetry_convert_point(line.start);
                let end = self.symmetry_convert_point(line.end);
                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    fn symmetry_convert_point(&self, point: Point) -> Point {
        let point_vector = point.to_vector();
        let matrix = self.get_symmetry_matrix();

        let answer = point_vector * matrix;

        Point::new(answer.x, answer.y)
    }

    fn get_affine_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
            self.xx,
            self.xy,
            0.0,
            self.yx,
            self.yy,
            0.0,
            self.zero_x,
            self.zero_y,
            1.0,
        )
    }

    fn get_scale_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
            self.scaling_x,
            0.0,
            0.0,
            0.0,
            self.scaling_y,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    fn get_symmetry_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
            -1.0,
            0.0,
            0.0,
            0.0,
            -1.0,
            0.0,
            2.0 * self.symmetry_x,
            2.0 * self.symmetry_y,
            1.0,
        )
    }

    pub fn symmetry_dot(&self) -> Circle {
        Circle {
            center: Point::new(self.symmetry_x, self.symmetry_y),
            ..Default::default()
        }
    }

    pub fn shape_symmetry_dot(
        circle: Circle, color: Color32, screen_params: ScreenParams,
    ) -> Shape {
        let screen_point = circle.to_screen_pos2(screen_params);

        let radius = screen_params.convert_single(circle.radius);

        if circle.center.x == 0.0 && circle.center.y == 0.0 {
            return Shape::circle_filled(screen_point, radius, Color32::from_white_alpha(0));
        }

        Shape::circle_filled(screen_point, radius, color)
    }
}
