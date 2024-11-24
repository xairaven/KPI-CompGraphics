use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use crate::math::angle::Angle;
use nalgebra::Matrix4;

#[derive(Default)]
pub struct Rotation {
    pub is_applied: bool,

    pub display_angle_x: f32,
    pub display_angle_y: f32,
    pub display_angle_z: f32,

    angle_x: f32,
    angle_y: f32,
    angle_z: f32,
}

impl Rotation {
    pub fn apply(&mut self, lines: &mut [Line3D], pivot: Point3D) {
        if self.is_applied {
            self.angle_x += self.display_angle_x;
            self.angle_y += self.display_angle_y;
            self.angle_z += self.display_angle_z;
            self.is_applied = false;
        }
        if self.are_internals_not_default() {
            self.process(lines, pivot);
        }
    }

    pub fn process(&self, lines: &mut [Line3D], pivot: Point3D) {
        let matrix_to_origin = self.matrix_offset_to_origin(&pivot);
        let matrix_from_origin = self.matrix_offset_to_point(&pivot);
        let matrix_ox = self.matrix_around_ox();
        let matrix_oy = self.matrix_around_oy();
        let matrix_oz = self.matrix_around_oz();

        let result_matrix =
            matrix_to_origin * matrix_ox * matrix_oy * matrix_oz * matrix_from_origin;

        lines.iter_mut().for_each(|line| {
            self.update_point(&mut line.start, &result_matrix);
            self.update_point(&mut line.end, &result_matrix);
        });
    }

    fn update_point(&self, point: &mut Point3D, result_matrix: &Matrix4<f32>) {
        let vector = point.to_vector();

        let result = vector * result_matrix;

        point.x = result.x;
        point.y = result.y;
        point.z = result.z;
    }

    fn matrix_around_ox(&self) -> Matrix4<f32> {
        let angle = Angle::from_degree(self.angle_x).radian();

        Matrix4::new(
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            f32::cos(angle),
            f32::sin(angle),
            0.0,
            0.0,
            -f32::sin(angle),
            f32::cos(angle),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    fn matrix_around_oy(&self) -> Matrix4<f32> {
        let angle = Angle::from_degree(self.angle_y).radian();

        Matrix4::new(
            f32::cos(angle),
            0.0,
            -f32::sin(angle),
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            f32::sin(angle),
            0.0,
            f32::cos(angle),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    fn matrix_around_oz(&self) -> Matrix4<f32> {
        let angle = Angle::from_degree(self.angle_z).radian();

        Matrix4::new(
            f32::cos(angle),
            f32::sin(angle),
            0.0,
            0.0,
            -f32::sin(angle),
            f32::cos(angle),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    fn matrix_offset_to_origin(&self, pivot: &Point3D) -> Matrix4<f32> {
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -pivot.x, -pivot.y,
            -pivot.z, 1.0,
        )
    }

    fn matrix_offset_to_point(&self, pivot: &Point3D) -> Matrix4<f32> {
        Matrix4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, pivot.x, pivot.y, pivot.z,
            1.0,
        )
    }

    fn are_internals_not_default(&self) -> bool {
        self.angle_x != 0.0 || self.angle_y != 0.0 || self.angle_z != 0.0
    }

    pub fn reset_position(&mut self) {
        self.angle_x = 0.0;
        self.angle_y = 0.0;
        self.angle_z = 0.0;
    }
}
