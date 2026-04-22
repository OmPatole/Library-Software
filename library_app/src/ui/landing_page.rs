use eframe::egui;

pub fn show(ui: &mut egui::Ui) {
    let central_frame = egui::Frame::NONE.fill(egui::Color32::WHITE);

    egui::CentralPanel::default().frame(central_frame).show_inside(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                
                // Pill Badge
                let pill_badge = egui::Frame::NONE
                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(230, 230, 230)))
                    .corner_radius(egui::CornerRadius::same(16))
                    .inner_margin(egui::Margin::symmetric(16, 6));
                
                pill_badge.show(ui, |ui| {
                    ui.label(egui::RichText::new(format!("{} No builders • No drag-drop • Just publish", egui_phosphor::regular::CHECK_CIRCLE)).color(egui::Color32::from_rgb(100, 100, 100)).size(13.0));
                });

                ui.add_space(40.0);
                
                // Main Hero text
                ui.label(
                    egui::RichText::new("Build your launch page")
                        .size(58.0)
                        .strong()
                        .color(egui::Color32::from_rgb(20, 25, 45))
                );
                ui.label(
                    egui::RichText::new("in under 60 seconds.")
                        .size(58.0)
                        .color(egui::Color32::from_rgb(180, 185, 195))
                );
                
                ui.add_space(20.0);
                
                // Subtitle
                ui.label(
                    egui::RichText::new("Pick a template. Edit inline. Set an expiry. Publish. BurnerSite gives you a real, live")
                        .size(19.0)
                        .color(egui::Color32::from_rgb(140, 140, 140))
                );
                ui.label(
                    egui::RichText::new("URL that auto-deletes when you're done — no cleanup, no complexity.")
                        .size(19.0)
                        .color(egui::Color32::from_rgb(140, 140, 140))
                );
                
                ui.add_space(45.0);
                
                // Call to Action Buttons
                ui.horizontal(|ui| {
                    // Centering layout hack
                    ui.add_space(ui.available_width() / 2.0 - 170.0);
                    
                    let _start_btn = ui.add_sized(
                        [160.0, 50.0],
                        egui::Button::new(egui::RichText::new(format!("Start for free {}", egui_phosphor::regular::ARROW_RIGHT)).color(egui::Color32::WHITE).size(16.0).strong())
                            .fill(egui::Color32::from_rgb(20, 25, 45))
                    );

                    ui.add_space(10.0);

                    let _pass_btn = ui.add_sized(
                        [160.0, 50.0],
                        egui::Button::new(egui::RichText::new(format!("{} Buy a pass — $4.99", egui_phosphor::regular::TICKET)).color(egui::Color32::from_rgb(40, 40, 60)).size(16.0))
                            .fill(egui::Color32::WHITE)
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(230, 230, 230)))
                    );
                });

                ui.add_space(25.0);
                ui.label(egui::RichText::new(format!("{check} No credit card    {check} Free forever plan    {check} 60-second publish    {check} Auto-expires", check=egui_phosphor::regular::CHECK)).color(egui::Color32::from_rgb(150, 150, 150)));
                
                ui.add_space(60.0);
                
                // Divider
                let sep_rect = egui::Rect::from_x_y_ranges(
                    0.0..=ui.available_width(),
                    ui.min_rect().bottom()..=ui.min_rect().bottom() + 1.0,
                );
                ui.painter().rect_filled(sep_rect, 0.0, egui::Color32::from_rgb(240, 240, 240));
                ui.add_space(60.0);
                
                // Stats Section
                ui.horizontal(|ui| {
                    ui.columns(3, |columns| {
                        columns[0].vertical_centered(|ui| {
                            ui.heading(egui::RichText::new("10,000+").size(34.0).strong().color(egui::Color32::from_rgb(20, 20, 25)));
                            ui.label(egui::RichText::new("sites published").color(egui::Color32::from_rgb(150, 150, 150)));
                        });
                        columns[1].vertical_centered(|ui| {
                            ui.heading(egui::RichText::new("60 sec").size(34.0).strong().color(egui::Color32::from_rgb(20, 20, 25)));
                            ui.label(egui::RichText::new("average publish time").color(egui::Color32::from_rgb(150, 150, 150)));
                        });
                        columns[2].vertical_centered(|ui| {
                            ui.heading(egui::RichText::new("100%").size(34.0).strong().color(egui::Color32::from_rgb(20, 20, 25)));
                            ui.label(egui::RichText::new("auto-expiry, no cleanup").color(egui::Color32::from_rgb(150, 150, 150)));
                        });
                    });
                });

                ui.add_space(80.0);
                
                // How it works
                ui.label(egui::RichText::new("HOW IT WORKS").color(egui::Color32::from_rgb(180, 180, 180)).strong());
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("From zero to live in four steps").size(34.0).strong().color(egui::Color32::from_rgb(20, 20, 25)));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("No account setup wizards. No onboarding. Just build.").color(egui::Color32::from_rgb(150, 150, 150)).size(16.0));
                
                ui.add_space(100.0);
            });
        });
    });
}