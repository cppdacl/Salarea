use eframe::egui::{Color32, FontFamily, FontId};

pub fn gold() -> Color32 { Color32::from_rgb(255, 180, 0) }
pub fn gold_dim() -> Color32 { Color32::from_rgb(160, 110, 0) }
pub fn bg() -> Color32 { Color32::from_rgb(12, 10, 5) }
pub fn green() -> Color32 { Color32::from_rgb(80, 210, 100) }
pub fn red() -> Color32 { Color32::from_rgb(220, 60, 60) }
pub fn mono(size: f32) -> FontId { FontId::new(size, FontFamily::Monospace) }