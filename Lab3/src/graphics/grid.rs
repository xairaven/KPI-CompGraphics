use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::graphics::screen::ScreenParams;
use crate::traits::positionable::Positionable;
use crate::ui::styles::strokes;
use eframe::epaint::Stroke;

pub const DEFAULT_UNIT_LENGTH: f32 = 1.0;

pub struct Grid {
    pub origin: Point,
    pub unit_x: Point,
    pub unit_y: Point,

    pub is_enabled: bool,

    pub axis_x_stroke: Stroke,
    pub axis_y_stroke: Stroke,
    pub grid_stroke: Stroke,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            origin: Point::new(0.0, 0.0),
            unit_x: Point::new(DEFAULT_UNIT_LENGTH, 0.0),
            unit_y: Point::new(0.0, DEFAULT_UNIT_LENGTH),

            is_enabled: false,

            axis_x_stroke: strokes::axis_red(),
            axis_y_stroke: strokes::axis_green(),
            grid_stroke: strokes::grid_gray(),
        }
    }
}

impl Grid {
    pub fn lines(&mut self, screen_params: ScreenParams) -> Vec<Line<Point>> {
        self.unit_x = Point::new(screen_params.grid_unit_length, 0.0);
        self.unit_y = Point::new(0.0, screen_params.grid_unit_length);

        let offset = (screen_params.offset.0, screen_params.offset.1);

        // Sides of grid: left and right
        let width = (
            screen_params.value_px_to_cm(
                screen_params.resolution.width - screen_params.canvas_center.x + offset.0,
            ),
            screen_params.value_px_to_cm(
                screen_params.resolution.width - screen_params.canvas_center.x - offset.0,
            ),
        );

        // Sides of grid: bottom and top
        let height = (
            screen_params.value_px_to_cm(
                screen_params.resolution.height - screen_params.canvas_center.y - offset.1,
            ),
            screen_params.value_px_to_cm(
                screen_params.resolution.height - screen_params.canvas_center.y + offset.1,
            ),
        );

        let ticks_x = (
            (width.0 - (width.0 % self.unit_x.x)) / self.unit_x.x,
            (width.1 - (width.1 % self.unit_x.x)) / self.unit_x.x,
        );
        let ticks_y = (
            (height.0 - (height.0 % self.unit_y.y)) / self.unit_y.y,
            (height.1 - (height.1 % self.unit_y.y)) / self.unit_y.y,
        );

        let axis_x = Line {
            start: Point::new(-width.0, self.unit_x.y),
            end: Point::new(width.1, self.unit_x.y),
            stroke: self.axis_x_stroke,
        };

        let axis_y = Line {
            start: Point::new(self.unit_y.x, -height.0),
            end: Point::new(self.unit_y.x, height.1),
            stroke: self.axis_y_stroke,
        };

        let mut lines: Vec<Line<Point>> = vec![];

        // OY Grid
        for i in (-ticks_x.0 as i32)..=(ticks_x.1 as i32) {
            if i == 0 {
                continue;
            }

            let x = self.unit_x.x * (i as f32);

            let start = Point::new(x, axis_y.start.y);
            let end = Point::new(x, axis_y.end.y);

            lines.push(Line::new(start, end, self.grid_stroke));
        }

        // OX Grid
        for i in (-ticks_y.0 as i32)..=(ticks_y.1 as i32) {
            if i == 0 {
                continue;
            }

            let y = self.unit_y.y * (i as f32);

            let start = Point::new(axis_x.start.x, y);
            let end = Point::new(axis_x.end.x, y);

            lines.push(Line::new(start, end, self.grid_stroke));
        }

        // Pushing main axes
        lines.push(axis_x);
        lines.push(axis_y);

        lines
    }
}
