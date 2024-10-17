use crate::ui::styles::colors;
use eframe::epaint::Stroke;

const AXIS_WIDTH: f32 = 2.0;
const MODEL_WIDTH: f32 = 2.0;
const GRID_WIDTH: f32 = 0.8;

pub fn axis_green() -> Stroke {
    Stroke::new(AXIS_WIDTH, colors::LIME)
}

pub fn axis_red() -> Stroke {
    Stroke::new(AXIS_WIDTH, colors::DARK_RED)
}

pub fn bezier_outline(width: f32) -> Stroke {
    Stroke::new(width, colors::BLACK)
}

pub fn grid_gray() -> Stroke {
    Stroke::new(GRID_WIDTH, colors::GRAY)
}

pub fn model_black() -> Stroke {
    Stroke::new(MODEL_WIDTH, colors::BLACK)
}

pub fn skeleton_dark_grey(width: f32) -> Stroke {
    Stroke::new(width, colors::DARK_GRAY)
}
