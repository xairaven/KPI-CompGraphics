use crate::context::Context;
use crate::ui::components::canvas::Canvas;
use crate::ui::windows::main_window;

#[derive(Default)]
pub struct App {
    pub canvas: Canvas,
    pub context: Context,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            main_window::show(self, ui, ctx);
        });
    }
}
