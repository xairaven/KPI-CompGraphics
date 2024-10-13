use crate::models::dot::Dot;
use crate::models::line::Line;

pub struct CurvePoint {
    pub index: u32,
    pub length: u32,
    pub dot: Dot,

    pub is_running: bool,
    pub is_visible: bool,
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
            dot: Default::default(),
            is_running: false,
            is_visible: false,
            direction: Direction::Right,
        }
    }
}

impl CurvePoint {
    pub fn step(&mut self, model_lines: &[Line]) {
        // Empty model - skip animation
        if model_lines.is_empty() {
            self.default_state();
            return;
        }

        // Got another model - changing state
        if model_lines.len() as u32 != self.length {
            self.index = 0;
            self.length = model_lines.len() as u32;
        }

        // Getting dot
        self.dot = Dot::from_point(&model_lines[self.index as usize].start);

        if self.length == 1 {
            return;
        }

        // Do we need to hop on next circle?
        match self.direction {
            Direction::Left if self.index == 0 => {
                self.index = (model_lines.len() as i32 - 1) as u32;
            },
            Direction::Right if self.index + 1 == model_lines.len() as u32 => {
                self.index = 0;
            },
            _ => {},
        }

        self.index = (self.index as i32 + self.direction_coefficient()) as u32;
    }

    pub fn default_state(&mut self) {
        let default = Self::default();

        self.index = default.index;
        self.length = default.length;
        self.dot = default.dot;
        self.is_running = default.is_running;
        self.is_visible = default.is_visible;
        self.direction = default.direction;
    }

    fn direction_coefficient(&self) -> i32 {
        match self.direction {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}
