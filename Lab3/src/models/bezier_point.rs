use crate::geometry::moveable_point::MoveablePoint;
use crate::traits::positionable::Positionable;

#[derive(Debug, Copy, Clone)]
pub struct BezierPoint {
    pub point: MoveablePoint,
    pub kind: BezierPointType,
}

impl BezierPoint {
    pub fn defining(x: f32, y: f32) -> Self {
        Self {
            point: MoveablePoint::new(x, y),
            kind: BezierPointType::Defining,
        }
    }

    pub fn control(x: f32, y: f32) -> Self {
        Self {
            point: MoveablePoint::new(x, y),
            kind: BezierPointType::Control,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BezierPointType {
    Control,
    Defining,
}
