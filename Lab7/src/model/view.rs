use crate::geometry::line2d::Line2D;
use crate::model::fractal::Fractal;
use crate::model::validator::FractalValidator;

pub struct FractalViewModel {
    pub is_drawing_requested: bool,

    pub angle: f32,
    pub axiom: String,
    pub rules: Vec<String>,

    pub iterations: usize,
    pub length: usize,

    validator: FractalValidator,
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

            validator: FractalValidator::default(),
        }
    }
}

impl FractalViewModel {
    pub fn generate(&mut self) -> Vec<Line2D> {
        if self.is_drawing_requested {
            self.is_drawing_requested = false;

            Fractal::default()
                .with_angle(self.angle)
                .with_axiom(self.axiom.clone())
                .with_rules(self.rules.clone())
                .with_iterations(self.iterations)
                .with_length(self.length)
                .lines()
        } else {
            Vec::with_capacity(0)
        }
    }

    pub fn request_draw(&mut self) {
        self.is_drawing_requested = true;

        todo!("VALIDATE")
    }

    pub fn reset_to_defaults(&mut self) {
        *self = Default::default();
    }

    pub fn reset_with_empty_rules(&mut self) {
        self.reset_to_defaults();
        self.rules = Vec::with_capacity(3);
    }
}
