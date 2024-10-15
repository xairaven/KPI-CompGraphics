use crate::geometry::point::Point;
use crate::graphics::grid;

pub const MIN_PX_PER_CM: f32 = 10.0;
pub const MAX_PX_PER_CM: f32 = 100.0;

#[derive(Debug, Clone, Copy)]
pub struct ScreenParams {
    pub base: Point,
    pub px_per_cm: f32,
}

impl Default for ScreenParams {
    fn default() -> Self {
        Self {
            base: Point::new(0.0, 0.0).with_converted_checked(),
            px_per_cm: 20.0,
        }
    }
}

impl ScreenParams {
    pub fn convert_single(&self, value: f32) -> f32 {
        value / grid::UNIT_LENGTH * self.px_per_cm
    }

    pub fn convert_point(&self, point: Point) -> Point {
        debug_assert!(!point.converted_to_screen);

        Point {
            x: self.base.x + (point.x / grid::UNIT_LENGTH * self.px_per_cm),
            y: self.base.y - (point.y / grid::UNIT_LENGTH * self.px_per_cm),
            converted_to_screen: true,
        }
    }
}
