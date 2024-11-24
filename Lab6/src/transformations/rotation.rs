use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use crate::math::angle::Angle;
use nalgebra::Matrix4;

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
    pub fn apply(&mut self, model: &mut [Line3D], pivot: Point3D) {
        if self.is_applied {
            self.static_angle_deg_x += self.angle_deg_x;
            self.static_angle_deg_y += self.angle_deg_y;
            self.static_angle_deg_z += self.angle_deg_z;
            self.is_applied = false;
        }
        if self.are_statics_not_default() {
            self.process(model, pivot);
        }
    }

    pub fn process(&self, model: &mut [Line3D], pivot: Point3D) {
        let matrix_to_origin = self.matrix_offset_to_origin(&pivot);
        let matrix_from_origin = self.matrix_offset_to_point(&pivot);
        let matrix_ox = self.matrix_around_ox();
        let matrix_oy = self.matrix_around_oy();
        let matrix_oz = self.matrix_around_oz();

        let result_matrix =
            matrix_to_origin * matrix_ox * matrix_oy * matrix_oz * matrix_from_origin;

        model.iter_mut().for_each(|line| {
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
        let angle = Angle::from_degree(self.static_angle_deg_x).radian();

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
        let angle = Angle::from_degree(self.static_angle_deg_y).radian();

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
        let angle = Angle::from_degree(self.static_angle_deg_z).radian();

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
