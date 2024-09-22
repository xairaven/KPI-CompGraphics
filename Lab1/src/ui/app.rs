use crate::ui;
use crate::ui::elements::canvas::PaintingCanvas;

pub struct Application {
    canvas: PaintingCanvas,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            canvas: Default::default(),
        }
    }
}

impl Application {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::main_window::show(self, ui, ctx);
        });
    }
}
