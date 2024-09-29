use crate::models::point::Point;
use crate::models::screen_params::ScreenParams;
use eframe::epaint::Stroke;
use egui::{Color32, Shape};

pub const SHADOW_COLOR: Color32 = Color32::from_rgb(171, 171, 171);

pub struct Line {
    pub start: Point,
    pub end: Point,

    pub stroke: Stroke,
}

impl Line {
    pub fn new(start: Point, end: Point, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }
    pub fn new_plain(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
            stroke: Stroke::default(),
        }
    }

    pub fn to_shape(&self) -> Shape {
        Shape::line(vec![self.start.to_pos2(), self.end.to_pos2()], self.stroke)
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Self {
        Self {
            start: self.start.to_screen(screen_params),
            end: self.end.to_screen(screen_params),
            stroke: self.stroke,
        }
    }

    pub fn to_screen_shape(&self, screen_params: ScreenParams) -> Shape {
        Shape::line(
            vec![
                self.start.to_screen_pos2(screen_params),
                self.end.to_screen_pos2(screen_params),
            ],
            self.stroke,
        )
    }

    pub fn length(&self) -> f32 {
        f32::sqrt((self.end.x - self.start.x).powf(2.0) + (self.end.y - self.start.y).powf(2.0))
    }

    pub fn color_shadow(lines: &[Line]) -> Vec<Line> {
        lines
            .iter()
            .map(|line| {
                Line::new(
                    line.start,
                    line.end,
                    Stroke::new(line.stroke.width, SHADOW_COLOR),
                )
            })
            .collect()
    }
}
