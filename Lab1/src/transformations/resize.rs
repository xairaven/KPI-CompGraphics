pub struct Resize {
    pub length_ab: f32,
    pub length_bc: f32,
    pub length_cd: f32,
    pub length_de: f32,
    pub length_ef: f32,
    pub length_fg: f32,
    pub length_gh: f32,
    pub length_ah: f32,

    pub radius_i: f32,
    pub radius_j: f32,
}

impl Default for Resize {
    fn default() -> Self {
        Self {
            length_ab: 110.0,
            length_bc: 37.0,
            length_cd: 35.0,
            length_de: 15.0,
            length_ef: 40.0,
            length_fg: 15.0,
            length_gh: 35.0,
            length_ah: 37.0,
            radius_i: 7.5,
            radius_j: 7.5,
        }
    }
}
