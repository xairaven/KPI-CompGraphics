use crate::ui::styles::colors;
use eframe::epaint::Stroke;

const MODEL_WIDTH: f32 = 2.0;
const GRID_WIDTH: f32 = 0.8;

pub fn model_black() -> Stroke {
    Stroke::new(MODEL_WIDTH, colors::BLACK)
}

pub fn grid_gray() -> Stroke {
    Stroke::new(GRID_WIDTH, colors::GRAY)
}
