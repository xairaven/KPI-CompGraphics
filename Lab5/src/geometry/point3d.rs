use crate::geometry::point2d::Point2D;
use crate::math::angle::Angle;
use nalgebra::{Matrix4, SMatrix};

#[derive(Debug, Default, Clone, Copy)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_vector(&self) -> SMatrix<f32, 1, 4> {
        SMatrix::<f32, 1, 4>::new(self.x, self.y, self.z, 1.0)
    }

    pub fn to_point2d(&self, angle_y: Angle, angle_z: Angle, p: f32) -> Point2D {
        let vector = self.to_vector() * self.matrix(angle_y, angle_z, p);

        Point2D::new(vector.y, vector.z)
    }

    pub fn matrix(&self, angle_y: Angle, angle_z: Angle, p: f32) -> Matrix4<f32> {
        Matrix4::new(
            0.0,
            f32::cos(angle_y.radian()) * f32::sin(angle_z.radian()),
            -f32::sin(angle_y.radian()),
            0.0,
            0.0,
            f32::cos(angle_z.radian()),
            0.0,
            0.0,
            0.0,
            f32::sin(angle_y.radian()) * f32::sin(angle_z.radian()),
            f32::cos(angle_y.radian()),
            0.0,
            p,
            0.0,
            0.0,
            1.0,
        )
    }
}
