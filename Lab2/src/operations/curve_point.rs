use crate::math::derivative::{derivative, second_derivative};
use crate::models::dot::Dot;
use crate::models::line::Line;
use crate::models::model::Model;

pub const MIN_SPEED: u32 = 1;
pub const MAX_SPEED: u32 = 10;

pub struct CurvePoint {
    pub index: u32,
    pub length: u32,
    pub dot: Dot,
    pub curvature_radius: f32,

    pub is_running: bool,
    pub is_visible: bool,
    pub direction: Direction,
    pub speed: u32,
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
            curvature_radius: 0.0,

            is_running: false,
            is_visible: false,
            direction: Direction::Right,
            speed: 1,
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

        // If length 1, there's no way to do animation
        if self.length == 1 {
            return;
        }

        // Speed can't be greater than length
        if self.speed > self.length {
            self.speed = self.length - 1;
        }

        // Do we need to hop on next circle?
        match self.direction {
            Direction::Left if (self.index as i32 - self.speed as i32) < 0 => {
                self.index =
                    (model_lines.len() as i32 + self.index as i32 - self.speed as i32) as u32;
            },
            Direction::Right if self.index + self.speed > model_lines.len() as u32 - 1 => {
                self.index = self.index + self.speed - model_lines.len() as u32;
            },
            _ => {},
        }

        self.index = (self.index as i32 + self.speed as i32 * self.direction_coefficient()) as u32;
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

    pub fn update_curvature_radius(&mut self, model: &Model) {
        let point = self.dot.center;

        let derivative = derivative(point.x, point.y, model.a, model.b);
        let second_derivative = second_derivative(point.x, point.y, model.a, model.b);

        let radius = (1.0 + derivative.powf(2.0)).powf(3.0 / 2.0) / f32::abs(second_derivative);

        self.curvature_radius = radius;
    }
}
