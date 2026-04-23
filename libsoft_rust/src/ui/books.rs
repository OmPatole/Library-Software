use egui::{Context, Window, Grid, RichText, Color32, ScrollArea};
use rusqlite::Connection;
use crate::models::book::Book;
use crate::db::books;

#[derive(Default)]
pub struct BooksState {
    // Add Books — bulk entry rows
    pub show_add: bool,
    pub add_rows: Vec<Book>,

    // Edit Book
    pub show_edit: bool,
    pub edit_search: String,
    pub edit_form: Book,
    pub edit_msg: String,

    // Book Detail (read-only)
    pub show_detail: bool,
    pub detail_search: String,
    pub detail_book: Option<Book>,

    // Stock Verification
    pub show_stock: bool,
    pub stock_scan_input: String,
    pub stock_verified: Vec<String>, // accession numbers scanned
    pub stock_total: usize,
    pub stock_msg: String,

    // Import
    pub show_import: bool,
    pub import_path: String,
    pub import_msg: String,
}

fn book_form_fields(ui: &mut egui::Ui, b: &mut Book, prefix: &str) {
    Grid::new(format!("{prefix}_bookgrid"))
        .num_columns(2)
        .spacing([12.0, 8.0])
        .show(ui, |ui| {
            ui.label("Accession No.:"); ui.text_edit_singleline(&mut b.accession_no); ui.end_row();
            ui.label("Call No.:");      ui.text_edit_singleline(&mut b.call_no);      ui.end_row();
            ui.label("Book Name:");     ui.text_edit_singleline(&mut b.title);        ui.end_row();
            ui.label("Author Name:");   ui.text_edit_singleline(&mut b.author);       ui.end_row();
            ui.label("Branch:");        ui.text_edit_singleline(&mut b.branch);       ui.end_row();
            ui.label("Publisher:");     ui.text_edit_singleline(&mut b.publisher);    ui.end_row();
            ui.label("Price:");
            let prev_price = b.price;
            let mut price_str = if b.price == 0.0 { String::new() } else { b.price.to_string() };
            ui.text_edit_singleline(&mut price_str);
            b.price = price_str.parse::<f64>().unwrap_or(prev_price);
            ui.end_row();
            ui.label("Bill No.:"); ui.text_edit_singleline(&mut b.bill_no); ui.end_row();
        });
}

pub fn show(ctx: &Context, state: &mut BooksState, conn: &Connection) {
    // ── Add Books (bulk grid) ─────────────────────────────────────────────────
    let mut show_add = state.show_add;
    Window::new("Add Books")
        .open(&mut show_add)
        .resizable(true)
        .min_width(760.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Add Books — Bulk Entry").color(Color32::from_rgb(30, 120, 220)));
            ui.separator();
            if ui.button("+ Add Row").clicked() {
                state.add_rows.push(Book { status: "Available".into(), ..Default::default() });
            }
            ui.add_space(4.0);
            ScrollArea::vertical().max_height(420.0).show(ui, |ui| {
                Grid::new("add_bulk_grid")
                    .num_columns(9)
                    .spacing([6.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        for h in ["Acc. No","Call No","Title","Author","Branch","Publisher","Price","Bill",""] {
                            ui.label(RichText::new(h).strong());
                        }
                        ui.end_row();
                        let mut to_remove = None;
                        for (i, b) in state.add_rows.iter_mut().enumerate() {
                            ui.text_edit_singleline(&mut b.accession_no);
                            ui.text_edit_singleline(&mut b.call_no);
                            ui.text_edit_singleline(&mut b.title);
                            ui.text_edit_singleline(&mut b.author);
                            ui.text_edit_singleline(&mut b.branch);
                            ui.text_edit_singleline(&mut b.publisher);
                            let mut price_str = if b.price == 0.0 { String::new() } else { b.price.to_string() };
                            ui.text_edit_singleline(&mut price_str);
                            b.price = price_str.parse::<f64>().unwrap_or(0.0);
                            ui.text_edit_singleline(&mut b.bill_no);
                            if ui.button("✕").clicked() { to_remove = Some(i); }
                            ui.end_row();
                        }
                        if let Some(i) = to_remove { state.add_rows.remove(i); }
                    });
            });
            ui.separator();
            if ui.button("  ✔  Save All  ").clicked() {
                for b in &state.add_rows {
                    if !b.accession_no.is_empty() {
                        let _ = books::add_book(conn, b);
                    }
                }
                state.add_rows.clear();
                state.show_add = false;
            }
        });
    state.show_add = show_add;

    // ── Edit Book ─────────────────────────────────────────────────────────────
    let mut show_edit = state.show_edit;
    Window::new("Edit Book")
        .open(&mut show_edit)
        .resizable(true)
        .min_width(440.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Edit Book").color(Color32::from_rgb(200, 120, 30)));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Accession No.:");
                ui.text_edit_singleline(&mut state.edit_search);
                if ui.button("Load").clicked() {
                    match books::get_book(conn, &state.edit_search) {
                        Ok(Some(b)) => { state.edit_form = b; state.edit_msg = String::new(); }
                        _ => state.edit_msg = "Book not found.".into(),
                    }
                }
            });
            if !state.edit_msg.is_empty() {
                ui.colored_label(Color32::RED, &state.edit_msg);
            }
            ui.separator();
            book_form_fields(ui, &mut state.edit_form, "edit");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Status:");
                egui::ComboBox::from_id_source("edit_status")
                    .selected_text(&state.edit_form.status)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut state.edit_form.status, "Available".into(), "Available");
                        ui.selectable_value(&mut state.edit_form.status, "Issued".into(), "Issued");
                        ui.selectable_value(&mut state.edit_form.status, "Lost".into(), "Lost");
                    });
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui.button("  ✔  Save  ").clicked() && !state.edit_form.accession_no.is_empty() {
                    let _ = books::add_book(conn, &state.edit_form);
                    state.edit_msg = "Saved.".into();
                }
                if ui.button("  🗑  Delete  ").clicked() && !state.edit_form.accession_no.is_empty() {
                    let _ = books::delete_book(conn, &state.edit_form.accession_no);
                    state.edit_form = Book::default();
                    state.edit_msg = "Deleted.".into();
                }
            });
        });
    state.show_edit = show_edit;

    // ── Book Detail ────────────────────────────────────────────────────────────
    let mut show_detail = state.show_detail;
    Window::new("Book Detail")
        .open(&mut show_detail)
        .resizable(false)
        .min_width(360.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Book Detail").color(Color32::from_rgb(30, 160, 80)));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Accession No.:");
                ui.text_edit_singleline(&mut state.detail_search);
                if ui.button("Search").clicked() {
                    state.detail_book = books::get_book(conn, &state.detail_search).ok().flatten();
                }
            });
            ui.separator();
            if let Some(b) = &state.detail_book {
                Grid::new("book_detail_grid").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                    macro_rules! row { ($l:expr, $v:expr) => {
                        ui.label($l); ui.label($v); ui.end_row();
                    }}
                    row!("Accession No.:", &b.accession_no);
                    row!("Call No.:",      &b.call_no);
                    row!("Title:",         &b.title);
                    row!("Author:",        &b.author);
                    row!("Branch:",        &b.branch);
                    row!("Publisher:",     &b.publisher);
                    row!("Price:",         &format!("₹ {:.2}", b.price));
                    row!("Bill No.:",      &b.bill_no);
                    row!("Status:",        &b.status);
                });
            }
        });
    state.show_detail = show_detail;

    // ── Stock Verification ────────────────────────────────────────────────────
    let mut show_stock = state.show_stock;
    Window::new("Stock Verification")
        .open(&mut show_stock)
        .resizable(false)
        .min_width(380.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Stock Verification").color(Color32::from_rgb(120, 30, 200)));
            ui.separator();
            let total = books::get_all_books(conn).map(|v| v.len()).unwrap_or(0);
            let verified = state.stock_verified.len();
            Grid::new("stock_stats").num_columns(2).spacing([16.0, 6.0]).show(ui, |ui| {
                ui.label(RichText::new("Total Books:").strong());
                ui.label(total.to_string());
                ui.end_row();
                ui.label(RichText::new("Verified:").strong());
                ui.colored_label(Color32::from_rgb(30, 160, 80), verified.to_string());
                ui.end_row();
                ui.label(RichText::new("Unverified:").strong());
                let unverified = total.saturating_sub(verified);
                ui.colored_label(Color32::RED, unverified.to_string());
                ui.end_row();
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Scan Accession No.:");
                let response = ui.text_edit_singleline(&mut state.stock_scan_input);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let acc = state.stock_scan_input.trim().to_string();
                    if !acc.is_empty() && !state.stock_verified.contains(&acc) {
                        state.stock_verified.push(acc);
                        state.stock_scan_input.clear();
                    }
                }
            });
            if !state.stock_msg.is_empty() {
                ui.label(&state.stock_msg);
            }
            ui.add_space(6.0);
            if ui.button("Reset Verification").clicked() {
                state.stock_verified.clear();
            }
        });
    state.show_stock = show_stock;

    // ── Import Book Data ───────────────────────────────────────────────────────
    let mut show_import = state.show_import;
    Window::new("Import Book Data")
        .open(&mut show_import)
        .resizable(false)
        .min_width(380.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Import Book Data").color(Color32::from_rgb(200, 80, 30)));
            ui.separator();
            ui.label("Excel file path (.xlsx):");
            ui.text_edit_singleline(&mut state.import_path);
            ui.add_space(6.0);
            if ui.button("  ⬆  Import  ").clicked() {
                match import_books_excel(conn, &state.import_path) {
                    Ok(n)  => state.import_msg = format!("Imported {n} books."),
                    Err(e) => state.import_msg = format!("Error: {e}"),
                }
            }
            if !state.import_msg.is_empty() {
                ui.label(&state.import_msg);
            }
        });
    state.show_import = show_import;
}

fn import_books_excel(conn: &Connection, path: &str) -> Result<usize, String> {
    use calamine::{Reader, open_workbook, Xlsx};
    let mut workbook: Xlsx<_> = open_workbook(path).map_err(|e: calamine::XlsxError| e.to_string())?;
    let sheet_name = workbook.sheet_names().first().cloned().ok_or("No sheets")?;
    let range = workbook.worksheet_range(&sheet_name).map_err(|e| e.to_string())?;
    let mut count = 0usize;
    for (i, row) in range.rows().enumerate() {
        if i == 0 { continue; } // skip header
        let get = |j: usize| row.get(j).map(|c| c.to_string()).unwrap_or_default();
        let book = Book {
            accession_no: get(0),
            call_no:       get(1),
            title:         get(2),
            author:        get(3),
            branch:        get(4),
            publisher:     get(5),
            price:         get(6).parse::<f64>().unwrap_or(0.0),
            bill_no:       get(7),
            status:        "Available".into(),
        };
        if !book.accession_no.is_empty() {
            let _ = books::add_book(conn, &book);
            count += 1;
        }
    }
    Ok(count)
}
