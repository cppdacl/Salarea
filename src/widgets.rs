use eframe::egui::{self, Color32, RichText};
use crate::style::{gold_dim, mono};

pub fn lv(ui: &mut egui::Ui, label: &str, val: &str, color: Color32) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("{:<22}", label)).font(mono(12.5)).color(gold_dim()));
        ui.label(RichText::new(val).font(mono(12.5)).color(color));
    });
}

pub fn mrow(ui: &mut egui::Ui, label: &str, amount: f64, color: Color32) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(format!("{:<30}", label)).font(mono(12.0)).color(gold_dim()));
        ui.label(RichText::new(format!("P {:>12.2}", amount)).font(mono(12.0)).color(color));
    });
}

pub fn sep(ui: &mut egui::Ui) {
    ui.add_space(3.0);
    ui.separator();
    ui.add_space(3.0);
}