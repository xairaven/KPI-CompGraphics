use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;

#[derive(Default)]
pub struct Offset {
    pub is_applied: bool,

    pub display_x: f32,
    pub display_y: f32,
    pub display_z: f32,

    x: f32,
    y: f32,
    z: f32,
}

impl Offset {
    pub fn apply(&mut self, lines: &mut [Line3D]) {
        if self.is_applied {
            self.x += self.display_x;
            self.y += self.display_y;
            self.z += self.display_z;
            self.is_applied = false;
        }
        if self.are_internals_not_default() {
            self.process(lines);
        }
    }

    pub fn process(&self, lines: &mut [Line3D]) {
        lines.iter_mut().for_each(|line| {
            self.update_point(&mut line.start);
            self.update_point(&mut line.end);
        });
    }

    fn update_point(&self, point: &mut Point3D) {
        point.x += self.x;
        point.y += self.y;
        point.z += self.z;
    }

    fn are_internals_not_default(&self) -> bool {
        self.x != 0.0 || self.y != 0.0 || self.z != 0.0
    }

    pub fn internals(&self) -> Point3D {
        Point3D::new(self.x, self.y, self.z)
    }

    pub fn reset_position(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }
}
