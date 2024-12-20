use crate::context::Context;
use crate::geometry::line2d::Line2D;
use crate::geometry::point2d::Point2D;
use crate::graphics::screen::{Resolution, ScreenParams};
use crate::ui::styles::colors;
use crate::ui::windows::message::MessageWindow;
use eframe::epaint::Shape;
use egui::{Frame, Response, Sense};
use std::fs;

pub struct Canvas {
    pub screen_params: ScreenParams,

    pub lines: Vec<Line2D>,

    filter_name: String,
    filter_file_extensions: Vec<&'static str>,

    error_window: Option<MessageWindow>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            screen_params: Default::default(),
            lines: Default::default(),

            filter_name: String::from("PNG"),
            filter_file_extensions: vec!["png"],

            error_window: None,
        }
    }
}

impl Canvas {
    pub fn process(&mut self, context: &mut Context) {
        let mut lines: Vec<Line2D> = Vec::new();

        let grid = context.grid.process(self.screen_params);
        grid.iter().for_each(|line| {
            lines.push(*line);
        });

        let fractal = context.fractal_view_model.process();
        fractal.iter().for_each(|line| {
            lines.push(*line);
        });

        self.lines = lines;
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) -> Response {
        let painter_size = ui.available_size_before_wrap();
        let (response, painter) = ui.allocate_painter(painter_size, Sense::click_and_drag());
        self.screen_params.canvas_center = Point2D::from_pos2(response.rect.center());
        self.screen_params.resolution = Resolution::from(response.rect.max.x, response.rect.max.y);

        let line_shapes: Vec<Shape> = self
            .lines
            .iter()
            .map(|line| line.to_screen(self.screen_params).to_shape())
            .collect();
        painter.extend(line_shapes);

        // Check for dragging
        self.screen_params.update_offset_on_drag(ui, &response);

        // Check for screenshot:
        ui.input(|i| {
            for event in &i.raw.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(&self.filter_name, &self.filter_file_extensions)
                        .save_file()
                    {
                        let pixels_per_point = i.pixels_per_point();

                        let region = response.rect;
                        let color_image = image.region(&region, Some(pixels_per_point));

                        if let Err(error) = image::save_buffer(
                            &path,
                            color_image.as_raw(),
                            color_image.width() as u32,
                            color_image.height() as u32,
                            image::ColorType::Rgba8,
                        ) {
                            let _ = fs::remove_file(&path);

                            self.error_window = Some(
                                MessageWindow::default()
                                    .with_message(format!(
                                        "Error occurred while saving screenshot: \n\n{}",
                                        error
                                    ))
                                    .with_name("Error ❎")
                                    .with_height(500.0)
                                    .with_width(300.0)
                                    .with_collapsible(false),
                            );
                        }
                    }
                }
            }
        });

        response
    }

    fn show_windows_if_opened(&mut self, ui: &mut egui::Ui) {
        let windows = vec![&mut self.error_window];

        for window_option in windows {
            if let Some(window) = window_option {
                window.show(ui);

                if window.is_closed() {
                    *window_option = None;
                }
            }
        }
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

        self.show_windows_if_opened(ui);
    }
}
