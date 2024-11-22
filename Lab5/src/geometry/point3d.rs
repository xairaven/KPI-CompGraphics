use crate::geometry::point2d::Point2D;
use crate::projections::trimetric::TrimetricProjection;
use nalgebra::SMatrix;

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

    pub fn to_point2d(&self, trimetric: &TrimetricProjection) -> Point2D {
        let vector = self.to_vector() * trimetric.matrix();

        Point2D::new(vector.x, vector.y)
    }
}
