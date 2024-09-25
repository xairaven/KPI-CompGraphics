#[derive(Default)]
pub struct Projective {
    pub xx: f32,
    pub xy: f32,
    pub wx: f32,
    pub yx: f32,
    pub yy: f32,
    pub wy: f32,
    pub zero_x: f32,
    pub zero_y: f32,
    pub w_zero: f32,
}
