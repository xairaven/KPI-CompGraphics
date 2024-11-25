use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use crate::graphics::screen::ScreenParams;
use crate::math::angle::Angle;
use crate::ui::styles::strokes;
use egui::{Color32, Stroke};
use std::f32::consts::PI;

pub struct Surface {
    pub display_radius: f32,
    pub display_height: f32,
    pub display_mesh: f32,

    lines: Vec<Line3D>,
    height: f32,
    radius: f32,
    mesh: f32,

    pub color: Color32,
    stroke: Stroke,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            display_radius: 10.0,
            display_height: 35.0,
            display_mesh: 5.0,

            lines: Vec::new(),
            height: 0.0,
            radius: 0.0,
            mesh: 0.0,

            color: Color32::BLACK,
            stroke: strokes::surface_black(0.05),
        }
    }
}

impl Surface {
    pub fn generate(&mut self, screen: ScreenParams) -> Vec<Line3D> {
        if self.radius != self.display_radius
            || self.height != self.display_height
            || self.mesh != self.display_mesh
        {
            self.radius = self.display_radius;
            self.height = self.display_height;
            self.mesh = self.display_mesh;
            self.lines = self.lines(screen);
        }

        self.sync_stroke_color();
        self.lines.clone()
    }

    fn lines(&self, screen: ScreenParams) -> Vec<Line3D> {
        let mut stroke = self.stroke;
        stroke.width = screen.value_cm_to_px(self.stroke.width);

        let mut lines: Vec<Line3D> = Vec::new();

        let surface_container = self.surface_area();

        for level in &surface_container {
            level.windows(2).for_each(|pair| {
                let line = Line3D::new(pair[0], pair[1], stroke);
                lines.push(line);
            })
        }

        surface_container.windows(2).for_each(|pair| {
            debug_assert!(pair[0].len() == pair[1].len());

            let length = pair[0].len();
            for counter in 0..length {
                let line = Line3D::new(pair[0][counter], pair[1][counter], stroke);
                lines.push(line);
            }
        });

        lines
    }

    fn surface_area(&self) -> Vec<Vec<Point3D>> {
        let mut container: Vec<Vec<Point3D>> = Vec::new();

        let mut u = -self.height;
        while u <= self.height {
            let mut vector: Vec<Point3D> = Vec::new();

            let mut v = 0.0;
            while v <= 360.0 {
                let point = Self::point(self.radius, u, v);
                vector.push(point);

                v += self.mesh;
            }
            container.push(vector);

            u += self.mesh;
        }

        container
    }

    pub fn point(radius: f32, u: f32, v: f32) -> Point3D {
        let angle = Angle::from_degree(2.0 * u * PI).radian();
        let v = Angle::from_degree(v).radian();

        let x = radius * (1.0 + f32::abs(f32::sin(angle))) * f32::cos(v);
        let y = u;
        let z = radius * (1.0 + f32::abs(f32::sin(angle))) * f32::sin(v);

        Point3D::new(x, y, z)
    }

    pub fn apply_default_settings(&mut self) {
        self.display_radius = 10.0;
        self.display_height = 35.0;
        self.display_mesh = 5.0;
        self.color = Color32::BLACK;
    }

    fn sync_stroke_color(&mut self) {
        self.stroke.color = self.color;
    }
}
