use crate::graphics::screen::ScreenParams;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Shape};
use nalgebra::SMatrix;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,

    // Debug fields:
    pub converted_to_screen: bool,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            converted_to_screen: false,
        }
    }

    pub fn to_vector(&self) -> SMatrix<f32, 1, 3> {
        SMatrix::<f32, 1, 3>::new(self.x, self.y, 1.0)
    }

    pub fn to_pos2(&self) -> Pos2 {
        Pos2::from([self.x, self.y])
    }

    pub fn from_pos2(pos: Pos2) -> Self {
        Self::new(pos.x, pos.y)
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Self {
        screen_params.convert_point(*self)
    }

    pub fn to_shape(&self, radius: f32, color: Color32) -> Shape {
        Shape::circle_filled(self.to_pos2(), radius, color)
    }

    pub fn with_converted_checked(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            converted_to_screen: true,
        }
    }
}
