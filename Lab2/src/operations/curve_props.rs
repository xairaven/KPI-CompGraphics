use crate::math::derivative::derivative;
use crate::models::point::Point;

pub const TANGENT_LINE_LENGTH: f32 = 5.0;
pub const NORMAL_LINE_LENGTH: f32 = TANGENT_LINE_LENGTH;
const RANDOM_X: f32 = 25.0;

#[derive(Default)]
pub struct CurveProperties {
    pub is_tangent_enabled: bool,
    pub is_normal_enabled: bool,
}

impl CurveProperties {
    pub fn tangent_point(x: f32, y: f32, a: f32, b: f32) -> Option<Point> {
        let derivative = derivative(x, y, a, b);

        let end = Point::new(RANDOM_X, y + derivative * (RANDOM_X - x));

        if end.y.is_finite() {
            Some(end)
        } else {
            None
        }
    }

    pub fn normal_point(x: f32, y: f32, a: f32, b: f32) -> Option<Point> {
        let derivative = derivative(x, y, a, b);

        let end = Point::new(RANDOM_X, y - 1.0 / derivative * (RANDOM_X - x));

        if end.y.is_finite() {
            Some(end)
        } else {
            None
        }
    }
}
