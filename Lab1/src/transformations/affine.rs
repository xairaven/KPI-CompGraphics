use crate::models::circle::Circle;
use crate::models::line::Line;
use crate::models::point::Point;
use nalgebra::Matrix3;

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

            scaling_x: 0.0,
            scaling_y: 0.0,
            symmetry_x: 0.0,
            symmetry_y: 0.0,
        }
    }
}

impl Affine {
    pub fn convert_line(&self, lines: Vec<Line>) -> Vec<Line> {
        if self.xx == 1.0
            && self.xy == 0.0
            && self.yx == 0.0
            && self.yy == 1.0
            && self.zero_x == 0.0
            && self.zero_y == 0.0
        {
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
        let point_vector = circle.center.to_vector();
        let matrix = self.get_affine_matrix();

        let answer = point_vector * matrix;

        Circle {
            center: Point::new(answer.x, answer.y),
            radius: (self.xx + self.yy) / 2.0,
        }
    }

    fn convert_point(&self, point: Point) -> Point {
        let point_vector = point.to_vector();
        let matrix = self.get_affine_matrix();

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
}
