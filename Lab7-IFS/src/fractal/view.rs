use crate::errors::validation::ValidationError;
use crate::fractal::validator;

const DEFAULT_SYSTEM: [f32; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0];

pub struct FractalView {
    initialized: bool,

    pub systems: Vec<[f32; 7]>,
}

impl Default for FractalView {
    fn default() -> Self {
        Self {
            initialized: false,

            systems: vec![DEFAULT_SYSTEM],
        }
    }
}

impl FractalView {
    pub fn add_system(&mut self) {
        self.systems.push(DEFAULT_SYSTEM);
    }

    pub fn initialize(&mut self) -> Result<(), ValidationError> {
        validator::probability_range(&self.systems)?;
        validator::probability_sum(&self.systems)?;

        self.initialized = true;

        Ok(())
    }

    pub fn reset_initialization(&mut self) {
        self.initialized = false;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}
