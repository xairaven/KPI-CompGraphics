use crate::geometry::line::Line;
use crate::geometry::moveable_point::MoveablePoint;
use crate::traits::positionable::Positionable;

pub struct Offset {
    pub is_enabled: bool,
    pub dot: MoveablePoint,
}

impl Default for Offset {
    fn default() -> Self {
        Self {
            is_enabled: false,
            dot: MoveablePoint::new(0.0, 0.0),
        }
    }
}

impl Offset {
    pub fn process<T: Positionable + Clone + Copy>(
        &self, model_lines: Vec<Line<T>>,
    ) -> Vec<Line<T>> {
        if self.dot.x == 0.0 && self.dot.y == 0.0 {
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
        T::new(point.x() + self.dot.x, point.y() + self.dot.y)
    }
}
