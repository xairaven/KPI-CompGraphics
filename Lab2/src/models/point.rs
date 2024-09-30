use crate::models::screen::ScreenParams;
use eframe::emath::Pos2;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,

    // For debug:
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

    pub fn to_pos2(&self) -> Pos2 {
        Pos2::from([self.x, self.y])
    }

    pub fn from_pos2(pos: Pos2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            converted_to_screen: false,
        }
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Self {
        screen_params.convert_point(*self)
    }

    pub fn with_converted_checked(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            converted_to_screen: true,
        }
    }
}
