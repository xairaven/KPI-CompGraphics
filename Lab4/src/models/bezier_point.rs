use crate::geometry::moveable_point::MoveablePoint;
use crate::models::model::Model;
use crate::traits::positionable::Positionable;
use eframe::epaint::Color32;
use egui::Response;

#[derive(Debug, Copy, Clone)]
pub struct BezierPoint {
    pub point: MoveablePoint,

    pub kind: BezierPointType,
    pub smoothness: SmoothnessType,
}

impl BezierPoint {
    pub fn defining(x: f32, y: f32) -> Self {
        Self {
            point: MoveablePoint::new(x, y),

            kind: BezierPointType::Defining,
            smoothness: SmoothnessType::None,
        }
    }

    pub fn control(x: f32, y: f32) -> Self {
        Self {
            point: MoveablePoint::new(x, y),

            kind: BezierPointType::Control,
            smoothness: SmoothnessType::Break,
        }
    }

    pub fn color(&self, model: &Model) -> Color32 {
        match self.kind {
            BezierPointType::Control if self.smoothness == SmoothnessType::Smooth => {
                model.fill_smooth
            },
            BezierPointType::Control => model.fill_control,
            BezierPointType::Defining => model.fill_defining,
        }
    }

    pub fn update_on_change_smoothness(&mut self, ui: &egui::Ui, response: &Response) {
        if let BezierPointType::Defining = self.kind {
            return;
        }

        if response.secondary_clicked() {
            self.smoothness = match self.smoothness {
                SmoothnessType::None => SmoothnessType::None,
                SmoothnessType::Break => SmoothnessType::Smooth,
                SmoothnessType::Smooth => SmoothnessType::Break,
            };

            ui.ctx().request_repaint();
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BezierPointType {
    Control,
    Defining,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SmoothnessType {
    // If point is defining
    None,

    Break,
    Smooth,
}
