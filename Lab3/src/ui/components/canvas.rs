use crate::context::Context;
use crate::geometry::point::Point;
use crate::graphics::screen::ScreenParams;
use crate::ui::styles::colors;
use egui::{Frame, Response, Sense};

#[derive(Default)]
pub struct Canvas {
    pub screen_params: ScreenParams,
}

impl Canvas {
    pub fn process(&mut self, ui: &mut egui::Ui, context: &mut Context) {}

    pub fn draw(&mut self, ui: &mut egui::Ui, context: &mut Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::hover());
        self.screen_params.base = Point::new(0.0, response.rect.max.y);

        response
    }

    pub fn show_content(&mut self, context: &mut Context, ui: &mut egui::Ui) {
        Frame::canvas(ui.style())
            .fill(colors::WHITE)
            .show(ui, |ui| {
                ui.input(|i| {
                    let delta = i.smooth_scroll_delta.y;
                    self.screen_params.px_per_cm += delta * 0.1;
                });
                self.process(ui, context);
                self.draw(ui, context);
            });
    }
}
