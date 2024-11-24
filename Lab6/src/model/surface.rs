use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use crate::graphics::screen::ScreenParams;
use crate::math::angle::Angle;
use crate::ui::styles::strokes;
use egui::Stroke;
use std::f32::consts::PI;

pub struct Surface {
    pub radius_min: f32,
    pub radius_max: f32,

    pub stroke: Stroke,
}

impl Default for Surface {
    fn default() -> Self {
        Self {
            radius_min: 2.0,
            radius_max: 10.0,

            stroke: strokes::surface_black(0.05),
        }
    }
}

impl Surface {
    pub fn lines(&self, screen: ScreenParams) -> Vec<Line3D> {
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

        let mesh = 10.0;
        let height: f32 = 30.0;

        let radius = self.radius_max;

        let mut u = 0.0;
        while u < height {
            let mut vector: Vec<Point3D> = Vec::new();

            let mut v = 0.0;
            while v < 360.0 {
                let point = self.point(radius, u, Angle::from_degree(v).radian());
                vector.push(point);

                v += mesh;
            }
            vector.push(vector[0]);
            container.push(vector);

            u += mesh;
        }

        container
    }

    fn point(&self, radius: f32, u: f32, v: f32) -> Point3D {
        let x = radius * (1.0 + f32::abs(f32::sin(2.0 * u * PI))) * f32::cos(v);
        let y = u;
        let z = radius * (1.0 + f32::abs(f32::sin(2.0 * u * PI))) * f32::sin(v);

        Point3D::new(x, y, z)
    }
}
