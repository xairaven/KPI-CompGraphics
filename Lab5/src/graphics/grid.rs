use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use crate::graphics::screen::ScreenParams;
use crate::math::angle::Angle;
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

    pub angle_z_degrees: i32,

    pub p: f32,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            origin: Point3D::new(0.0, 0.0, 0.0),
            unit_x: Point3D::new(DEFAULT_UNIT_LENGTH, 0.0, 0.0),
            unit_y: Point3D::new(0.0, DEFAULT_UNIT_LENGTH, 0.0),
            unit_z: Point3D::new(0.0, 0.0, DEFAULT_UNIT_LENGTH),

            angle_z_degrees: 0,
            p: 2.0,

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

    pub fn angle_y(&self) -> Angle {
        let angle_z = self.angle_z();

        let radians = f32::asin(f32::sqrt(
            f32::sin(angle_z.radian()).powf(2.0) / f32::cos(angle_z.radian()).powf(2.0),
        ));

        Angle::from_radian(radians)
    }

    pub fn angle_z(&self) -> Angle {
        Angle::from_degree(self.angle_z_degrees as f32)
    }

    pub fn hide_bad_angles(&mut self) {
        if matches!(self.angle_z_degrees, 46..=90) {
            self.angle_z_degrees = 135;
        }
        if matches!(self.angle_z_degrees, 91..=134) {
            self.angle_z_degrees = 45;
        }
        if matches!(self.angle_z_degrees, 226..=270) {
            self.angle_z_degrees = 316;
        }
        if matches!(self.angle_z_degrees, 271..=315) {
            self.angle_z_degrees = 225;
        }
        if matches!(self.angle_z_degrees, -90..=-45) {
            self.angle_z_degrees = -135;
        }
        if matches!(self.angle_z_degrees, -134..=-91) {
            self.angle_z_degrees = -44;
        }
        if matches!(self.angle_z_degrees, -270..=-226) {
            self.angle_z_degrees = -316;
        }
        if matches!(self.angle_z_degrees, -315..=-271) {
            self.angle_z_degrees = -225;
        }
    }
}
