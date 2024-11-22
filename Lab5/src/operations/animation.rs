use crate::graphics::model::Model;
use crate::transformations::rotation::Rotation;

pub struct Animation {
    pub is_running: bool,

    pub use_radius: bool,
    pub min_radius: f32,
    pub max_radius: f32,
    pub step_radius: f32,
    direction_radius: f32,

    pub use_rotation: bool,

    pub step_x: f32,
    direction_x: f32,

    pub step_y: f32,
    direction_y: f32,

    pub step_z: f32,
    direction_z: f32,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            is_running: false,

            use_radius: true,
            min_radius: 5.0,
            max_radius: 10.0,
            step_radius: 0.1,
            direction_radius: 1.0,

            use_rotation: false,
            step_x: 0.0,
            direction_x: 1.0,
            step_y: 0.0,
            direction_y: 1.0,
            step_z: 0.0,
            direction_z: 1.0,
        }
    }
}

impl Animation {
    pub fn process(&mut self, ui: &egui::Ui, model: &mut Model, rotation: &mut Rotation) {
        self.ensure_animation_enabled();
        self.validate_radius();

        if self.is_running {
            if self.use_radius {
                self.step_radius(model);
            }

            if self.use_rotation {
                self.step_rotation(rotation);
            }

            ui.ctx().request_repaint();
        }
    }

    pub fn step_radius(&mut self, model: &mut Model) {
        if model.radius < self.min_radius {
            model.radius = self.min_radius;
            self.direction_radius = 1.0;
        }
        if model.radius > self.max_radius {
            model.radius = self.max_radius;
            self.direction_radius = -1.0;
        }

        model.radius += self.step_radius * self.direction_radius;
    }

    pub fn step_rotation(&mut self, rotation: &mut Rotation) {}

    pub fn checkout_status(&mut self) {
        self.is_running = !self.is_running;
    }

    fn validate_radius(&mut self) {
        if self.min_radius > self.max_radius {
            self.min_radius = self.max_radius - 0.1;
        }

        if self.max_radius < self.min_radius {
            self.max_radius = self.min_radius + 0.1;
        }
    }

    fn ensure_animation_enabled(&mut self) {
        if self.is_running && !self.use_rotation && !self.use_radius {
            self.is_running = false;
        }
    }
}
