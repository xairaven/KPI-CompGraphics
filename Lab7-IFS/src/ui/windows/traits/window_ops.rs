use crate::context::Context;

pub trait WindowOps {
    fn show(&mut self, ui: &egui::Ui, context: &mut Context);
    fn is_closed(&self) -> bool;
}
