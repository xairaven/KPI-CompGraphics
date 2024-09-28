use eframe::emath::Pos2;

pub const INIT_X: f32 = 50.0;
pub const INIT_Y: f32 = 50.0;

#[derive(Debug, Clone, Copy)]
pub struct ScreenParams {
    pub canvas_height: f32,
    pub px_per_cm: f32,
}

impl Default for ScreenParams {
    fn default() -> Self {
        Self {
            canvas_height: 500.0,
            px_per_cm: 20.0,
        }
    }
}

impl ScreenParams {
    pub fn pos_convert(&self, pos: Pos2) -> Pos2 {
        let x = (pos.x / 10.0 * self.px_per_cm) + INIT_X;
        let y = self.canvas_height - INIT_Y - (pos.y / 10.0 * self.px_per_cm);

        Pos2::from([x, y])
    }
}
