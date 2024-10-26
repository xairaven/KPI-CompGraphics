use crate::graphics::screen::ScreenParams;
use crate::traits::positionable::Positionable;
use eframe::emath::Pos2;
use eframe::epaint::{CircleShape, Color32, Shape, Stroke};

#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,

    // Debug fields:
    pub converted_to_screen: bool,
}

impl Positionable for Point {
    fn new(x: f32, y: f32) -> Self {
        Self {
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

    fn from_pos2(pos: Pos2) -> Self {
        Self {
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
            x: self.x,
            y: self.y,
            converted_to_screen: true,
        }
    }

    fn with_converted_unchecked(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            converted_to_screen: false,
        }
    }
}
