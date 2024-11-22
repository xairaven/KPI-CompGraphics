use nalgebra::Matrix4;

#[derive(Default)]
pub struct OrthographicProjection {
    pub is_enabled: bool,
}

impl OrthographicProjection {
    // Projection on Z = 0
    pub fn matrix() -> Matrix4<f32> {
        Matrix4::<f32>::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}
