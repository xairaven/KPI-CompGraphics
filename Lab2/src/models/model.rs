use crate::models::line::Line;
use crate::models::point::Point;
use crate::ui::styles::strokes;
use eframe::epaint::Stroke;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::collections::HashMap;

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
        let (outer_upper, outer_lower, inner_upper, inner_lower) = self.points();

        let stroke = strokes::model_black();

        let mut lines: Vec<Line> = vec![];

        let mut outer_upper_lines: Vec<Line> = outer_upper
            .windows(2)
            .map(|pair| Line::new(pair[0], pair[1], stroke))
            .collect();
        let mut outer_lower_lines: Vec<Line> = outer_lower
            .windows(2)
            .map(|pair| Line::new(pair[0], pair[1], stroke))
            .collect();
        let mut inner_upper_lines: Vec<Line> = inner_upper
            .windows(2)
            .map(|pair| Self::divide_nan_lines(pair, stroke))
            .collect();
        let mut inner_lower_lines: Vec<Line> = inner_lower
            .windows(2)
            .map(|pair| Self::divide_nan_lines(pair, stroke))
            .collect();

        // Connections
        let connection_elements_option = vec![
            outer_upper.first(),
            outer_upper.last(),
            outer_lower.first(),
            outer_lower.last(),
            inner_upper.first(),
            inner_upper.last(),
            inner_lower.first(),
            inner_lower.last(),
        ];
        let mut connection_elements = vec![];
        for point in connection_elements_option.into_iter().flatten() {
            connection_elements.push(*point);
        }
        // Transparent Lines
        let transparent_upper = inner_upper_lines
            .iter()
            .enumerate()
            .find(|line| line.1.is_transparent());
        let transparent_lower = inner_lower_lines
            .iter()
            .enumerate()
            .find(|line| line.1.is_transparent());
        if let (Some(upper_line), Some(lower_line)) = (transparent_upper, transparent_lower) {
            let mut points = vec![
                upper_line.1.start,
                upper_line.1.end,
                lower_line.1.start,
                lower_line.1.end,
            ];

            // Deleting transparent lines
            inner_upper_lines.remove(upper_line.0);
            inner_lower_lines.remove(lower_line.0);
            connection_elements.append(&mut points);
        }

        let mut connection_lines = Self::connect_edge_points(connection_elements);

        lines.append(&mut outer_upper_lines);
        lines.append(&mut outer_lower_lines);
        lines.append(&mut inner_upper_lines);
        lines.append(&mut inner_lower_lines);
        lines.append(&mut connection_lines);

        lines
    }

    pub fn points(&self) -> (Vec<Point>, Vec<Point>, Vec<Point>, Vec<Point>) {
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

        (outer_upper, outer_lower, inner_upper, inner_lower)
    }

    fn connect_edge_points(points: Vec<Point>) -> Vec<Line> {
        let mut map: HashMap<Decimal, Vec<f32>> = HashMap::default();
        for point in points {
            map.entry(Decimal::from_f32_retain(point.x).unwrap())
                .or_default()
                .push(point.y);
        }

        let mut lines: Vec<Line> = vec![];
        for (x, y_values) in map {
            let mut values = y_values.clone();
            values.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

            let x = x.to_f32().unwrap();

            if values.len() % 2 == 0 && !values.is_empty() {
                for i in (0..values.len()).step_by(2) {
                    lines.push(Line::new(
                        Point::new(x, values[i]),
                        Point::new(x, values[i + 1]),
                        strokes::model_black(),
                    ));
                }
            }
        }

        lines
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
