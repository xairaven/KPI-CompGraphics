use egui::Color32;

pub const AQUA: Color32 = Color32::from_rgb(0, 255, 255);
pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
pub const BLUE: Color32 = Color32::from_rgb(63, 72, 204);
pub const DARK_RED: Color32 = Color32::from_rgb(198, 55, 57);
pub const GRAY: Color32 = Color32::from_rgb(200, 200, 200);
pub const GREEN: Color32 = Color32::from_rgb(0, 254, 0);
pub const LIME: Color32 = Color32::from_rgb(130, 245, 131);
pub const ORANGE: Color32 = Color32::from_rgb(255, 127, 39);
pub const PINK: Color32 = Color32::from_rgb(255, 0, 255);
pub const PURPLE: Color32 = Color32::from_rgb(163, 73, 164);
pub const RED: Color32 = Color32::from_rgb(255, 0, 0);
pub const YELLOW: Color32 = Color32::from_rgb(255, 242, 0);

pub fn transparent() -> Color32 {
    Color32::from_white_alpha(0)
}
