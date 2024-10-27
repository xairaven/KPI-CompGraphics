use crate::ui::styles::colors;
use eframe::epaint::Stroke;

const AXIS_WIDTH_PX: f32 = 2.0;
const GRID_WIDTH_PX: f32 = 0.8;

pub fn axis_green() -> Stroke {
    Stroke::new(AXIS_WIDTH_PX, colors::LIME)
}

pub fn axis_red() -> Stroke {
    Stroke::new(AXIS_WIDTH_PX, colors::DARK_RED)
}

pub fn bezier_outline(width: f32) -> Stroke {
    Stroke::new(width, colors::BLACK)
}

pub fn grid_gray() -> Stroke {
    Stroke::new(GRID_WIDTH_PX, colors::GRAY)
}

pub fn model_black(width: f32) -> Stroke {
    Stroke::new(width, colors::BLACK)
}

pub fn skeleton_dark_grey(width: f32) -> Stroke {
    Stroke::new(width, colors::DARK_GRAY)
}

pub fn tangent_pink(width: f32) -> Stroke {
    Stroke::new(width, colors::PINK)
}
