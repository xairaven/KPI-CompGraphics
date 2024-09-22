use egui::Pos2;

pub struct PaintingCanvas {
    lines: Vec<Vec<Pos2>>,
    px_per_cm: f32,
}

impl Default for PaintingCanvas {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            px_per_cm: 35.0,
        }
    }
}

impl PaintingCanvas {}
