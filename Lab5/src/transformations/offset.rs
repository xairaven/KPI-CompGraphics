use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;

#[derive(Default)]
pub struct Offset {
    pub is_applied: bool,

    pub x: f32,
    pub y: f32,
    pub z: f32,

    static_x: f32,
    static_y: f32,
    static_z: f32,
}

impl Offset {
    pub fn apply(&mut self, model: &mut [Line3D]) {
        if self.is_applied {
            self.static_x += self.x;
            self.static_y += self.y;
            self.static_z += self.z;
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
        point.x += self.static_x;
        point.y += self.static_y;
        point.z += self.static_z;
    }

    fn are_statics_not_default(&self) -> bool {
        self.static_x != 0.0 || self.static_y != 0.0 || self.static_z != 0.0
    }

    pub fn reset_position(&mut self) {
        self.static_x = 0.0;
        self.static_y = 0.0;
        self.static_z = 0.0;
    }
}
