use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use crate::graphics::screen::ScreenParams;
use crate::ui::styles::strokes;
use eframe::epaint::Stroke;

pub const DEFAULT_UNIT_LENGTH: f32 = 1.0;

pub struct Grid {
    pub origin: Point3D,

    pub unit_x: Point3D,
    pub unit_y: Point3D,
    pub unit_z: Point3D,

    pub axis_x_stroke: Stroke,
    pub axis_y_stroke: Stroke,
    pub axis_z_stroke: Stroke,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            origin: Point3D::new(0.0, 0.0, 0.0),
            unit_x: Point3D::new(DEFAULT_UNIT_LENGTH, 0.0, 0.0),
            unit_y: Point3D::new(0.0, DEFAULT_UNIT_LENGTH, 0.0),
            unit_z: Point3D::new(0.0, 0.0, DEFAULT_UNIT_LENGTH),

            axis_x_stroke: strokes::axis_red(),
            axis_y_stroke: strokes::axis_green(),
            axis_z_stroke: strokes::axis_blue(),
        }
    }
}

impl Grid {
    pub fn axes_lines(&mut self, screen_params: ScreenParams) -> Vec<Line3D> {
        self.unit_x = Point3D::new(screen_params.grid_unit_length, 0.0, 0.0);
        self.unit_y = Point3D::new(0.0, screen_params.grid_unit_length, 0.0);
        self.unit_z = Point3D::new(0.0, 0.0, screen_params.grid_unit_length);

        let axis_x = Line3D::new(self.origin, self.unit_x, self.axis_x_stroke);
        let axis_y = Line3D::new(self.origin, self.unit_y, self.axis_y_stroke);
        let axis_z = Line3D::new(self.origin, self.unit_z, self.axis_z_stroke);

        vec![axis_x, axis_y, axis_z]
    }
}
