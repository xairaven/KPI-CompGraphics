use crate::models::screen_params::ScreenParams;
use egui::Pos2;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn to_pos2(&self) -> Pos2 {
        Pos2::from([self.x, self.y])
    }

    pub fn from_pos2(pos: Pos2) -> Self {
        Self { x: pos.x, y: pos.y }
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Pos2 {
        let pos = Pos2::from([self.x, self.y]);
        screen_params.pos_convert(pos)
    }
}
