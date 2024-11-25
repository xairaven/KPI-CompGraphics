use eframe::epaint::Color32;

pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
pub const BLUE: Color32 = Color32::from_rgb(0, 0, 255);
pub const DARK_RED: Color32 = Color32::from_rgb(198, 55, 57);
pub const DARK_GRAY: Color32 = Color32::from_rgb(127, 127, 127);
pub const GRAY: Color32 = Color32::from_rgb(200, 200, 200);
pub const GREEN: Color32 = Color32::from_rgb(0, 255, 0);
pub const LIME: Color32 = Color32::from_rgb(50, 205, 50);
pub const PINK: Color32 = Color32::from_rgb(255, 0, 255);
pub const RED: Color32 = Color32::from_rgb(255, 0, 0);
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const YELLOW: Color32 = Color32::from_rgb(255, 242, 0);

pub fn transparent() -> Color32 {
    Color32::from_white_alpha(0)
}
