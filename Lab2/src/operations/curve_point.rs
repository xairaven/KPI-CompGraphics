use crate::models::point::Point;

pub struct CurvePoint {
    pub index: u32,
    pub length: u32,
    pub point: Point,

    pub is_running: bool,
    pub is_hidden: bool,
    pub direction: Direction,
}

pub enum Direction {
    Left,
    Right,
}

impl Default for CurvePoint {
    fn default() -> Self {
        Self {
            index: 0,
            length: 0,
            point: Default::default(),
            is_running: false,
            is_hidden: true,
            direction: Direction::Right,
        }
    }
}
