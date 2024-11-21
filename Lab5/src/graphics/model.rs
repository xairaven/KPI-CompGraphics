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

    pub fn hide_bad_angles(&mut self) {
        if matches!(self.angle_z_degrees, 46..=90) {
            self.angle_z_degrees = 135;
        }
        if matches!(self.angle_z_degrees, 91..=134) {
            self.angle_z_degrees = 45;
        }
        if matches!(self.angle_z_degrees, 226..=270) {
            self.angle_z_degrees = 316;
        }
        if matches!(self.angle_z_degrees, 271..=315) {
            self.angle_z_degrees = 225;
        }
        if matches!(self.angle_z_degrees, -90..=-45) {
            self.angle_z_degrees = -135;
        }
        if matches!(self.angle_z_degrees, -134..=-91) {
            self.angle_z_degrees = -44;
        }
        if matches!(self.angle_z_degrees, -270..=-226) {
            self.angle_z_degrees = -316;
        }
        if matches!(self.angle_z_degrees, -315..=-271) {
            self.angle_z_degrees = -225;
        }
    }
}
