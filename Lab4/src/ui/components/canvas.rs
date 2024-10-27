use crate::context::Context;
use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::graphics::screen::{Resolution, ScreenParams};
use crate::traits::positionable::Positionable;
use crate::ui::styles::{colors, strokes};
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
    pub fn process(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        // Creating grid:
        if context.grid.is_enabled {
            self.grid_lines = context.grid.lines(self.screen_params)
        }

        // Euclidean Offset
        if context.euclidean_offset.is_enabled {
            context.euclidean_offset.process(&mut context.model);
        }

        // Euclidean Rotation
        if context.euclidean_rotation.is_enabled {
            context.euclidean_rotation.process(&mut context.model);
        }

        // Animation:
        if context.animation_settings.is_running {
            context.euclidean_offset = Default::default();
            context.euclidean_rotation = Default::default();

            context
                .animation_settings
                .process_animation(&mut context.model);
            ui.ctx().request_repaint();
        }

        // Creating skeleton:
        if context.model.is_skeleton_enabled {
            self.skeleton_lines = context.model.skeleton_lines(self.screen_params);
        }

        // Creating model:
        self.model_lines = context.model.lines(self.screen_params);
    }

    pub fn draw(&mut self, ui: &mut egui::Ui, context: &mut Context) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::click_and_drag());
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

        // Dot radius
        let dot_radius = self.screen_params.value_cm_to_px(context.model.radius);

        // Outline stroke for all dots:
        let outline_width = self.screen_params.value_cm_to_px(0.02);
        let outline_stroke = strokes::bezier_outline(outline_width);

        if context.model.is_skeleton_enabled {
            // Draw skeleton lines
            let skeleton_lines: Vec<Shape> = self
                .skeleton_lines
                .iter()
                .map(|line| line.to_screen(self.screen_params).to_shape())
                .collect();
            painter.extend(skeleton_lines);

            // Draw skeleton points:
            let point_shapes: Vec<Shape> = context
                .model
                .points
                .iter()
                .map(|bezier| {
                    let color = bezier.color(&context.model);

                    bezier.point.to_screen(self.screen_params).to_dot(
                        dot_radius,
                        color,
                        outline_stroke,
                    )
                })
                .collect();
            painter.extend(point_shapes);

            // Updating model
            let mut changed_points_indexes: Vec<usize> = vec![];

            context
                .model
                .points
                .iter_mut()
                .enumerate()
                .for_each(|(index, bezier)| {
                    let interaction_response = bezier.point.interaction_response(
                        dot_radius,
                        self.screen_params,
                        ui,
                        &response,
                    );

                    if bezier
                        .point
                        .update_on_drag(self.screen_params, ui, &interaction_response)
                    {
                        changed_points_indexes.push(index);
                    };

                    bezier.update_on_change_smoothness(ui, &interaction_response);

                    if context.model.are_tooltips_enabled {
                        bezier.point.show_tooltip(index + 1, interaction_response);
                    }
                });

            if !changed_points_indexes.is_empty() {
                context.model.update_smoothness(changed_points_indexes);
            }
        }

        // Offset Dot
        if context.euclidean_offset.is_enabled {
            let dot = context
                .euclidean_offset
                .dot
                .to_screen(self.screen_params)
                .to_dot(dot_radius, context.euclidean_offset.color, outline_stroke);
            painter.add(dot);

            let interaction_response = context.euclidean_offset.dot.interaction_response(
                dot_radius,
                self.screen_params,
                ui,
                &response,
            );

            context.euclidean_offset.dot.update_on_drag(
                self.screen_params,
                ui,
                &interaction_response,
            );
        }

        // Rotation Dot
        if context.euclidean_rotation.is_enabled {
            let dot = context
                .euclidean_rotation
                .dot
                .to_screen(self.screen_params)
                .to_dot(dot_radius, context.euclidean_rotation.color, outline_stroke);
            painter.add(dot);

            let interaction_response = context.euclidean_rotation.dot.interaction_response(
                dot_radius,
                self.screen_params,
                ui,
                &response,
            );

            context.euclidean_rotation.dot.update_on_drag(
                self.screen_params,
                ui,
                &interaction_response,
            );
        }

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
                self.process(ui, context);
                self.draw(ui, context);
            });
    }
}
