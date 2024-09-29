pub const INIT_X: f32 = 50.0;
pub const INIT_Y: f32 = 50.0;

#[derive(Debug, Clone, Copy)]
pub struct ScreenParams {
    pub canvas_height: f32,
    pub px_per_cm: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Default for ScreenParams {
    fn default() -> Self {
        Self {
            canvas_height: 500.0,
            px_per_cm: 20.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

impl ScreenParams {
    pub fn convert_single(&self, value: f32) -> f32 {
        value / 10.0 * self.px_per_cm
    }

    pub fn convert_xy(&self, x: f32, y: f32) -> (f32, f32) {
        let new_x = (x / 10.0 * self.px_per_cm) + INIT_X - self.offset_x;
        let new_y = self.canvas_height - INIT_Y - (y / 10.0 * self.px_per_cm) + self.offset_y;

        (new_x, new_y)
    }
}
