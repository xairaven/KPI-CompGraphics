use crate::context::Context;
use crate::math;
use crate::models::dot::Dot;
use crate::models::line::Line;
use crate::models::point::Point;
use crate::models::screen::ScreenParams;
use crate::operations::curve_props::{CurveProperties, NORMAL_LINE_LENGTH, TANGENT_LINE_LENGTH};
use crate::ui::styles::{colors, strokes};
use eframe::epaint::{Color32, Shape};
use egui::{Frame, Response, Sense, Stroke};

#[derive(Default)]
pub struct Canvas {
    pub screen_params: ScreenParams,

    pub grid_lines: Vec<Line>,
    pub model_lines: Vec<Line>,

    pub tangent_line: Option<Line>,
    pub normal_line: Option<Line>,

    pub inflection_points: Vec<Point>,
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

        // Euclidean Offset
        let model_lines = context.offset.process(model_lines);

        // Euclidean Rotation
        let model_lines = context.rotation.process(model_lines);

        // Curvature Radius
        if context.curve_point.is_visible {
            context.curve_point.update_curvature_radius(&context.model);
        }

        // Tangent & Normal Lines
        if context.curve_point.is_visible && context.curve_props.is_tangent_enabled {
            self.tangent_line = Self::build_prop_line(
                context,
                TANGENT_LINE_LENGTH,
                strokes::tangent_blue(),
                CurveProperties::tangent_point,
            );
        }
        if context.curve_point.is_visible && context.curve_props.is_normal_enabled {
            self.normal_line = Self::build_prop_line(
                context,
                NORMAL_LINE_LENGTH,
                strokes::normal_aqua(),
                CurveProperties::normal_point,
            );
        }

        // Curve Point
        if context.curve_point.is_visible && context.curve_point.is_running {
            context.curve_point.step(&model_lines);
            ui.ctx().request_repaint();

            if context.animation_settings.is_running {
                context.animation_settings.is_running = Default::default();
            }
        }

        // Characteristics: Length
        context.curve_props.length(&model_lines);

        // Inflection points
        if context.curve_props.is_inflection_enabled {
            self.inflection_points =
                CurveProperties::inflection_points(&model_lines, context.model.a, context.model.b);
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

        // Draw tangent
        if context.curve_props.is_tangent_enabled {
            if let Some(line) = self.tangent_line {
                painter.add(line.to_screen(self.screen_params).to_shape());
            }
        }

        // Draw normal
        if context.curve_props.is_normal_enabled {
            if let Some(line) = self.normal_line {
                painter.add(line.to_screen(self.screen_params).to_shape());
            }
        }

        // Draw inflection points
        if context.curve_props.is_inflection_enabled {
            let shapes: Vec<Shape> = self
                .inflection_points
                .iter()
                .map(|point| {
                    Dot::from_point(point)
                        .to_screen(self.screen_params)
                        .to_shape(colors::ORANGE)
                })
                .collect();
            painter.extend(shapes);
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

        // Draw rotation dot:
        if context.rotation.x != 0.0 || context.rotation.y != 0.0 {
            let shape = context
                .rotation
                .dot()
                .to_screen(self.screen_params)
                .to_shape(colors::GREEN);
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

    fn build_prop_line(
        context: &Context, length: f32, stroke: Stroke,
        function: fn(f32, f32, f32, f32) -> Option<Point>,
    ) -> Option<Line> {
        let curve_point = context.curve_point.dot.center;

        let point = function(
            curve_point.x,
            curve_point.y,
            context.model.a,
            context.model.b,
        );

        point.map(|point| math::vector::line_with_center(curve_point, point, length, stroke))
    }
}
