use crate::geometry::moveable_point::MoveablePoint;
use crate::math::angle::Angle;
use crate::models::model::Model;
use crate::traits::positionable::Positionable;
use nalgebra::Matrix3;

pub struct Rotation {
    pub is_enabled: bool,

    pub dot: MoveablePoint,
    pub angle: f32,

    pub old: (f32, f32, f32),
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            is_enabled: false,

            dot: MoveablePoint::new(0.0, 0.0),
            angle: 0.0,

            old: (0.0, 0.0, 0.0),
        }
    }
}

impl Rotation {
    pub fn process(&mut self, model: &mut Model) {
        if self.angle == 0.0 {
            return;
        };
        if self.dot.x == self.old.0 && self.dot.y == self.old.1 && self.angle == self.old.2 {
            return;
        }

        let delta = self.angle - self.old.2;
        model.points.iter_mut().for_each(|bezier| {
            self.update_point(&mut bezier.point, delta);
        });

        self.old = (self.dot.x, self.dot.y, self.angle);
    }

    pub fn update_point(&self, point: &mut MoveablePoint, angle: f32) {
        let point_vector = point.to_vector();
        let matrix = self.matrix(angle);

        let answer = point_vector * matrix;

        point.x = answer.x;
        point.y = answer.y;
    }

    fn matrix(&self, angle: f32) -> Matrix3<f32> {
        let angle = Angle::from_degree(angle).radian();

        let m11 = f32::cos(angle);
        let m12 = f32::sin(angle);
        let m21 = -1.0 * f32::sin(angle);
        let m22 = f32::cos(angle);
        let m31 = -1.0 * self.dot.x * (f32::cos(angle) - 1.0) + self.dot.y * f32::sin(angle);
        let m32 = -1.0 * self.dot.x * f32::sin(angle) - self.dot.y * (f32::cos(angle) - 1.0);

        Matrix3::new(m11, m12, 0.0, m21, m22, 0.0, m31, m32, 1.0)
    }
}
