use crate::math::derivative::derivative;
use crate::math::derivative::second_derivative;
use crate::models::line::Line;
use crate::models::point::Point;
use rust_decimal::Decimal;

pub const TANGENT_LINE_LENGTH: f32 = 5.0;
pub const NORMAL_LINE_LENGTH: f32 = TANGENT_LINE_LENGTH;
const RANDOM_X: f32 = 25.0;

#[derive(Default)]
pub struct CurveProperties {
    pub is_tangent_enabled: bool,
    pub is_normal_enabled: bool,

    pub is_inflection_enabled: bool,

    pub length: f32,
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

    pub fn length(&mut self, model_lines: &[Line]) {
        let sum: f32 = model_lines.iter().map(|line| line.length()).sum();

        self.length = sum;
    }

    pub fn inflection_points(lines: &[Line], a: f32, b: f32) -> Vec<Point> {
        let points: Vec<Point> = lines.iter().map(|line| line.start).collect();

        let comparer = Decimal::from_f32_retain(0.01).unwrap();

        let mut results: Vec<Point> = vec![];
        for point in points {
            let derivative = second_derivative(point.x, point.y, a, b);
            let derivative = Decimal::from_f32_retain(derivative);
            if let Some(value) = derivative {
                if Decimal::abs(&value) <= comparer {
                    results.push(Point::new(point.x, point.y));
                }
            }
        }

        results
    }
}
