pub struct Application {
    // Example stuff:
    label: String,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
        }
    }
}

impl Application {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for Application {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Smth here
    }
}
