use crate::geometry::line2d::Line2D;
use crate::geometry::point3d::Point3D;
use crate::math::angle::Angle;
use eframe::epaint::Stroke;

#[derive(Debug, Default, Clone, Copy)]
pub struct Line3D {
    pub start: Point3D,
    pub end: Point3D,

    pub stroke: Stroke,
}

impl Line3D {
    pub fn new(start: Point3D, end: Point3D, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }

    pub fn to_line2d(&self, angle_y: Angle, angle_z: Angle, p: f32) -> Line2D {
        let start = self.start.to_point2d(angle_y, angle_z, p);
        let end = self.end.to_point2d(angle_y, angle_z, p);

        Line2D::new(start, end, self.stroke)
    }
}
