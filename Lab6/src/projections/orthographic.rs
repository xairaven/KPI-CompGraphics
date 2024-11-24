use crate::geometry::line3d::Line3D;
use crate::geometry::point3d::Point3D;
use nalgebra::Matrix4;

pub struct OrthographicProjection {
    pub is_enabled: bool,
    pub mode: OrthographicMode,

    pub p: f32,
}

impl Default for OrthographicProjection {
    fn default() -> Self {
        Self {
            is_enabled: false,
            mode: OrthographicMode::ZOnZero,

            p: 0.0,
        }
    }
}

impl OrthographicProjection {
    pub fn apply_if_set(&self, model: &mut [Line3D]) {
        if self.is_enabled {
            self.process(model);
        }
    }

    fn process(&self, model: &mut [Line3D]) {
        model.iter_mut().for_each(|line| {
            self.transform_point(&mut line.start);
            self.transform_point(&mut line.end);
        })
    }

    fn transform_point(&self, point: &mut Point3D) {
        let vector = point.to_vector();

        let matrix = match &self.mode {
            OrthographicMode::XOnZero => self.matrix_x_zero(),
            OrthographicMode::XOnP => self.matrix_x_p(),
            OrthographicMode::YOnZero => self.matrix_y_zero(),
            OrthographicMode::YOnP => self.matrix_y_p(),
            OrthographicMode::ZOnZero => self.matrix_z_zero(),
            OrthographicMode::ZOnP => self.matrix_z_p(),
        };

        let result = vector * matrix;

        point.x = result.x;
        point.y = result.y;
        point.z = result.z;
    }

    fn matrix_x_zero(&self) -> Matrix4<f32> {
        Matrix4::<f32>::new(
            0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    fn matrix_y_zero(&self) -> Matrix4<f32> {
        Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    fn matrix_z_zero(&self) -> Matrix4<f32> {
        Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }

    fn matrix_x_p(&self) -> Matrix4<f32> {
        Matrix4::<f32>::new(
            0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, self.p, 0.0, 0.0, 1.0,
        )
    }

    fn matrix_y_p(&self) -> Matrix4<f32> {
        Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, self.p, 0.0, 1.0,
        )
    }

    fn matrix_z_p(&self) -> Matrix4<f32> {
        Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, self.p, 1.0,
        )
    }
}

#[derive(PartialEq)]
pub enum OrthographicMode {
    XOnZero,
    XOnP,
    YOnZero,
    YOnP,
    ZOnZero,
    ZOnP,
}
