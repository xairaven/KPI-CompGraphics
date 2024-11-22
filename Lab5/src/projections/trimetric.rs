use crate::math::angle::Angle;
use nalgebra::Matrix4;

#[derive(Default)]
pub struct TrimetricProjection {
    pub angle_deg_x: i32,
    pub angle_deg_y: i32,
}

impl TrimetricProjection {
    pub fn angle_x(&self) -> Angle {
        Angle::from_degree(self.angle_deg_x as f32)
    }

    pub fn angle_y(&self) -> Angle {
        Angle::from_degree(self.angle_deg_y as f32)
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        let alpha = self.angle_x().radian();
        let beta = self.angle_y().radian();

        Matrix4::new(
            f32::cos(beta),
            f32::sin(beta) * f32::sin(alpha),
            0.0,
            0.0,
            0.0,
            f32::cos(alpha),
            0.0,
            0.0,
            f32::sin(beta),
            -f32::sin(alpha) * f32::cos(beta),
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
}
