mod app;
mod compute;
mod constants;
mod style;
mod widgets;

use app::SalareaApp;

fn main() -> eframe::Result {
    eframe::run_native(
        "Salarea",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title("Salarea")
                .with_inner_size([580.0, 700.0])
                .with_resizable(true),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(SalareaApp::default()))),
    )
}