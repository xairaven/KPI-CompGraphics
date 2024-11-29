use crate::geometry::line2d::Line2D;

pub struct Fractal {
    pub is_drawing_requested: bool,

    pub angle: f32,
    pub axiom: String,
    pub rules: Vec<String>,

    pub iterations: usize,
    pub length: usize,
}

impl Default for Fractal {
    fn default() -> Self {
        Self {
            is_drawing_requested: false,

            angle: 0.0,
            axiom: String::new(),
            rules: vec![String::new()],
            iterations: 1,
            length: 1,
        }
    }
}

impl Fractal {
    pub fn process(&mut self) -> Vec<Line2D> {
        if self.is_drawing_requested {
            self.is_drawing_requested = false;
            self.lines()
        } else {
            Vec::with_capacity(0)
        }
    }

    fn lines(&self) -> Vec<Line2D> {
        let mut lines: Vec<Line2D> = Vec::new();

        lines
    }
}
