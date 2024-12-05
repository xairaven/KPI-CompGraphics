use crate::graphics::screen::ScreenParams;
use crate::math::angle::Angle;
use eframe::emath::Pos2;
use eframe::epaint::{CircleShape, Color32, Shape, Stroke};
use nalgebra::{Matrix3, SMatrix};
use std::f32::consts::PI;

#[derive(Debug, Default, Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,

    // Debug fields:
    pub converted_to_screen: bool,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            converted_to_screen: false,
        }
    }

    pub fn to_vector(&self) -> SMatrix<f32, 1, 3> {
        SMatrix::<f32, 1, 3>::new(self.x, self.y, 1.0)
    }

    pub fn to_pos2(&self) -> Pos2 {
        Pos2::from([self.x, self.y])
    }

    pub fn to_uv(&self, screen_params: &ScreenParams) -> Self {
        let u = self.x * (PI / 6.0) / screen_params.unit_length;
        let v = self.y * (PI / 6.0) / screen_params.unit_length;

        Self::new(u, v)
    }

    pub fn scale(&mut self, scale_factor: f32) {
        self.x *= scale_factor;
        self.y *= scale_factor;
    }

    pub fn offset(&mut self, delta_x: f32, delta_y: f32) {
        self.x += delta_x;
        self.y += delta_y;
    }

    pub fn rotate(&mut self, angle: f32, pivot_point: Point2D) {
        let angle = Angle::from_degree(angle).radian();

        let vector = self.to_vector();
        let matrix = Matrix3::new(
            f32::cos(angle),
            f32::sin(angle),
            0.0,
            -f32::sin(angle),
            f32::cos(angle),
            0.0,
            -pivot_point.x * (f32::cos(angle) - 1.0) + pivot_point.y * f32::sin(angle),
            -pivot_point.y * (f32::cos(angle) - 1.0) - pivot_point.x * f32::sin(angle),
            1.0,
        );

        let result = vector * matrix;
        self.x = result.x;
        self.y = result.y;
    }

    pub fn from_pos2(pos: Pos2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            converted_to_screen: false,
        }
    }

    pub fn to_screen(&self, screen_params: ScreenParams) -> Self {
        screen_params.point_cm_to_px(*self)
    }

    pub fn to_shape(&self, radius: f32, color: Color32) -> Shape {
        Shape::circle_filled(self.to_pos2(), radius, color)
    }

    pub fn to_dot(&self, radius: f32, fill: Color32, stroke: Stroke) -> Shape {
        let mut shape = CircleShape::filled(self.to_pos2(), radius, fill);
        shape.stroke = stroke;

        Shape::Circle(shape)
    }

    pub fn with_converted_checked(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            converted_to_screen: true,
        }
    }

    pub fn with_converted_unchecked(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            converted_to_screen: false,
        }
    }
}
