use crate::math::angle::Angle;

#[derive(Default)]
pub struct Model {
    pub angle_z_degrees: i32,

    pub p: f32,
}

impl Model {
    pub fn angle_y(&self) -> Angle {
        let angle_z = self.angle_z();

        let radians = f32::asin(f32::sqrt(
            f32::sin(angle_z.radian()).powf(2.0) / f32::cos(angle_z.radian()).powf(2.0),
        ));

        Angle::from_radian(radians)
    }

    pub fn angle_z(&self) -> Angle {
        Angle::from_degree(self.angle_z_degrees as f32)
    }
}
