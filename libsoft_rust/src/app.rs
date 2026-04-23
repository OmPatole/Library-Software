use eframe::App;
use egui::{Context, FontData, FontDefinitions, FontFamily, Visuals};
use rusqlite::Connection;

use crate::db;
use crate::ui::{
    books::BooksState,
    faculty::FacultyState,
    main_window::{MainWindowState},
    membership::MembershipState,
    reports::ReportsState,
    settings::SettingsState,
};

pub struct LibSoftApp {
    conn: Connection,

    main_state:       MainWindowState,
    membership_state: MembershipState,
    books_state:      BooksState,
    faculty_state:    FacultyState,
    reports_state:    ReportsState,
    settings_state:   SettingsState,
}

impl LibSoftApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        let conn = db::get_db_connection().expect("Failed to initialise SQLite database");

        // Pre-load active transactions
        let active_txns = crate::db::transactions::get_active_transactions(&conn).unwrap_or_default();
        let mut main_state = MainWindowState::default();
        main_state.active_txns = active_txns;

        Self {
            conn,
            main_state,
            membership_state: MembershipState::default(),
            books_state:      BooksState::default(),
            faculty_state:    FacultyState::default(),
            reports_state:    ReportsState::default(),
            settings_state:   SettingsState::default(),
        }
    }
}

impl App for LibSoftApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Dark theme with custom accent
        let mut visuals = Visuals::dark();
        visuals.window_rounding = egui::Rounding::same(8.0);
        visuals.panel_fill = egui::Color32::from_rgb(18, 20, 28);
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 33, 45);
        ctx.set_visuals(visuals);

        // ── Menu Bar ──────────────────────────────────────────────────────────
        egui::TopBottomPanel::top("menu_bar")
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(22, 25, 37))
                .inner_margin(egui::Margin::symmetric(8.0, 6.0)))
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.label(
                        egui::RichText::new("📚 LibSoft")
                            .size(15.0)
                            .strong()
                            .color(egui::Color32::from_rgb(100, 180, 255)),
                    );
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Membership
                    ui.menu_button("Membership", |ui| {
                        if ui.button("Add Member").clicked() {
                            self.membership_state.show_add = true; ui.close_menu();
                        }
                        if ui.button("Edit Member").clicked() {
                            self.membership_state.show_edit = true; ui.close_menu();
                        }
                        if ui.button("Member Details").clicked() {
                            self.membership_state.show_details = true; ui.close_menu();
                        }
                        if ui.button("Export Member Data").clicked() {
                            self.membership_state.show_export = true; ui.close_menu();
                        }
                    });

                    // Book Details
                    ui.menu_button("Book Details", |ui| {
                        if ui.button("Add Books").clicked() {
                            if self.books_state.add_rows.is_empty() {
                                self.books_state.add_rows.push(Default::default());
                            }
                            self.books_state.show_add = true; ui.close_menu();
                        }
                        if ui.button("Edit Book").clicked() {
                            self.books_state.show_edit = true; ui.close_menu();
                        }
                        if ui.button("Book Detail").clicked() {
                            self.books_state.show_detail = true; ui.close_menu();
                        }
                        if ui.button("Stock Verification").clicked() {
                            self.books_state.show_stock = true; ui.close_menu();
                        }
                        if ui.button("Import Book Data").clicked() {
                            self.books_state.show_import = true; ui.close_menu();
                        }
                    });

                    // Reports
                    ui.menu_button("Reports", |ui| {
                        if ui.button("Generate Report").clicked() {
                            self.reports_state.show_generate = true; ui.close_menu();
                        }
                    });

                    // Settings
                    ui.menu_button("Settings", |ui| {
                        if ui.button("Activation Settings").clicked() {
                            self.settings_state.show_activation = true; ui.close_menu();
                        }
                        if ui.button("Edit Branch").clicked() {
                            self.settings_state.show_branch = true; ui.close_menu();
                        }
                    });

                    // Faculty Module
                    ui.menu_button("Faculty Module", |ui| {
                        if ui.button("Add Faculty").clicked() {
                            self.faculty_state.show_add = true; ui.close_menu();
                        }
                        if ui.button("Edit Faculty").clicked() {
                            self.faculty_state.show_edit = true; ui.close_menu();
                        }
                        if ui.button("Faculty Details").clicked() {
                            self.faculty_state.show_details = true; ui.close_menu();
                        }
                    });

                    // Connection
                    ui.menu_button("Connection", |ui| {
                        ui.label("Database: libsoft.db (local)");
                        ui.label("Status: ✔ Connected");
                    });
                });
            });

        // ── Modal windows ─────────────────────────────────────────────────────
        crate::ui::membership::show(ctx, &mut self.membership_state, &self.conn);
        crate::ui::books::show(ctx, &mut self.books_state, &self.conn);
        crate::ui::faculty::show(ctx, &mut self.faculty_state, &self.conn);
        crate::ui::reports::show(ctx, &mut self.reports_state, &self.conn);
        crate::ui::settings::show(ctx, &mut self.settings_state, &self.conn);

        // ── Central Panel: Main Dashboard ──────────────────────────────────────
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::main_window::render(ctx, ui, &mut self.main_state, &self.conn);
        });
    }
}

fn setup_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();

    // Try to load JetBrainsMono Nerd Font — fall back to system defaults if absent
    if let Ok(font_bytes) = std::fs::read("assets/JetBrainsMonoNerdFont-Regular.ttf") {
        fonts.font_data.insert(
            "JetBrainsMono".into(),
            FontData::from_owned(font_bytes),
        );
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "JetBrainsMono".into());
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "JetBrainsMono".into());
    }

    ctx.set_fonts(fonts);

    // Global font sizes
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(13.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(13.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(16.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(13.0, FontFamily::Monospace),
    );
    ctx.set_style(style);
}
