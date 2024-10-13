use crate::math::derivative::derivative;
use crate::models::point::Point;

#[derive(Default)]
pub struct CurveProperties {
    pub is_tangent_enabled: bool,
    pub is_normal_enabled: bool,
}

impl CurveProperties {
    pub fn tangent_point(x: f32, y: f32, a: f32, b: f32) -> Option<Point> {
        let derivative = derivative(x, y, a, b);

        let random_x = 25.0;
        let end = Point::new(random_x, y + derivative * (random_x - x));

        if end.y.is_finite() {
            Some(end)
        } else {
            None
        }
    }
}
