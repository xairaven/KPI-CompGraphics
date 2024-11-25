pub struct MessageBox {
    name: String,
    message: String,

    is_open: bool,
}

impl MessageBox {
    pub fn new(name: String, msg: String) -> Self {
        Self {
            name: name.to_string(),
            message: msg,
            is_open: true,
        }
    }

    pub fn show(&mut self, ui: &egui::Ui) {
        egui::Window::new(&self.name)
            .open(&mut self.is_open)
            .default_height(100.0)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ui.label(&self.message);
            });
    }

    pub fn is_closed(&self) -> bool {
        !self.is_open
    }
}
