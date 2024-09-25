use eframe::emath::vec2;
use egui::{emath, Color32, Frame, Pos2, Rect, Sense, Stroke, Vec2};
use std::f32::consts::TAU;

pub struct Canvas {
    pub lines: Vec<Vec<Pos2>>,
    pub px_per_cm: f32,
    pub stroke: Stroke,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            px_per_cm: 35.0,
            stroke: Stroke::new(1.0, Color32::from_rgb(25, 200, 100)),
        }
    }
}

impl Canvas {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.label("Stroke:");
            ui.add(&mut self.stroke);
            ui.separator();
            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let size = ui.available_size_before_wrap();

        let (mut response, painter) = ui.allocate_painter(size, Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
            response.mark_changed();
        }

        let shapes = self
            .lines
            .iter()
            .filter(|line| line.len() >= 2)
            .map(|line| {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                egui::Shape::line(points, self.stroke)
            });

        painter.extend(shapes);

        // let size = Vec2::splat(16.0);
        // let (response, painter) = ui.allocate_painter(size, Sense::hover());
        // let rect = response.rect;
        // let c = rect.center();
        // let r = rect.width() / 2.0 - 1.0;
        // let color = Color32::from_gray(128);
        // let stroke = Stroke::new(1.0, color);
        // painter.circle_stroke(c, r, stroke);
        // painter.line_segment([c - vec2(0.0, r), c + vec2(0.0, r)], stroke);
        // painter.line_segment([c, c + r * Vec2::angled(TAU * 1.0 / 8.0)], stroke);
        // painter.line_segment([c, c + r * Vec2::angled(TAU * 3.0 / 8.0)], stroke);

        response
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.ui_control(ui);
        ui.label("Paint with your mouse/touch!");
        Frame::canvas(ui.style())
            .fill(Color32::from_rgb(255, 255, 255))
            .show(ui, |ui| {
                self.ui_content(ui);
            });
    }
}
