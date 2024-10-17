use crate::graphics::screen::ScreenParams;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Shape, Stroke};
use nalgebra::SMatrix;

pub trait Positionable {
    fn new(x: f32, y: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn to_pos2(&self) -> Pos2 {
        Pos2::from([self.x(), self.y()])
    }
    fn from_pos2(pos: Pos2) -> Self;
    fn to_screen(&self, screen_params: ScreenParams) -> Self;
    fn to_shape(&self, radius: f32, color: Color32) -> Shape;
    fn to_vector(&self) -> SMatrix<f32, 1, 3> {
        SMatrix::<f32, 1, 3>::new(self.x(), self.y(), 1.0)
    }
    fn to_dot(&self, radius: f32, fill: Color32, stroke: Stroke) -> Shape;
    fn is_converted_checked(&self) -> bool;
    fn with_converted_checked(&self) -> Self;
    fn with_converted_unchecked(&self) -> Self;
}
