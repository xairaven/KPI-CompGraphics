use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use crate::ui::styles::{colors, strokes};
use eframe::epaint::Stroke;
use egui::Color32;

pub const DEFAULT_AXIS_LENGTH: u32 = 1;
pub const DEFAULT_UNIT_LENGTH: f32 = 1.0;

pub struct Axes {
    pub are_enabled: bool,

    pub origin: Point3D,

    pub unit_x: Point3D,
    pub unit_y: Point3D,
    pub unit_z: Point3D,

    pub axis_length: u32,

    pub color_x: Color32,
    pub color_y: Color32,
    pub color_z: Color32,
    stroke_x: Stroke,
    stroke_y: Stroke,
    stroke_z: Stroke,
}

impl Default for Axes {
    fn default() -> Self {
        Self {
            are_enabled: false,

            origin: Point3D::new(0.0, 0.0, 0.0),
            unit_x: Point3D::new(DEFAULT_UNIT_LENGTH, 0.0, 0.0),
            unit_y: Point3D::new(0.0, DEFAULT_UNIT_LENGTH, 0.0),
            unit_z: Point3D::new(0.0, 0.0, DEFAULT_UNIT_LENGTH),

            axis_length: DEFAULT_AXIS_LENGTH,

            color_x: colors::RED,
            color_y: colors::LIME,
            color_z: colors::BLUE,
            stroke_x: strokes::axis_red(),
            stroke_y: strokes::axis_lime(),
            stroke_z: strokes::axis_blue(),
        }
    }
}

impl Axes {
    pub fn lines(&mut self) -> Vec<Line3D> {
        self.sync_stroke_colors();

        self.unit_x = Point3D::new(self.axis_length as f32, 0.0, 0.0);
        self.unit_y = Point3D::new(0.0, self.axis_length as f32, 0.0);
        self.unit_z = Point3D::new(0.0, 0.0, self.axis_length as f32);

        let axis_x = Line3D::new(self.origin, self.unit_x, self.stroke_x);
        let axis_y = Line3D::new(self.origin, self.unit_y, self.stroke_y);
        let axis_z = Line3D::new(self.origin, self.unit_z, self.stroke_z);

        vec![axis_x, axis_y, axis_z]
    }

    fn sync_stroke_colors(&mut self) {
        self.stroke_x.color = self.color_x;
        self.stroke_y.color = self.color_y;
        self.stroke_z.color = self.color_z;
    }
}
