use egui::{emath, DragValue, Ui};
use std::ops::RangeInclusive;

pub fn label_centered_with_drag<Num: emath::Numeric>(
    ui: &mut Ui, label: &str, variable: &mut f32, speed: impl Into<f64>, range: RangeInclusive<Num>,
) {
    ui.vertical_centered(|ui| {
        ui.label(label);
    });
    ui.add(DragValue::new(variable).speed(speed).range(range));
}
