pub const ELEMENT_NAME_SETTINGS_PANEL: &str = "Settings_Panel";

pub fn show(ui: &mut egui::Ui) {
    egui::SidePanel::right(ELEMENT_NAME_SETTINGS_PANEL)
        .resizable(false)
        .default_width(250.0)
        .show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Settings");
                });

                ui.label(egui::RichText::new("SomeText"));
            });
        });
}
