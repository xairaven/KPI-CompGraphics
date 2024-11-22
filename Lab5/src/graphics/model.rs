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
            stroke: strokes::model_black(0.1),

            radius: 10.0,
            thickness: 5.0,
        }
    }
}

impl Model {
    pub fn lines(&self, screen_params: ScreenParams) -> Vec<Line3D> {
        let mut stroke = self.stroke;
        stroke.width = screen_params.value_cm_to_px(self.stroke.width);

        let mut points: Vec<Point3D> = vec![];
        let mut upper_points: Vec<Point3D> = vec![];

        for k in 0..=4 {
            points.push(Self::formula_outer(k, self.radius, 0.0));
            points.push(Self::formula_inner(k, self.radius / 2.0, 0.0));
            upper_points.push(Self::formula_outer(k, self.radius, self.thickness));
            upper_points.push(Self::formula_inner(k, self.radius / 2.0, self.thickness));
        }

        let mut lines: Vec<Line3D> = vec![];
        for (index, point) in points.iter().enumerate() {
            let line = Line3D::new(*point, upper_points[index], stroke);
            lines.push(line);
        }

        points.push(points[0]);
        upper_points.push(upper_points[0]);
        points.windows(2).for_each(|points| {
            let line = Line3D::new(points[0], points[1], stroke);
            lines.push(line);
        });
        upper_points.windows(2).for_each(|points| {
            let line = Line3D::new(points[0], points[1], stroke);
            lines.push(line);
        });

        lines
    }

    // https://math.stackexchange.com/questions/3582342/coordinates-of-the-vertices-of-a-five-pointed-star
    fn formula_outer(k: u32, r: f32, z: f32) -> Point3D {
        let x =
            r * f32::cos(k as f32 * 2.0 * std::f32::consts::PI / 5.0 + std::f32::consts::PI / 2.0);
        let y =
            r * f32::sin(k as f32 * 2.0 * std::f32::consts::PI / 5.0 + std::f32::consts::PI / 2.0);
        Point3D::new(x, y, z)
    }

    fn formula_inner(k: u32, r: f32, z: f32) -> Point3D {
        let x = r * f32::cos(
            k as f32 * 2.0 * std::f32::consts::PI / 5.0
                + std::f32::consts::PI / 2.0
                + 2.0 * std::f32::consts::PI / 10.0,
        );
        let y = r * f32::sin(
            k as f32 * 2.0 * std::f32::consts::PI / 5.0
                + std::f32::consts::PI / 2.0
                + 2.0 * std::f32::consts::PI / 10.0,
        );
        Point3D::new(x, y, z)
    }
}
