use egui::Pos2;

pub struct Model {
    pub a: Pos2,
    pub b: Pos2,
    pub c: Pos2,
    pub d: Pos2,
    pub e: Pos2,
    pub f: Pos2,
    pub g: Pos2,
    pub h: Pos2,

    pub i: Pos2,
    pub j: Pos2,
    pub i_radius: f32,
    pub j_radius: f32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            a: Pos2::from([10.0, 120.0]),
            b: Pos2::from([10.0, 10.0]),
            c: Pos2::from([47.0, 10.0]),
            d: Pos2::from([47.0, 45.0]),
            e: Pos2::from([32.0, 45.0]),
            f: Pos2::from([32.0, 85.0]),
            g: Pos2::from([47.0, 85.0]),
            h: Pos2::from([47.0, 120.0]),

            i: Pos2::from([20.0, 20.0]),
            i_radius: 7.5,
            j: Pos2::from([20.0, 110.0]),
            j_radius: 7.5,
        }
    }
}
