use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    let panel_frame = egui::Frame::NONE
        .fill(egui::Color32::WHITE)
        .inner_margin(egui::Margin::symmetric(32, 16));

    egui::Panel::top("top_panel").frame(panel_frame).show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            ui.heading(
                egui::RichText::new(format!("{} BurnerSite", egui_phosphor::regular::LIGHTNING))
                    .strong()
                    .color(egui::Color32::from_rgb(20, 20, 25))
            );
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let _btn_rect = ui.add(
                    egui::Button::new(egui::RichText::new("Get started free").color(egui::Color32::WHITE).strong())
                        .fill(egui::Color32::from_rgb(20, 25, 45))
                );
                
                ui.add_space(16.0);
                ui.label(egui::RichText::new("Sign in").color(egui::Color32::GRAY));
                ui.add_space(16.0);
                ui.label(egui::RichText::new("Pricing").color(egui::Color32::GRAY));
                ui.add_space(16.0);
                ui.label(egui::RichText::new("Use cases").color(egui::Color32::GRAY));
                ui.add_space(16.0);
                ui.label(egui::RichText::new("How it works").color(egui::Color32::GRAY));
            });
        });
    });
}