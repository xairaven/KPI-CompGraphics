const DEFAULT_SYSTEM: [f32; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0];

pub struct FractalView {
    pub initialized: bool,

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
}
