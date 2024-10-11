use crate::models::grid;
use crate::models::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct ScreenParams {
    pub canvas_center: Point,
    pub px_per_cm: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Default for ScreenParams {
    fn default() -> Self {
        Self {
            canvas_center: Point::new(500.0, 500.0),
            px_per_cm: 20.0,
            offset_x: 0.0,
            offset_y: 0.0,
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
            x: self.canvas_center.x + (point.x / grid::UNIT_LENGTH * self.px_per_cm)
                - self.offset_x,
            y: self.canvas_center.y
                + (point.y / grid::UNIT_LENGTH * self.px_per_cm)
                + self.offset_y,
            converted_to_screen: true,
        }
    }
}
