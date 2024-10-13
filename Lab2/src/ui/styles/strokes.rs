use crate::ui::styles::colors;
use eframe::epaint::Stroke;

pub fn axis_green() -> Stroke {
    Stroke::new(2.0, colors::LIME)
}

pub fn axis_red() -> Stroke {
    Stroke::new(2.0, colors::DARK_RED)
}

pub fn tangent_blue() -> Stroke {
    Stroke::new(2.0, colors::BLUE)
}

pub fn grid_gray() -> Stroke {
    Stroke::new(0.8, colors::GRAY)
}

pub fn model_aqua() -> Stroke {
    Stroke::new(2.0, colors::AQUA)
}

pub fn model_black() -> Stroke {
    Stroke::new(2.0, colors::BLACK)
}

pub fn model_orange() -> Stroke {
    Stroke::new(2.0, colors::ORANGE)
}

pub fn model_purple() -> Stroke {
    Stroke::new(2.0, colors::PURPLE)
}

pub fn model_yellow() -> Stroke {
    Stroke::new(2.0, colors::YELLOW)
}

pub fn normal_aqua() -> Stroke {
    Stroke::new(2.0, colors::AQUA)
}
