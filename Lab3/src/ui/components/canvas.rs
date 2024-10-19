use crate::context::Context;
use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::graphics::screen::{Resolution, ScreenParams};
use crate::models::bezier_point::BezierPointType;
use crate::traits::positionable::Positionable;
use crate::ui::styles::colors;
use eframe::epaint::Shape;
use egui::{Frame, Response, Sense};

#[derive(Default)]
pub struct Canvas {
    pub screen_params: ScreenParams,

    pub grid_lines: Vec<Line<Point>>,
    pub model_lines: Vec<Line<Point>>,
    pub skeleton_lines: Vec<Line<Point>>,
}

impl Canvas {
    pub fn process(&mut self, _ui: &mut egui::Ui, context: &mut Context) {
        // Creating grid:
        if context.grid.is_enabled {
            self.grid_lines = context.grid.lines(self.screen_params)
        }

        // Creating skeleton:
        self.skeleton_lines = context.model.skeleton_lines(self.screen_params);

        // Creating model:
        self.model_lines = context.model.lines(self.screen_params);
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, context: &mut Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::hover());
        self.screen_params.canvas_center = Point::from_pos2(response.rect.center());
        self.screen_params.resolution = Resolution::from(response.rect.max.x, response.rect.max.y);

        // Draw grid:
        if context.grid.is_enabled {
            let grid_shapes: Vec<Shape> = self
                .grid_lines
                .iter()
                .map(|line| line.to_screen(self.screen_params).to_shape())
                .collect();
            painter.extend(grid_shapes);
        }

        // Draw model lines
        let model_lines: Vec<Shape> = self
            .model_lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(model_lines);

        // Draw skeleton lines
        let skeleton_lines: Vec<Shape> = self
            .skeleton_lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(skeleton_lines);

        // Point radius
        let points_radius = self.screen_params.value_cm_to_px(context.model.radius);

        // Draw points:
        let point_shapes: Vec<Shape> = context
            .model
            .points
            .iter()
            .map(|bezier| {
                let mut outline_stroke = context.model.outline;
                outline_stroke.width = self
                    .screen_params
                    .value_cm_to_px(context.model.outline.width);

                let color = match bezier.kind {
                    BezierPointType::Control => context.model.fill_control,
                    BezierPointType::Defining => context.model.fill_defining,
                };

                bezier.point.to_screen(self.screen_params).to_dot(
                    points_radius,
                    color,
                    outline_stroke,
                )
            })
            .collect();
        painter.extend(point_shapes);

        // Updating model
        context
            .model
            .points
            .iter_mut()
            .enumerate()
            .for_each(|(index, bezier)| {
                bezier
                    .point
                    .update_self(points_radius, self.screen_params, ui, &response);

                if context.model.are_tooltips_enabled {
                    bezier.point.show_tooltip(
                        index + 1,
                        points_radius,
                        self.screen_params,
                        ui,
                        &response,
                    );
                }
            });

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
