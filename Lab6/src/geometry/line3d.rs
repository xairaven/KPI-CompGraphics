use crate::geometry::line2d::Line2D;
use crate::geometry::point3d::Point3D;
use crate::projections::trimetric::TrimetricProjection;
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

    pub fn to_line2d(&self, trimetric: &TrimetricProjection) -> Line2D {
        let start = self.start.to_point2d(trimetric);
        let end = self.end.to_point2d(trimetric);

        Line2D::new(start, end, self.stroke)
    }
}
