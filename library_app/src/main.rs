#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;

mod app;
mod ui;

use app::LibraryApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 900.0])
            .with_decorations(true),
        ..Default::default()
    };

    eframe::run_native(
        "Modern UI App",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Ok(Box::<LibraryApp>::default())
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    ctx.set_fonts(fonts);
}
