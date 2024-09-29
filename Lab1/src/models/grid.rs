use crate::math::angle::Angle;
use crate::models::line::Line;
use crate::models::point::Point;
use eframe::epaint::{Color32, Stroke};

pub struct Grid {
    pub ticks: u32,

    pub point_origin: Point,
    pub point_unit_x: Point,
    pub point_unit_y: Point,

    pub tick_unit_x: Point,
    pub tick_unit_y: Point,

    pub arrows_length: f32,
    pub tick_marks_length: f32,

    pub axis_stroke: Stroke,
    pub grid_stroke: Stroke,
    pub marks_stroke: Stroke,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            ticks: 20,

            point_origin: Point::new(0.0, 0.0),
            point_unit_x: Point::new(1.0, 0.0),
            point_unit_y: Point::new(0.0, 1.0),

            tick_unit_x: Point::new(10.0, 0.0),
            tick_unit_y: Point::new(0.0, 10.0),

            arrows_length: 5.0,
            tick_marks_length: 3.0,

            axis_stroke: Stroke::new(1.8, Color32::from_rgb(0, 0, 0)),
            grid_stroke: Stroke::new(0.8, Color32::from_rgb(200, 200, 200)),
            marks_stroke: Stroke::new(1.0, Color32::from_rgb(0, 0, 0)),
        }
    }
}

impl Grid {
    pub fn lines(&self) -> Vec<Line> {
        // (x; 0)
        let axis_x_end = Point {
            x: self.tick_unit_x.x * (self.ticks as f32),
            y: self.tick_unit_x.y,
        };

        // (0;y)
        let axis_y_end = Point {
            x: self.tick_unit_y.x,
            y: self.tick_unit_y.y * (self.ticks as f32),
        };

        // (0;0)
        let point_origin = Point {
            x: self.point_origin.x,
            y: self.point_origin.y,
        };

        let mut lines: Vec<Line> = vec![];

        // OY
        for i in 1..=self.ticks {
            let x = self.tick_unit_x.x * i as f32;

            let start = Point { x, y: 0.0 };
            let end = Point { x, y: axis_y_end.y };

            lines.push(Line::new(start, end, self.grid_stroke));
        }

        // OX
        for i in 1..=self.ticks {
            let y = self.tick_unit_y.y * i as f32;

            let start = Point { x: 0.0, y };
            let end = Point { x: axis_x_end.x, y };

            lines.push(Line::new(start, end, self.grid_stroke));
        }

        // Main Axes
        lines.push(Line::new(point_origin, axis_x_end, self.axis_stroke));
        lines.push(Line::new(point_origin, axis_y_end, self.axis_stroke));

        for line in self.axis_arrows(axis_x_end, axis_y_end) {
            lines.push(line);
        }
        for line in self.tick_marks() {
            lines.push(line);
        }

        lines
    }

    fn axis_arrows(&self, x_end: Point, y_end: Point) -> Vec<Line> {
        let degree = Angle::from_degree(45.0).radian();
        let delta = self.arrows_length * f32::sin(degree);

        let stroke = self.axis_stroke;

        let first_arrow_y = Line::new(
            y_end,
            Point {
                x: y_end.x - delta,
                y: y_end.y - delta,
            },
            stroke,
        );
        let second_arrow_y = Line::new(
            y_end,
            Point {
                x: y_end.x + delta,
                y: y_end.y - delta,
            },
            stroke,
        );

        let first_arrow_x = Line::new(
            x_end,
            Point {
                x: x_end.x - delta,
                y: x_end.y - delta,
            },
            stroke,
        );
        let second_arrow_x = Line::new(
            x_end,
            Point {
                x: x_end.x - delta,
                y: x_end.y + delta,
            },
            stroke,
        );

        vec![first_arrow_x, second_arrow_x, first_arrow_y, second_arrow_y]
    }

    fn tick_marks(&self) -> Vec<Line> {
        let stroke = self.marks_stroke;

        let tick_ox = Line::new(
            Point {
                x: self.tick_unit_x.x,
                y: self.tick_unit_x.y + self.tick_marks_length,
            },
            Point {
                x: self.tick_unit_x.x,
                y: self.tick_unit_x.y - self.tick_marks_length,
            },
            stroke,
        );
        let tick_oy = Line::new(
            Point {
                x: self.tick_unit_y.x + self.tick_marks_length,
                y: self.tick_unit_y.y,
            },
            Point {
                x: self.tick_unit_y.x - self.tick_marks_length,
                y: self.tick_unit_y.y,
            },
            stroke,
        );

        vec![tick_ox, tick_oy]
    }
}
