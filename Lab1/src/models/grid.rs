use crate::ui::components::canvas::inverse_coordinates as to_screen;
use eframe::epaint::{Color32, Stroke};
use egui::{Pos2, Shape};

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
    pub fn shape(&self, canvas_height: f32, px_per_cm: f32) -> Vec<Shape> {
        let end_axis_x = Pos2::from([
            self.point_unit_x.x * (self.ticks as f32),
            self.point_unit_x.y,
        ]);
        let end_axis_y = Pos2::from([
            self.point_unit_y.x,
            self.point_unit_y.y * (self.ticks as f32),
        ]);

        let end_axis_x_to_screen = to_screen(end_axis_x, canvas_height, px_per_cm);
        let end_axis_y_to_screen = to_screen(end_axis_y, canvas_height, px_per_cm);
        let point_origin_to_screen = to_screen(self.point_origin, canvas_height, px_per_cm);

        let mut lines: Vec<Shape> = vec![];

        for i in 1..=self.ticks {
            let x = self.point_unit_x.x * i as f32;

            let start = to_screen(Pos2::from([x, 0.0]), canvas_height, px_per_cm);
            let end = to_screen(Pos2::from([x, end_axis_y.y]), canvas_height, px_per_cm);

            let shape = Shape::line(Vec::from([start, end]), self.grid_stroke);
            lines.push(shape);
        }

        for i in 1..=self.ticks {
            let y = self.point_unit_y.y * i as f32;

            let start = to_screen(Pos2::from([0.0, y]), canvas_height, px_per_cm);
            let end = to_screen(Pos2::from([end_axis_x.x, y]), canvas_height, px_per_cm);

            let shape = Shape::line(Vec::from([start, end]), self.grid_stroke);
            lines.push(shape);
        }

        // OX + OY
        lines.push(Shape::line(
            Vec::from([point_origin_to_screen, end_axis_x_to_screen]),
            self.axis_stroke,
        ));
        lines.push(Shape::line(
            Vec::from([point_origin_to_screen, end_axis_y_to_screen]),
            self.axis_stroke,
        ));

        lines
    }
}
