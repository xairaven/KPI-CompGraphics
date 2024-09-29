use crate::models::circle::Circle;
use crate::models::line::Line;
use crate::models::point::Point;
use nalgebra::Matrix3;

pub struct Projective {
    pub xx: f32,
    pub xy: f32,
    pub wx: f32,
    pub yx: f32,
    pub yy: f32,
    pub wy: f32,
    pub zero_x: f32,
    pub zero_y: f32,
    pub zero_w: f32,
}

impl Default for Projective {
    fn default() -> Self {
        Self {
            xx: 1.0,
            xy: 0.0,
            wx: 1.0,
            yx: 0.0,
            yy: 1.0,
            wy: 1.0,
            zero_x: 0.0,
            zero_y: 0.0,
            zero_w: 250.0,
        }
    }
}

impl Projective {
    pub fn is_projective_default(&self) -> bool {
        let default: Self = Projective::default();

        self.xx == default.xx
            && self.xy == default.xy
            && self.wx == default.wx
            && self.yx == default.yx
            && self.yy == default.yy
            && self.wy == default.wy
            && self.zero_x == default.zero_x
            && self.zero_y == default.zero_y
            && self.zero_w == default.zero_w
    }

    pub fn convert_line(&self, lines: Vec<Line>) -> Vec<Line> {
        if self.is_projective_default() {
            return lines;
        };

        lines
            .iter()
            .map(|line| {
                let start = self.convert_point(line.start);
                let end = self.convert_point(line.end);
                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    pub fn convert_circle(&self, circle: Circle) -> Circle {
        if self.is_projective_default() {
            return circle;
        }

        let point_vector = circle.center.to_vector();
        let matrix = self.get_matrix();

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

    fn convert_point(&self, point: Point) -> Point {
        let point_vector = point.to_vector();
        let matrix = self.get_matrix();

        let answer = point_vector * matrix;

        Point::new(answer.x, answer.y)
    }

    fn get_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(
            self.xx * self.wx / 100.0,
            self.xy * self.wx / 100.0,
            self.wx / 100.0,
            self.yx * self.wy / 100.0,
            self.yy * self.wy / 100.0,
            self.wy / 100.0,
            self.zero_x * self.zero_w / 100.0,
            self.zero_y * self.zero_w / 100.0,
            self.zero_w / 100.0,
        )
    }
}
