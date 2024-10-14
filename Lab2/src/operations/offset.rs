use crate::models::line::Line;
use crate::models::point::Point;

#[derive(Default)]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

impl Offset {
    pub fn process(&self, model_lines: Vec<Line>) -> Vec<Line> {
        if self.x == 0.0 && self.y == 0.0 {
            return model_lines;
        };

        model_lines
            .iter()
            .map(|line| {
                let start = self.point(line.start);
                let end = self.point(line.end);

                Line::new(start, end, line.stroke)
            })
            .collect()
    }

    pub fn point(&self, point: Point) -> Point {
        Point {
            x: point.x + self.x,
            y: point.y + self.y,

            converted_to_screen: false,
        }
    }
}
