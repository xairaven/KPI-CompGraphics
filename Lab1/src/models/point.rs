use crate::ui::components::canvas;
use egui::Pos2;

#[derive(Default, Clone, Copy)]
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

    pub fn to_screen(&self, canvas_height: f32, px_per_cm: f32) -> Pos2 {
        let pos = Pos2::from([self.x, self.y]);
        Self::inverse_coordinates(pos, canvas_height, px_per_cm)
    }

    pub fn inverse_coordinates(pos: Pos2, canvas_height: f32, px_per_cm: f32) -> Pos2 {
        let x = (pos.x / 10.0 * px_per_cm) + canvas::INIT_X;
        let y = canvas_height - canvas::INIT_Y - (pos.y / 10.0 * px_per_cm);

        Pos2::from([x, y])
    }
}
