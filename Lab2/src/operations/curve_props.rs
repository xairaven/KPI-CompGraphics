use crate::models::line::Line;
use crate::models::point::Point;
use crate::ui::styles::strokes;

#[derive(Default)]
pub struct CurveProperties {
    pub is_tangent_enabled: bool,
    pub is_normal_enabled: bool,
}

impl CurveProperties {
    pub fn tangent_line(x: f32, y: f32, a: f32, b: f32) -> Option<Line> {
        let start = Point::new(x, y);
        let end = Self::tangent_end(x, y, a, b);

        end.map(|end| Line::new(start, end, strokes::derivative_blue()))
    }

    fn tangent_end(x: f32, y: f32, a: f32, b: f32) -> Option<Point> {
        let derivative = Self::derivative(x, y, a, b);

        let random_x = 25.0;
        let end = Point::new(random_x, y + derivative * (random_x - x));

        if end.y.is_finite() {
            Some(end)
        } else {
            None
        }
    }

    fn derivative(x: f32, y: f32, a: f32, b: f32) -> f32 {
        -1.0 * (4.0 * a.powf(2.0) * x
            - 6.0 * a * x.powf(2.0)
            - 2.0 * a * y.powf(2.0)
            - b.powf(2.0) * x
            + 2.0 * x.powf(3.0)
            + 2.0 * x * y.powf(2.0))
            / (y * (-4.0 * a * x - b.powf(2.0) + 2.0 * x.powf(2.0) + 2.0 * y.powf(2.0)))
    }
}
