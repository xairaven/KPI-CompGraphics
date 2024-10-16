use crate::geometry::point::Point;

pub const MIN_PX_PER_CM: f32 = 10.0;
pub const MAX_PX_PER_CM: f32 = 100.0;

#[derive(Debug, Clone, Copy)]
pub struct ScreenParams {
    pub canvas_center: Point,
    pub grid_unit_length: f32,
    pub resolution: Resolution,
    pub px_per_cm: f32,
}

impl Default for ScreenParams {
    fn default() -> Self {
        Self {
            canvas_center: Point::new(500.0, 500.0),
            grid_unit_length: 1.0,
            resolution: Default::default(),
            px_per_cm: 20.0,
        }
    }
}

impl ScreenParams {
    pub fn value_cm_to_px(&self, value: f32) -> f32 {
        value / self.grid_unit_length * self.px_per_cm
    }

    pub fn point_cm_to_px(&self, point: Point) -> Point {
        debug_assert!(!point.converted_to_screen);

        Point {
            x: self.canvas_center.x + (point.x / self.grid_unit_length * self.px_per_cm),
            y: self.canvas_center.y - (point.y / self.grid_unit_length * self.px_per_cm),
            converted_to_screen: true,
        }
    }

    pub fn value_px_to_cm(&self, value: f32) -> f32 {
        value / self.px_per_cm * self.grid_unit_length
    }

    pub fn point_px_to_cm(&self, point: Point) -> Point {
        debug_assert!(point.converted_to_screen);

        Point {
            x: (point.x * self.grid_unit_length / self.px_per_cm) - self.canvas_center.x,
            y: (point.y * self.grid_unit_length / self.px_per_cm) + self.canvas_center.y,
            converted_to_screen: false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Resolution {
    pub width: f32,
    pub height: f32,
}

impl Resolution {
    pub fn from(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
