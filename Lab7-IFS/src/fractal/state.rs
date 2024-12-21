use crate::errors::validation::ValidationError;
use crate::fractal::dot::Dot;
use crate::fractal::model::Model;
use crate::fractal::validator;
use crate::ui::styles::colors;
use egui::Color32;

const DEFAULT_SYSTEM: [f32; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0];

pub struct FractalState {
    initialized: bool,
    is_drawing_requested: bool,

    pub is_coloring_enabled: bool,

    pub systems: Vec<[f32; 7]>,
    pub colors: Vec<Color32>,

    pub iterations: u32,
    pub radius_centimeters: f32,

    dots: Vec<Dot>,
}

impl Default for FractalState {
    fn default() -> Self {
        Self {
            initialized: false,
            is_drawing_requested: false,
            is_coloring_enabled: false,

            systems: vec![DEFAULT_SYSTEM],
            colors: vec![colors::BLACK],

            iterations: 10000,
            radius_centimeters: 0.01,
            dots: vec![],
        }
    }
}

impl FractalState {
    pub fn process(&mut self) -> Vec<Dot> {
        if self.is_drawing_requested {
            self.is_drawing_requested = false;
            self.dots = Model::default()
                .with_systems(self.systems.clone())
                .with_colors(self.colors.clone())
                .with_iterations(self.iterations)
                .with_radius(self.radius_centimeters)
                .dots();
        }

        self.dots.clone()
    }

    pub fn request_draw(&mut self) {
        self.is_drawing_requested = true;
    }

    pub fn add_system(&mut self) {
        self.reset_initialization();

        self.systems.push(DEFAULT_SYSTEM);
        self.colors.push(colors::BLACK);
    }

    pub fn remove_system(&mut self, index: usize) {
        debug_assert!(self.systems.len() == self.colors.len());

        self.reset_initialization();

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
