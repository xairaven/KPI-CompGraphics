use crate::geometry::line::Line;
use crate::geometry::moveable_point::MoveablePoint;
use crate::math::angle::Angle;
use crate::traits::positionable::Positionable;
use nalgebra::Matrix3;

#[derive(Default)]
pub struct Rotation {
    pub is_enabled: bool,

    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Rotation {
    pub fn process<T: Positionable + Clone + Copy>(
        &self, model_lines: Vec<Line<T>>,
    ) -> Vec<Line<T>> {
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

    pub fn point<T: Positionable + Clone + Copy>(&self, point: T) -> T {
        let point_vector = point.to_vector();
        let matrix = self.matrix::<T>();

        let answer = point_vector * matrix;

        T::new(answer.x, answer.y)
    }

    fn matrix<T: Positionable + Clone + Copy>(&self) -> Matrix3<f32> {
        let angle = Angle::from_degree(self.angle).radian();
        let point = T::new(self.x, self.y);

        let m11 = f32::cos(angle);
        let m12 = f32::sin(angle);
        let m21 = -1.0 * f32::sin(angle);
        let m22 = f32::cos(angle);
        let m31 = -1.0 * point.x() * (f32::cos(angle) - 1.0) + point.y() * f32::sin(angle);
        let m32 = -1.0 * point.x() * f32::sin(angle) - point.y() * (f32::cos(angle) - 1.0);

        Matrix3::new(m11, m12, 0.0, m21, m22, 0.0, m31, m32, 1.0)
    }

    pub fn dot(&self) -> MoveablePoint {
        MoveablePoint::new(self.x, self.y)
    }
}
