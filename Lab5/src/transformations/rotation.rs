use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;

#[derive(Default)]
pub struct Rotation {
    pub is_applied: bool,

    pub angle_deg_x: f32,
    pub angle_deg_y: f32,
    pub angle_deg_z: f32,

    static_angle_deg_x: f32,
    static_angle_deg_y: f32,
    static_angle_deg_z: f32,
}

impl Rotation {
    pub fn apply_if_set(&mut self, model: &mut [Line3D]) {
        if self.is_applied {
            self.static_angle_deg_x += self.angle_deg_x;
            self.static_angle_deg_y += self.angle_deg_y;
            self.static_angle_deg_z += self.angle_deg_z;
            self.is_applied = false;
        }
        if self.are_statics_not_default() {
            self.process(model);
        }
    }

    pub fn process(&self, model: &mut [Line3D]) {
        model.iter_mut().for_each(|line| {
            self.update_point(&mut line.start);
            self.update_point(&mut line.end);
        });
    }

    fn update_point(&self, point: &mut Point3D) {
        let vector = point.to_vector();
    }

    // fn matrix_around_x(&self) -> Matrix4<f32> {
    //     let angle = Angle::from_degree(self.angle_deg_x);
    //
    //     Matrix4::new(
    //
    //     )
    // }

    fn are_statics_not_default(&self) -> bool {
        self.static_angle_deg_x != 0.0
            || self.static_angle_deg_y != 0.0
            || self.static_angle_deg_z != 0.0
    }

    pub fn reset_position(&mut self) {
        self.static_angle_deg_x = 0.0;
        self.static_angle_deg_y = 0.0;
        self.static_angle_deg_z = 0.0;
    }
}
