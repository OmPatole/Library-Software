use egui::{Context, Ui, RichText, Color32, Grid, ScrollArea, Frame};
use rusqlite::Connection;
use crate::models::transaction::Transaction;
use crate::models::book::Book;
use crate::db::{books, transactions};

/// Which tab is active in the centre panel
#[derive(PartialEq, Clone, Default)]
pub enum TxnTab { #[default] Issue, Return, Renew }

#[derive(Default)]
pub struct MainWindowState {
    // ── Left: Quick Search ─────────────────────────────────────────────────────
    pub qs_accession: String,
    pub qs_book_name: String,
    pub qs_author:    String,
    pub qs_call_no:   String,
    pub qs_status:    String,
    pub qs_publisher: String,
    pub qs_results:   Vec<Book>,

    // ── Centre: Transactions ───────────────────────────────────────────────────
    pub active_tab: TxnTab,
    pub txn_accession:   String,
    pub txn_book_name:   String,
    pub txn_author:      String,
    pub txn_call_no:     String,
    pub txn_issue_date:  String,
    pub txn_return_date: String,
    pub txn_issued_by:   String,   // member/faculty ID
    pub txn_msg:         String,

    // ── Right: Active Transactions grid ───────────────────────────────────────
    pub active_txns: Vec<Transaction>,

    // ── Bottom: Global Search ─────────────────────────────────────────────────
    pub global_search:  String,
    pub global_results: Vec<Book>,
}

pub fn render(ctx: &Context, ui: &mut Ui, state: &mut MainWindowState, conn: &Connection) {
    // Fill today's date as default for Issue/Return dates
    if state.txn_issue_date.is_empty() {
        state.txn_issue_date = today();
    }
    if state.txn_return_date.is_empty() {
        state.txn_return_date = in_days(14);
    }

    let panel_height = ui.available_height() - 60.0; // reserve for global search bar

    ui.horizontal(|ui| {
        ui.set_min_height(panel_height);

        // ── LEFT: Quick Search ─────────────────────────────────────────────────
        Frame::group(ui.style())
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.set_min_width(200.0);
                ui.set_max_width(210.0);
                ui.vertical(|ui| {
                    ui.label(RichText::new("📖  Quick Search").strong().size(13.0));
                    ui.separator();
                    Grid::new("qs_grid").num_columns(1).spacing([4.0, 6.0]).show(ui, |ui| {
                        macro_rules! qs_field { ($label:expr, $field:expr) => {
                            ui.label($label);
                            ui.end_row();
                            ui.text_edit_singleline($field);
                            ui.end_row();
                        }}
                        qs_field!("Accession No.", &mut state.qs_accession);
                        qs_field!("Book Name",     &mut state.qs_book_name);
                        qs_field!("Author",        &mut state.qs_author);
                        qs_field!("Call No.",      &mut state.qs_call_no);
                        qs_field!("Issue Status",  &mut state.qs_status);
                        qs_field!("Publisher",     &mut state.qs_publisher);
                    });
                    if ui.button("  🔍  Search  ").clicked() {
                        let q = build_qs_query(state);
                        state.qs_results = books::search_books(conn, &q).unwrap_or_default();
                    }
                    ui.add_space(4.0);
                    ScrollArea::vertical().id_source("qs_results").max_height(200.0).show(ui, |ui| {
                        for b in &state.qs_results {
                            ui.group(|ui| {
                                ui.label(RichText::new(&b.title).strong());
                                ui.label(format!("{} — {}", b.author, b.accession_no));
                                let color = if b.status == "Available" {
                                    Color32::from_rgb(30, 160, 80)
                                } else {
                                    Color32::from_rgb(200, 60, 30)
                                };
                                ui.colored_label(color, &b.status);
                            });
                            ui.add_space(2.0);
                        }
                    });
                });
            });

        ui.separator();

        // ── CENTRE: Transactions ───────────────────────────────────────────────
        Frame::group(ui.style())
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.set_min_width(300.0);
                ui.vertical(|ui| {
                    // Tab bar
                    ui.horizontal(|ui| {
                        tab_btn(ui, &mut state.active_tab, TxnTab::Issue,  "  Issue  ");
                        tab_btn(ui, &mut state.active_tab, TxnTab::Return, "  Return  ");
                        tab_btn(ui, &mut state.active_tab, TxnTab::Renew,  "  Renew  ");
                    });
                    ui.separator();
                    Grid::new("txn_grid").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
                        macro_rules! txn_row { ($l:expr, $f:expr) => {
                            ui.label($l); ui.text_edit_singleline($f); ui.end_row();
                        }}
                        txn_row!("Accession No.:",    &mut state.txn_accession);
                        txn_row!("Book Name:",         &mut state.txn_book_name);
                        txn_row!("Author Name:",       &mut state.txn_author);
                        txn_row!("Call No.:",          &mut state.txn_call_no);
                        txn_row!("Issue Date:",        &mut state.txn_issue_date);
                        txn_row!("Exp. Return Date:",  &mut state.txn_return_date);
                        txn_row!("Issued By (ID):",    &mut state.txn_issued_by);
                    });
                    ui.add_space(6.0);

                    // Auto-fill book fields from accession number
                    if ui.button("  🔎  Lookup Book  ").clicked() {
                        if let Ok(Some(b)) = books::get_book(conn, &state.txn_accession) {
                            state.txn_book_name = b.title;
                            state.txn_author    = b.author;
                            state.txn_call_no   = b.call_no;
                        }
                    }

                    ui.add_space(4.0);
                    let action_label = match state.active_tab {
                        TxnTab::Issue  => "  ✔  Issue Book  ",
                        TxnTab::Return => "  ✔  Return Book  ",
                        TxnTab::Renew  => "  ✔  Renew Book  ",
                    };
                    if ui.button(action_label).clicked() {
                        let msg = do_transaction(state, conn);
                        state.txn_msg = msg;
                        // Refresh active list
                        state.active_txns = transactions::get_active_transactions(conn).unwrap_or_default();
                    }
                    if !state.txn_msg.is_empty() {
                        ui.colored_label(Color32::DARK_GREEN, &state.txn_msg);
                    }
                });
            });

        ui.separator();

        // ── RIGHT: Active Transactions ─────────────────────────────────────────
        Frame::group(ui.style())
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("📋  Active Issues").strong().size(13.0));
                        if ui.small_button("⟳ Refresh").clicked() {
                            state.active_txns = transactions::get_active_transactions(conn).unwrap_or_default();
                        }
                    });
                    ui.separator();
                    ScrollArea::vertical().id_source("active_txns").show(ui, |ui| {
                        Grid::new("active_grid")
                            .num_columns(4)
                            .spacing([10.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
                                for h in ["Acc. No","Name","Status","Return Date"] {
                                    ui.label(RichText::new(h).strong());
                                }
                                ui.end_row();
                                for t in &state.active_txns {
                                    ui.label(&t.accession_no);
                                    ui.label(&t.user_name);
                                    let color = if t.status == "Issued" {
                                        Color32::from_rgb(200, 100, 30)
                                    } else {
                                        Color32::from_rgb(100, 100, 200)
                                    };
                                    ui.colored_label(color, &t.status);
                                    ui.label(&t.expected_return_date);
                                    ui.end_row();
                                }
                            });
                    });
                });
            });
    });

    ui.add_space(6.0);
    ui.separator();

    // ── BOTTOM: Global Search ──────────────────────────────────────────────────
    ui.horizontal(|ui| {
        ui.label(RichText::new("🔍").size(16.0));
        let response = ui.add(
            egui::TextEdit::singleline(&mut state.global_search)
                .hint_text("Global search — books by title, author, accession no., publisher…")
                .desired_width(f32::INFINITY),
        );
        if response.changed() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
            if state.global_search.is_empty() {
                state.global_results.clear();
            } else {
                state.global_results = books::search_books(conn, &state.global_search).unwrap_or_default();
            }
        }
    });
    if !state.global_results.is_empty() {
        ui.add_space(4.0);
        ScrollArea::horizontal().id_source("global_results_scroll").show(ui, |ui| {
            Grid::new("global_results_grid")
                .num_columns(5)
                .spacing([12.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for h in ["Acc. No","Title","Author","Publisher","Status"] {
                        ui.label(RichText::new(h).strong());
                    }
                    ui.end_row();
                    for b in &state.global_results {
                        ui.label(&b.accession_no);
                        ui.label(&b.title);
                        ui.label(&b.author);
                        ui.label(&b.publisher);
                        let color = if b.status == "Available" {
                            Color32::from_rgb(30, 160, 80)
                        } else {
                            Color32::from_rgb(200, 60, 30)
                        };
                        ui.colored_label(color, &b.status);
                        ui.end_row();
                    }
                });
        });
    }
}

fn tab_btn(ui: &mut Ui, current: &mut TxnTab, target: TxnTab, label: &str) {
    let selected = *current == target;
    let text = if selected {
        RichText::new(label).strong().color(Color32::from_rgb(30, 120, 220))
    } else {
        RichText::new(label)
    };
    if ui.selectable_label(selected, text).clicked() {
        *current = target;
    }
}

fn build_qs_query(state: &MainWindowState) -> String {
    // combine the first non-empty field for searching
    [&state.qs_accession, &state.qs_book_name, &state.qs_author,
     &state.qs_call_no, &state.qs_publisher]
        .iter()
        .find(|s| !s.is_empty())
        .map(|s| s.as_str())
        .unwrap_or("")
        .to_string()
}

fn do_transaction(state: &mut MainWindowState, conn: &Connection) -> String {
    if state.txn_accession.is_empty() || state.txn_issued_by.is_empty() {
        return "Fill in Accession No. and User ID.".into();
    }
    match state.active_tab {
        TxnTab::Issue => {
            let t = crate::models::transaction::Transaction {
                accession_no:           state.txn_accession.clone(),
                user_id:                state.txn_issued_by.clone(),
                issue_date:             state.txn_issue_date.clone(),
                expected_return_date:   state.txn_return_date.clone(),
                status:                 "Issued".into(),
                ..Default::default()
            };
            match transactions::issue_book(conn, &t) {
                Ok(_)  => "Book issued successfully.".into(),
                Err(e) => format!("Error: {e}"),
            }
        }
        TxnTab::Return => {
            match transactions::return_book(conn, &state.txn_accession, &today()) {
                Ok(_)  => "Book returned successfully.".into(),
                Err(e) => format!("Error: {e}"),
            }
        }
        TxnTab::Renew => {
            match transactions::renew_book(conn, &state.txn_accession, &state.txn_return_date) {
                Ok(_)  => "Book renewed successfully.".into(),
                Err(e) => format!("Error: {e}"),
            }
        }
    }
}

fn today() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let days = (secs / 86400) as i64;
    let (y, m, d) = days_to_ymd(days + 719162);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn in_days(n: i64) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let days = (secs / 86400) as i64 + n;
    let (y, m, d) = days_to_ymd(days + 719162);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

fn days_to_ymd(z: i64) -> (i64, i64, i64) {
    // Civil calendar from Howard Hinnant
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
