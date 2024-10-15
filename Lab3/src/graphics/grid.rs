use crate::geometry::point::Point;

pub const UNIT_LENGTH: f32 = 1.0;

pub struct Grid {
    pub origin: Point,
    pub unit_x: Point,
    pub unit_y: Point,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            origin: Point::new(0.0, 0.0),
            unit_x: Point::new(UNIT_LENGTH, 0.0),
            unit_y: Point::new(0.0, UNIT_LENGTH),
        }
    }
}

impl Grid {}
