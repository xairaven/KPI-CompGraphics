use crate::errors::validation::ValidationError;
use crate::fractal::validator;
use crate::geometry::point2d::Point2D;
use egui::Color32;

pub struct EquationSystem {
    a: f32,
    b: f32,
    d: f32,
    e: f32,
    c: f32,
    f: f32,
    p: f32,

    color: Option<Color32>,
}

impl EquationSystem {
    pub fn new(coefficients: [f32; 7]) -> Result<Self, ValidationError> {
        validator::probability(coefficients[6])?;

        Ok(Self {
            a: coefficients[0],
            b: coefficients[1],
            d: coefficients[2],
            e: coefficients[3],
            c: coefficients[4],
            f: coefficients[5],
            p: coefficients[6],

            color: None,
        })
    }

    pub fn new_colored(coefficients: [f32; 7], color: Color32) -> Result<Self, ValidationError> {
        let mut system = Self::new(coefficients)?;
        system.color = Some(color);

        Ok(system)
    }

    pub fn probability(&self) -> f32 {
        self.p
    }

    pub fn next_point(&self, start: &Point2D) -> Point2D {
        let x = self.a * start.x + self.b * start.y + self.c;
        let y = self.d * start.x + self.e * start.y + self.f;

        Point2D::new(x, y)
    }
}
