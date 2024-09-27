use crate::models::line::Line;
use crate::models::point::Point;
use eframe::epaint::{Color32, Stroke};
use egui::Pos2;

pub struct Grid {
    pub ticks: u32,

    pub point_origin: Pos2,
    pub point_unit_x: Pos2,
    pub point_unit_y: Pos2,

    pub axis_stroke: Stroke,
    pub grid_stroke: Stroke,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            ticks: 20,

            point_origin: Pos2::from([0.0, 0.0]),
            point_unit_x: Pos2::from([10.0, 0.0]),
            point_unit_y: Pos2::from([0.0, 10.0]),

            axis_stroke: Stroke::new(1.8, Color32::from_rgb(0, 0, 0)),
            grid_stroke: Stroke::new(0.8, Color32::from_rgb(200, 200, 200)),
        }
    }
}

impl Grid {
    pub fn lines(&self) -> Vec<Line> {
        // (x; 0)
        let axis_x_end = Point {
            x: self.point_unit_x.x * (self.ticks as f32),
            y: self.point_unit_x.y,
        };

        // (0;y)
        let axis_y_end = Point {
            x: self.point_unit_y.x,
            y: self.point_unit_y.y * (self.ticks as f32),
        };

        // (0;0)
        let point_origin = Point {
            x: self.point_origin.x,
            y: self.point_origin.y,
        };

        let mut lines: Vec<Line> = vec![];

        // OY
        for i in 1..=self.ticks {
            let x = self.point_unit_x.x * i as f32;

            let start = Point { x, y: 0.0 };
            let end = Point { x, y: axis_y_end.y };

            lines.push(Line::new(start, end, self.grid_stroke));
        }

        // OX
        for i in 1..=self.ticks {
            let y = self.point_unit_y.y * i as f32;

            let start = Point { x: 0.0, y };
            let end = Point { x: axis_x_end.x, y };

            lines.push(Line::new(start, end, self.grid_stroke));
        }

        // Main Axes
        lines.push(Line::new(point_origin, axis_x_end, self.axis_stroke));
        lines.push(Line::new(point_origin, axis_y_end, self.axis_stroke));

        lines
    }
}
