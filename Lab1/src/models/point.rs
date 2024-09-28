use crate::models::screen_params::ScreenParams;
use egui::Pos2;
use nalgebra::SMatrix;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_vector(&self) -> SMatrix<f32, 1, 3> {
        SMatrix::<f32, 1, 3>::new(self.x, self.y, 1.0)
    }

    pub fn to_pos2(&self) -> Pos2 {
        Pos2::from([self.x, self.y])
    }

    pub fn from_pos2(pos: Pos2) -> Self {
        Self { x: pos.x, y: pos.y }
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Point {
        let (x, y) = screen_params.convert_xy(self.x, self.y);
        Self { x, y }
    }

    pub fn to_screen_pos2(&self, screen_params: ScreenParams) -> Pos2 {
        let (x, y) = screen_params.convert_xy(self.x, self.y);
        Pos2::from([x, y])
    }
}
