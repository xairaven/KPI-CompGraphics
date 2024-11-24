use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use crate::graphics::screen::ScreenParams;
use crate::ui::styles::strokes;
use eframe::epaint::Stroke;

// Star figure.
pub struct Model {
    pub radius: f32,
    pub thickness: f32,

    pub stroke: Stroke,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            stroke: strokes::model_black(0.05),

            radius: 5.0,
            thickness: 2.5,
        }
    }
}

impl Model {
    pub fn lines(&self, screen_params: ScreenParams) -> Vec<Line3D> {
        let mut stroke = self.stroke;
        stroke.width = screen_params.value_cm_to_px(self.stroke.width);

        let mut points: Vec<Point3D> = Vec::with_capacity(10);
        let mut upper_points: Vec<Point3D> = Vec::with_capacity(10);

        let mut lines: Vec<Line3D> = Vec::with_capacity(30);

        let radius = self.radius;
        let inner_radius = radius / 2.0;

        let initial_thickness = 0.0;
        let thickness = self.thickness;

        for k in 0..=4 {
            let angle = k as f32 * 2.0 * std::f32::consts::PI / 5.0 + std::f32::consts::PI / 2.0;
            let offset_angle = angle + 2.0 * std::f32::consts::PI / 10.0;

            let outer = Self::point(angle, radius, initial_thickness);
            let inner = Self::point(offset_angle, inner_radius, initial_thickness);

            let upper_outer = Self::point(angle, radius, thickness);
            let upper_inner = Self::point(offset_angle, inner_radius, thickness);

            points.push(outer);
            points.push(inner);
            upper_points.push(upper_outer);
            upper_points.push(upper_inner);

            // Add vertical lines directly
            lines.push(Line3D::new(outer, upper_outer, stroke));
            lines.push(Line3D::new(inner, upper_inner, stroke));
        }

        // Close the loops
        points.push(points[0]);
        upper_points.push(upper_points[0]);

        // Add horizontal lines
        points.windows(2).for_each(|pair| {
            lines.push(Line3D::new(pair[0], pair[1], stroke));
        });
        upper_points.windows(2).for_each(|pair| {
            lines.push(Line3D::new(pair[0], pair[1], stroke));
        });

        lines
    }

    // https://math.stackexchange.com/questions/3582342/coordinates-of-the-vertices-of-a-five-pointed-star
    fn point(angle: f32, radius: f32, thickness: f32) -> Point3D {
        let x = radius * f32::cos(angle);
        let y = radius * f32::sin(angle);
        Point3D::new(x, y, thickness)
    }

    pub fn pivot_point(&self, offset: Point3D) -> Point3D {
        Point3D::new(
            0.0 + offset.x,
            0.0 + offset.y,
            self.thickness / 2.0 + offset.z,
        )
    }
}
