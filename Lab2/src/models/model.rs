use crate::models::line::Line;
use crate::models::point::Point;
use crate::ui::styles::strokes;
use eframe::epaint::Stroke;

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
        let points = self.points();

        let stroke = strokes::model_black();

        let mut lines: Vec<Line> = vec![];

        let mut outer_upper_lines = points[0]
            .windows(2)
            .map(|pair| Line::new(pair[0], pair[1], stroke))
            .collect();
        let mut outer_lower_lines = points[1]
            .windows(2)
            .map(|pair| Line::new(pair[0], pair[1], stroke))
            .collect();
        let mut inner_upper_lines = points[2]
            .windows(2)
            .map(|pair| Self::divide_nan_lines(pair, stroke))
            .collect();
        let mut inner_lower_lines = points[3]
            .windows(2)
            .map(|pair| Self::divide_nan_lines(pair, stroke))
            .collect();

        // EDGE CASES
        // if (a > 0)

        // APPENDS
        lines.append(&mut outer_upper_lines);
        lines.append(&mut outer_lower_lines);
        lines.append(&mut inner_upper_lines);
        lines.append(&mut inner_lower_lines);

        lines
    }

    pub fn points(&self) -> Vec<Vec<Point>> {
        let mut outer_upper: Vec<Point> = vec![];
        let mut outer_lower: Vec<Point> = vec![];
        let mut inner_upper: Vec<Point> = vec![];
        let mut inner_lower: Vec<Point> = vec![];

        let mut x = -X_BOUND;
        while x < X_BOUND {
            if let Some(point) = self.get_point(Model::outer_circle_formula, x) {
                outer_upper.push(point);
                outer_lower.push(Point::new(point.x, -point.y));
            }

            if let Some(point) = self.get_point(Model::inner_circle_formula, x) {
                inner_upper.push(point);
                inner_lower.push(Point::new(point.x, -point.y));
            }

            x += X_STEP;
        }

        vec![outer_upper, outer_lower, inner_upper, inner_lower]
    }

    fn divide_nan_lines(pair: &[Point], stroke: Stroke) -> Line {
        let mut line = Line::transparent(pair[0], pair[1]);

        if f32::abs(line.start.x - line.end.x) > X_STEP + (X_STEP / 10.0) {
            return line;
        }

        line.stroke = stroke;
        line
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
