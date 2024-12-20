use crate::context::Context;
use crate::fractal::dot::Dot;
use crate::geometry::line2d::Line2D;
use crate::geometry::point2d::Point2D;
use crate::graphics::screen::{Resolution, ScreenParams};
use crate::ui::styles::colors;
use eframe::epaint::Shape;
use egui::{Frame, Response, Sense};

pub struct Canvas {
    pub screen_params: ScreenParams,

    grid: Vec<Line2D>,
    fractal: Vec<Dot>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            screen_params: Default::default(),

            grid: Vec::with_capacity(3),
            fractal: vec![],
        }
    }
}

impl Canvas {
    pub fn process(&mut self, context: &mut Context) {
        self.grid = context.grid.process(self.screen_params);

        self.fractal = context.fractal_view.process();
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::click_and_drag());
        self.screen_params.canvas_center = Point2D::from_pos2(response.rect.center());
        self.screen_params.resolution = Resolution::from(response.rect.max.x, response.rect.max.y);

        let grid_shapes: Vec<Shape> = self
            .grid
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(grid_shapes);

        let fractal_shapes: Vec<Shape> = self
            .fractal
            .iter()
            .map(|dot| dot.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(fractal_shapes);

        // Check for dragging
        self.screen_params.update_offset_on_drag(ui, &response);

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
                self.process(context);
                self.draw(ui);
            });
    }
}
