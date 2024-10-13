use crate::context::Context;
use crate::models::line::Line;
use crate::models::point::Point;
use crate::models::screen::ScreenParams;
use crate::operations::curve_props::CurveProperties;
use crate::ui::styles::colors;
use eframe::epaint::{Color32, Shape};
use egui::{Frame, Response, Sense};

#[derive(Default)]
pub struct Canvas {
    pub screen_params: ScreenParams,

    pub grid_lines: Vec<Line>,
    pub model_lines: Vec<Line>,

    pub tangent_line: Option<Line>,
}

impl Canvas {
    pub fn process(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        // Creating grid:
        self.grid_lines = context.grid.lines();

        // Animation:
        if context.animation_settings.is_running {
            context.animation_settings.step(&mut context.model);
            ui.ctx().request_repaint();
        }

        // Creating model:
        let model_lines = context.model.lines();

        // Curve Point
        if context.curve_point.is_visible && context.curve_point.is_running {
            context.curve_point.step(&model_lines);
            ui.ctx().request_repaint();

            if context.animation_settings.is_running {
                context.animation_settings.is_running = Default::default();
            }

            if context.curve_props.is_tangent_enabled {
                let curve_point = context.curve_point.dot.center;
                self.tangent_line = CurveProperties::tangent_line(
                    curve_point.x,
                    curve_point.y,
                    context.model.a,
                    context.model.b,
                );
            }
        }

        // Passing to draw
        self.model_lines = model_lines;
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, context: &mut Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::hover());
        self.screen_params.canvas_center = Point::from_pos2(response.rect.center());

        // Draw grid:
        let grid_shapes: Vec<Shape> = self
            .grid_lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(grid_shapes);

        // Draw model:
        let model_shapes: Vec<Shape> = self
            .model_lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(model_shapes);

        // Draw derivative
        if context.curve_props.is_tangent_enabled {
            if let Some(line) = self.tangent_line {
                painter.add(line.to_screen(self.screen_params).to_shape());
            }
        }

        // Draw curve dot:
        if context.curve_point.is_visible {
            let shape = context
                .curve_point
                .dot
                .to_screen(self.screen_params)
                .to_shape(colors::PINK);
            painter.add(shape);
        }

        response
    }

    pub fn show_content(&mut self, context: &mut Context, ui: &mut egui::Ui) {
        Frame::canvas(ui.style())
            .fill(Color32::from_rgb(255, 255, 255))
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
