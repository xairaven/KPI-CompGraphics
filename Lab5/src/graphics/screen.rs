pub const MIN_PX_PER_CM: f32 = 10.0;
pub const MAX_PX_PER_CM: f32 = 100.0;

#[derive(Debug, Clone, Copy)]
pub struct ScreenParams {}

impl Default for ScreenParams {
    fn default() -> Self {
        Self {}
    }
}
