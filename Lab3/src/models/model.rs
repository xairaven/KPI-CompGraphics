use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::graphics::screen::ScreenParams;
use crate::models::bezier_point::BezierPoint;
use crate::traits::positionable::Positionable;
use crate::ui::styles::{colors, strokes};
use egui::{Color32, Stroke};

pub struct Model {
    pub points: Vec<BezierPoint>,

    pub outline: Stroke,
    pub fill_control: Color32,
    pub fill_defining: Color32,

    pub skeleton_stroke: Stroke,

    pub radius: f32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            points: Self::default_points(),
            outline: strokes::bezier_outline(0.02),
            fill_control: colors::RED,
            fill_defining: colors::GREEN,

            skeleton_stroke: strokes::skeleton_dark_grey(0.1),

            radius: 0.1,
        }
    }
}

impl Model {
    pub fn skeleton_lines(&self, screen_params: ScreenParams) -> Vec<Line<Point>> {
        let mut stroke = self.skeleton_stroke;
        stroke.width = screen_params.value_cm_to_px(self.skeleton_stroke.width);

        self.points
            .windows(2)
            .map(|pair| {
                let start = Point::new(pair[0].point.x, pair[0].point.y);
                let end = Point::new(pair[1].point.x, pair[1].point.y);

                Line::new(start, end, stroke)
            })
            .collect()
    }

    pub fn default_points() -> Vec<BezierPoint> {
        vec![
            BezierPoint::control(1.0, 1.0),
            BezierPoint::defining(5.0, 5.0),
            BezierPoint::control(10.0, 10.0),
        ]
    }
}
