use crate::models::line::Line;
use crate::models::point::Point;
use crate::ui::styles::strokes;
use std::collections::VecDeque;

pub const X_BOUND: f32 = 20.0;
pub const X_STEP: f32 = 0.01;

pub struct Model {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            a: 1.5,
            b: 1.0,
            c: 0.5,
        }
    }
}

impl Model {
    pub fn lines(&self) -> Vec<Line> {
        let (outer_points, inner_points) = self.points();

        let stroke = strokes::model_black();

        let mut outer_lines: Vec<Line> = outer_points
            .windows(2)
            .map(|pair| Line::new(pair[0], pair[1], stroke))
            .collect();

        let mut inner_lines: Vec<Line> = inner_points
            .windows(2)
            .map(|pair| Line::new(pair[0], pair[1], stroke))
            .collect();

        // Connecting
        if let (Some(first), Some(last)) = (outer_points.first(), outer_points.last()) {
            outer_lines.push(Line::new(*first, *last, stroke));
        }
        if let (Some(first), Some(last)) = (inner_points.first(), inner_points.last()) {
            inner_lines.push(Line::new(*first, *last, stroke));
        }

        outer_lines.append(&mut inner_lines);

        outer_lines
    }

    pub fn points(&self) -> (Vec<Point>, Vec<Point>) {
        let mut outer_circle: VecDeque<Point> = VecDeque::new();
        let mut inner_circle: VecDeque<Point> = VecDeque::new();

        let mut x = -X_BOUND;
        while x < X_BOUND {
            if let Some(point) = self.get_point(Model::outer_circle_formula, x) {
                outer_circle.push_back(point);
                outer_circle.push_front(Point::new(point.x, -point.y));
            }

            if let Some(point) = self.get_point(Model::inner_circle_formula, x) {
                inner_circle.push_back(point);
                inner_circle.push_front(Point::new(point.x, -point.y));
            }

            x += X_STEP;
        }

        (Vec::from(outer_circle), Vec::from(inner_circle))
    }

    pub fn outer_circle_formula(x: f32, a: f32, b: f32, c: f32) -> f32 {
        f32::sqrt(
            f32::sqrt(8.0 * a * b.powf(2.0) * x + b.powf(4.0) + 4.0 * c)
                + 4.0 * a * x
                + b.powf(2.0)
                - 2.0 * x.powf(2.0),
        ) / f32::sqrt(2.0)
    }

    pub fn inner_circle_formula(x: f32, a: f32, b: f32, c: f32) -> f32 {
        f32::sqrt(
            -f32::sqrt(8.0 * a * b.powf(2.0) * x + b.powf(4.0) + 4.0 * c)
                + 4.0 * a * x
                + b.powf(2.0)
                - 2.0 * x.powf(2.0),
        ) / f32::sqrt(2.0)
    }

    pub fn get_point<F>(&self, function: F, x: f32) -> Option<Point>
    where
        F: Fn(f32, f32, f32, f32) -> f32,
    {
        let y = function(x, self.a, self.b, self.c);

        if y.is_finite() {
            Some(Point::new(x, y))
        } else {
            None
        }
    }
}
