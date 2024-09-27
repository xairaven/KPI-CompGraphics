use crate::context::Context;
use egui::{Color32, Frame, Pos2, Response, Sense, Stroke};

const INIT_X: f32 = 50.0;
const INIT_Y: f32 = 100.0;

pub struct Canvas {
    pub lines: Vec<Vec<Pos2>>,
    pub px_per_cm: f32,

    pub axis_stroke: Stroke,
    pub grid_stroke: Stroke,
    pub model_stroke: Stroke,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            px_per_cm: 15.0,

            axis_stroke: Stroke::new(1.8, Color32::from_rgb(0, 0, 0)),
            grid_stroke: Stroke::new(0.8, Color32::from_rgb(150, 150, 150)),
            model_stroke: Stroke::new(2.0, Color32::from_rgb(0, 0, 0)),
        }
    }
}

impl Canvas {
    pub fn draw(&mut self, ui: &mut egui::Ui, context: &Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::hover());
        let canvas_height = response.rect.max.y;

        let shapes = context
            .model
            .shape(canvas_height, self.px_per_cm, self.model_stroke);
        painter.extend(shapes);

        response
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui, context: &Context) {
        Frame::canvas(ui.style())
            .fill(Color32::from_rgb(255, 255, 255))
            .show(ui, |ui| {
                self.draw(ui, context);
            });
    }
}

pub fn inverse_coordinates(pos: Pos2, max_y: f32, px_per_cm: f32) -> Pos2 {
    let x = (pos.x / 10.0 * px_per_cm) + INIT_X;
    let y = max_y - INIT_Y - (pos.y / 10.0 * px_per_cm);

    Pos2::from([x, y])
}
