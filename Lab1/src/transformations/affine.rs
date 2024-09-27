#[derive(Default)]
pub struct Affine {
    pub xx: f32,
    pub xy: f32,
    pub yx: f32,
    pub yy: f32,
    pub zero_x: f32,
    pub zero_y: f32,

    pub scaling_x: f32,
    pub scaling_y: f32,
}
