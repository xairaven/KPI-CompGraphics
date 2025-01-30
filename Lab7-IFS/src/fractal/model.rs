use crate::fractal::dot::Dot;
use crate::fractal::system::EquationSystem;
use eframe::epaint::Color32;
use rand::distr::weighted::WeightedIndex;
use rand::distr::Distribution;

pub const DEFAULT_ITERATIONS: u32 = 20000;
pub const DEFAULT_RADIUS: f32 = 0.025;

pub struct Model {
    systems: Vec<[f32; 7]>,
    colors: Vec<Color32>,

    iterations: u32,
    radius: f32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            systems: vec![],
            colors: vec![],

            iterations: DEFAULT_ITERATIONS,
            radius: DEFAULT_RADIUS,
        }
    }
}

impl Model {
    pub fn dots(&self) -> Vec<Dot> {
        debug_assert!(self.systems.len() == self.colors.len());

        let mut equations: Vec<EquationSystem> = Vec::new();
        for (index, parameters) in self.systems.iter().enumerate() {
            let equation =
                EquationSystem::new_colored(*parameters, self.colors[index], self.radius);
            equations.push(equation);
        }

        let mut dots: Vec<Dot> = Vec::new();

        let start_dot = Dot::default();
        dots.push(start_dot);

        let probabilities: Vec<f32> = equations
            .iter()
            .map(|equation| equation.probability())
            .collect();
        let mut rng = rand::rng();

        let dist = match WeightedIndex::new(&probabilities) {
            Ok(value) => value,
            Err(err) => {
                log::error!(
                    "{}",
                    format!(
                        "Error occurred while creating weighted index. Additional Info: {}",
                        err
                    )
                );
                std::process::exit(1);
            },
        };

        for current_index in 0..self.iterations {
            let equation = &equations[dist.sample(&mut rng)];
            let new_dot = equation.next_dot(&dots[current_index as usize]);

            dots.push(new_dot);
        }

        dots
    }

    pub fn with_systems(mut self, systems: Vec<[f32; 7]>) -> Self {
        self.systems = systems;
        self
    }

    pub fn with_colors(mut self, colors: Vec<Color32>) -> Self {
        self.colors = colors;
        self
    }

    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}
