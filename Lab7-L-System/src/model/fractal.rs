use crate::geometry::line2d::Line2D;
use crate::geometry::point2d::Point2D;
use crate::math::angle::Angle;
use crate::ui::styles::strokes;
use egui::Stroke;
use std::collections::HashMap;

pub const TERMINAL_SYMBOLS: [char; 5] = ['F', '+', '-', '[', ']'];

pub struct Fractal {
    pub angle: f32,
    pub initial_angle: f32,
    pub axiom: String,
    pub rules: HashMap<char, String>,

    pub iterations: usize,
    pub length: f32,

    pub stroke: Stroke,
}

impl Default for Fractal {
    fn default() -> Self {
        Self {
            angle: 0.0,
            initial_angle: 0.0,

            axiom: String::new(),
            rules: HashMap::new(),
            iterations: 1,
            length: 0.5,

            stroke: strokes::model_black(0.1),
        }
    }
}

impl Fractal {
    pub fn lines(&self) -> Vec<Line2D> {
        let mut lines: Vec<Line2D> = Vec::new();

        let path = self.create_path();

        let mut current_x = 0.0;
        let mut current_y = 0.0;
        let mut current_angle = self.initial_angle;

        let mut stack: Vec<(f32, f32, f32)> = Vec::new();

        for symbol in path.chars() {
            if !TERMINAL_SYMBOLS.contains(&symbol) {
                continue;
            }

            match symbol {
                'F' => {
                    let radians = Angle::from_degree(current_angle).radian();

                    let start = Point2D::new(current_x, current_y);

                    current_x += self.length * f32::cos(radians);
                    current_y += self.length * f32::sin(radians);

                    let end = Point2D::new(current_x, current_y);

                    lines.push(Line2D::new(start, end, self.stroke));
                },
                '+' => {
                    current_angle -= self.angle;
                },
                '-' => {
                    current_angle += self.angle;
                },
                '[' => {
                    stack.push((current_x, current_y, current_angle));
                },
                ']' => {
                    if let Some((x, y, angle)) = stack.pop() {
                        current_x = x;
                        current_y = y;
                        current_angle = angle;
                    }
                },
                _ => {},
            }
        }

        lines
    }

    fn create_path(&self) -> String {
        let mut path = self.axiom.clone();
        let mut buffer = String::new();

        for _ in 1..=self.iterations {
            for symbol in path.chars() {
                if let Some(steps) = self.rules.get(&symbol) {
                    buffer += steps;
                } else {
                    buffer.push(symbol);
                }
            }

            path = buffer;
            buffer = String::new();
        }

        path
    }

    pub fn with_axiom(mut self, axiom: String) -> Self {
        self.axiom = axiom;
        self
    }

    pub fn with_angle(mut self, angle: f32) -> Self {
        self.angle = angle;
        self
    }

    pub fn with_initial_angle(mut self, initial_angle: f32) -> Self {
        self.initial_angle = initial_angle;
        self
    }

    pub fn with_rules(mut self, rules: HashMap<char, String>) -> Self {
        self.rules = rules;
        self
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_length(mut self, length: f32) -> Self {
        self.length = length;
        self
    }

    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }
}
