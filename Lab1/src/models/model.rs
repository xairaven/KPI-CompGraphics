use crate::models::point::Point;
use eframe::epaint::{Color32, Stroke};
use egui::{Pos2, Shape};

pub struct Model {
    pub a: Pos2,
    pub b: Pos2,
    pub c: Pos2,
    pub d: Pos2,
    pub e: Pos2,
    pub f: Pos2,
    pub g: Pos2,
    pub h: Pos2,

    pub i: Pos2,
    pub j: Pos2,
    pub i_radius: f32,
    pub j_radius: f32,

    pub stroke: Stroke,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            a: Pos2::from([10.0, 120.0]),
            b: Pos2::from([10.0, 10.0]),
            c: Pos2::from([47.0, 10.0]),
            d: Pos2::from([47.0, 45.0]),
            e: Pos2::from([32.0, 45.0]),
            f: Pos2::from([32.0, 85.0]),
            g: Pos2::from([47.0, 85.0]),
            h: Pos2::from([47.0, 120.0]),

            i: Pos2::from([20.0, 20.0]),
            i_radius: 7.5,
            j: Pos2::from([20.0, 110.0]),
            j_radius: 7.5,

            stroke: Stroke::new(2.0, Color32::from_rgb(0, 0, 0)),
        }
    }
}

impl Model {
    pub fn shape(&self, canvas_height: f32, px_per_cm: f32) -> Vec<Shape> {
        let points = [
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.a,
        ];

        let shapes: Vec<Shape> = points
            .windows(2)
            .map(|pair| {
                let first = Point::from_pos2(pair[0]);
                let second = Point::from_pos2(pair[1]);

                Shape::line(
                    vec![
                        first.to_screen(canvas_height, px_per_cm),
                        second.to_screen(canvas_height, px_per_cm),
                    ],
                    self.stroke,
                )
            })
            .collect();

        shapes
    }
}
