use crate::errors::validation::ValidationError;
use crate::fractal::validator;
use crate::ui::styles::colors;
use egui::Color32;

const DEFAULT_SYSTEM: [f32; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0];

pub struct FractalView {
    initialized: bool,

    pub systems: Vec<[f32; 7]>,
    pub colors: Vec<Color32>,
}

impl Default for FractalView {
    fn default() -> Self {
        Self {
            initialized: false,

            systems: vec![DEFAULT_SYSTEM],
            colors: vec![colors::BLACK],
        }
    }
}

impl FractalView {
    pub fn add_system(&mut self) {
        self.systems.push(DEFAULT_SYSTEM);
        self.colors.push(colors::BLACK);
    }

    pub fn remove_system(&mut self, index: usize) {
        debug_assert!(index < self.systems.len());
        debug_assert!(index < self.colors.len());

        self.systems.remove(index);
        self.colors.remove(index);
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
