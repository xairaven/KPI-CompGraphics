use crate::models::line::Line;
use crate::models::point::Point;
use crate::ui::styles::strokes;

pub const UNIT_LENGTH: f32 = 1.0;

pub struct Grid {
    pub ticks_amount: i32,

    pub origin: Point,
    pub unit_x: Point,
    pub unit_y: Point,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            ticks_amount: 20,

            origin: Point::new(0.0, 0.0),
            unit_x: Point::new(UNIT_LENGTH, 0.0),
            unit_y: Point::new(0.0, UNIT_LENGTH),
        }
    }
}

impl Grid {
    pub fn lines(&self) -> Vec<Line> {
        let axis_x = Line {
            start: Point::new(self.unit_x.x * -(self.ticks_amount as f32), self.unit_x.y),
            end: Point::new(self.unit_x.x * (self.ticks_amount as f32), self.unit_x.y),
            stroke: strokes::axis_red(),
        };

        let axis_y = Line {
            start: Point::new(self.unit_y.x, self.unit_y.y * -(self.ticks_amount as f32)),
            end: Point::new(self.unit_y.x, self.unit_y.y * (self.ticks_amount as f32)),
            stroke: strokes::axis_green(),
        };

        let mut lines: Vec<Line> = vec![];

        // OY Grid
        for i in -self.ticks_amount..=self.ticks_amount {
            if i == 0 {
                continue;
            }

            let x = self.unit_x.x * (i as f32);

            let start = Point::new(x, axis_y.start.y);
            let end = Point::new(x, axis_y.end.y);

            lines.push(Line::new(start, end, strokes::grid_gray()));
        }

        // OX Grid
        for i in -self.ticks_amount..=self.ticks_amount {
            if i == 0 {
                continue;
            }

            let y = self.unit_y.y * (i as f32);

            let start = Point::new(axis_x.start.x, y);
            let end = Point::new(axis_x.end.x, y);

            lines.push(Line::new(start, end, strokes::grid_gray()));
        }

        // Pushing main axes
        lines.push(axis_x);
        lines.push(axis_y);

        lines
    }
}
