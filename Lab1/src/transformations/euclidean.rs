pub struct Euclidean {
    pub rotation_x: f32,
    pub rotation_y: f32,
    pub offset_angle: f32,
}

impl Default for Euclidean {
    fn default() -> Self {
        Self {
            rotation_x: 0.0,
            rotation_y: 0.0,
            offset_angle: 0.0,
        }
    }
}
