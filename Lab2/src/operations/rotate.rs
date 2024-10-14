use crate::math::angle::Angle;
use crate::models::dot::Dot;
use crate::models::line::Line;
use crate::models::point::Point;
use nalgebra::Matrix3;

#[derive(Default)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Rotation {
    pub fn process(&self, model_lines: Vec<Line>) -> Vec<Line> {
        if self.angle == 0.0 {
            return model_lines;
        };

        model_lines
            .iter()
            .map(|line| {
                let start = self.point(line.start);
                let end = self.point(line.end);

                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    pub fn point(&self, point: Point) -> Point {
        let point_vector = point.to_vector();
        let matrix = self.matrix();

        let answer = point_vector * matrix;

        Point::new(answer.x, answer.y)
    }

    fn matrix(&self) -> Matrix3<f32> {
        let angle = Angle::from_degree(self.angle).radian();
        let point = Point::new(self.x, self.y);

        let m11 = f32::cos(angle);
        let m12 = f32::sin(angle);
        let m21 = -1.0 * f32::sin(angle);
        let m22 = f32::cos(angle);
        let m31 = -1.0 * point.x * (f32::cos(angle) - 1.0) + point.y * f32::sin(angle);
        let m32 = -1.0 * point.x * f32::sin(angle) - point.y * (f32::cos(angle) - 1.0);

        Matrix3::new(m11, m12, 0.0, m21, m22, 0.0, m31, m32, 1.0)
    }

    pub fn dot(&self) -> Dot {
        Dot::from_point(&Point::new(self.x, self.y))
    }
}
