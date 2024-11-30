use crate::errors::validation::FractalValidationError;
use crate::geometry::line2d::Line2D;
use crate::model::fractal::Fractal;
use crate::model::validator::FractalValidator;
use crate::ui::styles::{colors, strokes};
use egui::{Color32, Stroke};
use std::collections::HashMap;

pub struct FractalViewModel {
    pub is_drawing_requested: bool,

    pub angle: f32,
    pub axiom: String,
    pub rules: Vec<String>,

    pub iterations: usize,
    pub length: usize,

    pub color: Color32,
    stroke: Stroke,

    lines: Vec<Line2D>,

    validator: FractalValidator,
    rules_set: HashMap<char, String>,
}

impl Default for FractalViewModel {
    fn default() -> Self {
        Self {
            is_drawing_requested: false,

            angle: 0.0,
            axiom: String::new(),
            rules: vec![String::new()],
            iterations: 1,
            length: 1,

            color: colors::BLACK,
            stroke: strokes::model_black(1.0),

            lines: Vec::new(),

            validator: FractalValidator::default(),
            rules_set: HashMap::new(),
        }
    }
}

impl FractalViewModel {
    pub fn process(&mut self) -> Vec<Line2D> {
        self.sync_stroke();

        if self.is_drawing_requested {
            self.is_drawing_requested = false;
            self.lines = Fractal::default()
                .with_angle(self.angle)
                .with_axiom(self.axiom.clone())
                .with_rules(self.rules_set.clone())
                .with_iterations(self.iterations)
                .with_length(self.length)
                .with_stroke(self.stroke)
                .lines();
        }

        self.lines.clone()
    }

    pub fn request_draw(&mut self) -> Result<(), FractalValidationError> {
        let validation_result = self.validator.check(self);

        self.is_drawing_requested = validation_result.is_ok();

        self.rules_set = self.validator.rules(&self.rules)?;

        validation_result
    }

    pub fn reset_fractal_settings(&mut self) {
        self.is_drawing_requested = false;

        self.angle = 0.0;
        self.axiom = String::new();
        self.rules = vec![String::new()];
        self.iterations = 1;
        self.length = 1;

        self.rules_set = HashMap::new();
    }

    pub fn reset_with_empty_rules(&mut self) {
        self.reset_fractal_settings();
        self.rules = Vec::with_capacity(3);
    }

    fn sync_stroke(&mut self) {
        self.stroke.color = self.color;
    }
}
