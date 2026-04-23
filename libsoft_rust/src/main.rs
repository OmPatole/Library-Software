mod app;
mod db;
mod models;
mod ui;
mod utils;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("LibSoft — Library Management System")
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "LibSoft",
        options,
        Box::new(|cc| Box::new(app::LibSoftApp::new(cc))),
    )
}
