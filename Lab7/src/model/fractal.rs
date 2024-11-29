use crate::geometry::line2d::Line2D;

pub struct Fractal {
    pub angle: f32,
    pub axiom: String,
    pub rules: Vec<String>,

    pub iterations: usize,
    pub length: usize,
}

impl Default for Fractal {
    fn default() -> Self {
        Self {
            angle: 0.0,
            axiom: String::new(),
            rules: vec![String::new()],
            iterations: 1,
            length: 1,
        }
    }
}

impl Fractal {
    pub fn lines(&self) -> Vec<Line2D> {
        let mut lines: Vec<Line2D> = Vec::new();

        lines
    }

    pub fn with_axiom(mut self, axiom: String) -> Self {
        self.axiom = axiom;
        self
    }

    pub fn with_angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    pub fn with_rules(mut self, rules: Vec<String>) -> Self {
        self.rules = rules;
        self
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_length(mut self, length: usize) -> Self {
        self.length = length;
        self
    }
}
