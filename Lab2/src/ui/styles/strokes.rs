use crate::ui::styles::colors;
use eframe::epaint::Stroke;

pub fn axis_green() -> Stroke {
    Stroke::new(2.0, colors::LIME)
}

pub fn axis_red() -> Stroke {
    Stroke::new(2.0, colors::DARK_RED)
}

pub fn grid_gray() -> Stroke {
    Stroke::new(0.8, colors::GRAY)
}
