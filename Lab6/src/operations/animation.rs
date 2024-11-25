use crate::model::surface::Surface;

pub struct Animation {
    pub is_running: bool,

    pub min_radius: f32,
    pub max_radius: f32,
    pub step_radius: f32,
    direction_radius: f32,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            is_running: false,
            min_radius: 5.0,
            max_radius: 15.0,
            step_radius: 0.1,
            direction_radius: 1.0,
        }
    }
}

impl Animation {
    pub fn process(&mut self, surface: &mut Surface, ui: &egui::Ui) {
        self.validate_radius();

        if self.is_running {
            self.step(surface);

            ui.ctx().request_repaint();
        }
    }

    fn step(&mut self, surface: &mut Surface) {
        if surface.display_radius < self.min_radius {
            surface.display_radius = self.min_radius;
            self.direction_radius = 1.0;
        }
        if surface.display_radius > self.max_radius {
            surface.display_radius = self.max_radius;
            self.direction_radius = -1.0;
        }

        surface.display_radius += self.step_radius * self.direction_radius;
    }

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
}
