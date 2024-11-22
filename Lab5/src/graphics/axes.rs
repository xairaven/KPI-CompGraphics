use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use crate::graphics::screen::ScreenParams;
use crate::ui::styles::strokes;
use eframe::epaint::Stroke;

pub const DEFAULT_UNIT_LENGTH: f32 = 1.0;

pub struct Axes {
    pub origin: Point3D,

    pub unit_x: Point3D,
    pub unit_y: Point3D,
    pub unit_z: Point3D,

    pub stroke_x: Stroke,
    pub stroke_y: Stroke,
    pub stroke_z: Stroke,
}

impl Default for Axes {
    fn default() -> Self {
        Self {
            origin: Point3D::new(0.0, 0.0, 0.0),
            unit_x: Point3D::new(DEFAULT_UNIT_LENGTH, 0.0, 0.0),
            unit_y: Point3D::new(0.0, DEFAULT_UNIT_LENGTH, 0.0),
            unit_z: Point3D::new(0.0, 0.0, DEFAULT_UNIT_LENGTH),

            stroke_x: strokes::axis_red(),
            stroke_y: strokes::axis_green(),
            stroke_z: strokes::axis_blue(),
        }
    }
}

impl Axes {
    pub fn lines(&mut self, screen: ScreenParams) -> Vec<Line3D> {
        self.unit_x = Point3D::new(screen.unit_length, 0.0, 0.0);
        self.unit_y = Point3D::new(0.0, screen.unit_length, 0.0);
        self.unit_z = Point3D::new(0.0, 0.0, screen.unit_length);

        let axis_x = Line3D::new(self.origin, self.unit_x, self.stroke_x);
        let axis_y = Line3D::new(self.origin, self.unit_y, self.stroke_y);
        let axis_z = Line3D::new(self.origin, self.unit_z, self.stroke_z);

        vec![axis_x, axis_y, axis_z]
    }
}
