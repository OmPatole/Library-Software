use eframe::egui;

#[derive(Default)]
pub struct LibraryApp {}

impl eframe::App for LibraryApp {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut style = (*ctx.global_style()).clone();
        style.spacing.item_spacing = egui::vec2(12.0, 16.0);
        style.spacing.button_padding = egui::vec2(16.0, 8.0);
        style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(8);
        style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(8);
        style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(8);
        ctx.set_global_style(style);

        #[allow(deprecated)]
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::top_nav::show(ui);
            crate::ui::landing_page::show(ui);
        });
    }
}