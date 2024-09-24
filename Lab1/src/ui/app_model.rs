use crate::models::grid::Grid;
use crate::ui;
use crate::ui::components::canvas::Canvas;

#[derive(Default)]
pub struct AppModel {
    pub canvas: Canvas,
    pub grid: Grid,
}

impl AppModel {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for AppModel {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::main_window::show(self, ui, ctx);
        });
    }
}
