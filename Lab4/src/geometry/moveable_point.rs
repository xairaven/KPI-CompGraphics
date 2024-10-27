use crate::graphics::screen::ScreenParams;
use crate::traits::positionable::Positionable;
use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{CircleShape, Color32, Shape, Stroke};
use egui::{Id, Response, Sense};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct MoveablePoint {
    pub id: Id,

    pub x: f32,
    pub y: f32,

    // Debug fields:
    pub converted_to_screen: bool,
}

impl MoveablePoint {
    fn generate_id() -> Id {
        Id::new(Uuid::new_v4())
    }

    pub fn interaction_response(
        &self, radius: f32, screen_params: ScreenParams, ui: &egui::Ui, response: &Response,
    ) -> Response {
        let size = Vec2::splat(2.0 * radius);

        let area = Rect::from_center_size(self.to_screen(screen_params).to_pos2(), size);

        ui.interact(area, response.id.with(self.id), Sense::click_and_drag())
    }

    pub fn update_on_drag(
        &mut self, screen_params: ScreenParams, ui: &egui::Ui, response: &Response,
    ) -> bool {
        if response.dragged_by(egui::PointerButton::Primary) {
            let offset = screen_params.vec2_px_to_cm(response.drag_delta());
            self.x += offset.x;
            self.y += offset.y;

            if offset.x != 0.0 || offset.y != 0.0 {
                ui.ctx().request_repaint();
            }

            return true;
        }

        false
    }

    pub fn show_tooltip(&self, index: usize, response: Response) {
        let label = format!(
            "Point #{index}.\nCoordinates:\n- X: {}\n- Y: {}",
            self.x, self.y
        );

        response.on_hover_text(label);
    }
}

impl Positionable for MoveablePoint {
    fn new(x: f32, y: f32) -> Self {
        Self {
            id: Self::generate_id(),
            x,
            y,
            converted_to_screen: false,
        }
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    fn set_y(&mut self, y: f32) {
        self.y = y;
    }

    fn from_pos2(pos: Pos2) -> Self {
        Self {
            id: Self::generate_id(),
            x: pos.x,
            y: pos.y,
            converted_to_screen: true,
        }
    }

    fn to_screen(&self, screen_params: ScreenParams) -> Self {
        screen_params.point_cm_to_px(*self)
    }

    fn to_shape(&self, radius: f32, color: Color32) -> Shape {
        Shape::circle_filled(self.to_pos2(), radius, color)
    }

    fn to_dot(&self, radius: f32, fill: Color32, stroke: Stroke) -> Shape {
        let mut shape = CircleShape::filled(self.to_pos2(), radius, fill);
        shape.stroke = stroke;

        Shape::Circle(shape)
    }

    fn is_converted_checked(&self) -> bool {
        self.converted_to_screen
    }

    fn with_converted_checked(&self) -> Self {
        Self {
            id: self.id,
            x: self.x,
            y: self.y,
            converted_to_screen: true,
        }
    }

    fn with_converted_unchecked(&self) -> Self {
        Self {
            id: self.id,
            x: self.x,
            y: self.y,
            converted_to_screen: false,
        }
    }
}
