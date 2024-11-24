pub struct Animation {
    pub is_running: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Self { is_running: false }
    }
}

impl Animation {
    pub fn process(&mut self, ui: &egui::Ui) {
        if self.is_running {
            self.step();

            ui.ctx().request_repaint();
        }
    }

    fn step(&mut self) {}

    pub fn checkout_status(&mut self) {
        self.is_running = !self.is_running;
    }
}
