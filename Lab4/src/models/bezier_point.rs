use crate::geometry::moveable_point::MoveablePoint;
use crate::traits::positionable::Positionable;

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
}

#[derive(Debug, Copy, Clone)]
pub enum BezierPointType {
    Control,
    Defining,
}

#[derive(Debug, Copy, Clone)]
pub enum SmoothnessType {
    // If point is defining
    None,

    Break,
    Smooth,
}
