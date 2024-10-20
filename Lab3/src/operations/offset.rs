use crate::geometry::line::Line;
use crate::geometry::moveable_point::MoveablePoint;
use crate::traits::positionable::Positionable;

#[derive(Default)]
pub struct Offset {
    pub is_enabled: bool,

    pub x: f32,
    pub y: f32,
}

impl Offset {
    pub fn process<T: Positionable + Clone + Copy>(
        &self, model_lines: Vec<Line<T>>,
    ) -> Vec<Line<T>> {
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

    pub fn point<T: Positionable + Clone + Copy>(&self, point: T) -> T {
        T::new(point.x() + self.x, point.y() + self.y)
    }

    pub fn dot(&self) -> MoveablePoint {
        MoveablePoint::new(self.x, self.y)
    }
}
