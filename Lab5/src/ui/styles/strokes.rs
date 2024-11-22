use crate::ui::styles::colors;
use eframe::epaint::Stroke;

const AXIS_WIDTH_PX: f32 = 2.0;

pub fn axis_blue() -> Stroke {
    Stroke::new(AXIS_WIDTH_PX, colors::BLUE)
}
pub fn axis_green() -> Stroke {
    Stroke::new(AXIS_WIDTH_PX, colors::LIME)
}

pub fn axis_red() -> Stroke {
    Stroke::new(AXIS_WIDTH_PX, colors::DARK_RED)
}

pub fn model_black(width: f32) -> Stroke {
    Stroke::new(width, colors::BLACK)
}
