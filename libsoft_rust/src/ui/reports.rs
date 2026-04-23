use egui::{Context, Window, Grid, RichText, Color32, ScrollArea};
use rusqlite::Connection;
use crate::models::transaction::Transaction;
use crate::db::transactions;

#[derive(Default)]
pub struct ReportsState {
    pub show_generate: bool,

    pub date_from: String,
    pub date_to: String,
    pub report_rows: Vec<Transaction>,
    pub report_msg: String,
}

pub fn show(ctx: &Context, state: &mut ReportsState, conn: &Connection) {
    let mut show_generate = state.show_generate;
    Window::new("Generate Report")
        .open(&mut show_generate)
        .resizable(true)
        .min_width(720.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Generate Report").color(Color32::from_rgb(30, 120, 220)));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("From (YYYY-MM-DD):");
                ui.text_edit_singleline(&mut state.date_from);
                ui.label("To:");
                ui.text_edit_singleline(&mut state.date_to);
                if ui.button("  🔍  Load  ").clicked() {
                    match transactions::get_transactions_in_range(conn, &state.date_from, &state.date_to) {
                        Ok(rows) => { state.report_rows = rows; state.report_msg.clear(); }
                        Err(e)  => state.report_msg = format!("Error: {e}"),
                    }
                }
            });
            if !state.report_msg.is_empty() {
                ui.colored_label(Color32::RED, &state.report_msg);
            }
            ui.separator();
            ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
                Grid::new("report_grid")
                    .num_columns(7)
                    .spacing([10.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        for h in ["#","Acc. No","User ID","Name","Issue Date","Exp. Return","Status"] {
                            ui.label(RichText::new(h).strong());
                        }
                        ui.end_row();
                        for (i, t) in state.report_rows.iter().enumerate() {
                            ui.label((i + 1).to_string());
                            ui.label(&t.accession_no);
                            ui.label(&t.user_id);
                            ui.label(&t.user_name);
                            ui.label(&t.issue_date);
                            ui.label(&t.expected_return_date);
                            let color = match t.status.as_str() {
                                "Issued"  => Color32::from_rgb(200, 100, 30),
                                "Returned"=> Color32::from_rgb(30, 160, 80),
                                _         => Color32::from_rgb(100, 100, 200),
                            };
                            ui.colored_label(color, &t.status);
                            ui.end_row();
                        }
                    });
            });
            ui.separator();
            if ui.button("  ⬇  Export to Excel  ").clicked() {
                match export_report_excel(&state.report_rows) {
                    Ok(path) => state.report_msg = format!("Saved to {path}"),
                    Err(e)  => state.report_msg = format!("Error: {e}"),
                }
            }
        });
    state.show_generate = show_generate;
}

fn export_report_excel(rows: &[Transaction]) -> Result<String, String> {
    use rust_xlsxwriter::Workbook;
    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();
    let headers = ["ID","Acc. No","User ID","Name","Issue Date","Exp. Return","Actual Return","Status"];
    for (c, h) in headers.iter().enumerate() {
        ws.write(0, c as u16, *h).ok();
    }
    for (r, t) in rows.iter().enumerate() {
        let row = r as u32 + 1;
        ws.write(row, 0, t.id).ok();
        ws.write(row, 1, t.accession_no.as_str()).ok();
        ws.write(row, 2, t.user_id.as_str()).ok();
        ws.write(row, 3, t.user_name.as_str()).ok();
        ws.write(row, 4, t.issue_date.as_str()).ok();
        ws.write(row, 5, t.expected_return_date.as_str()).ok();
        ws.write(row, 6, t.actual_return_date.as_deref().unwrap_or("")).ok();
        ws.write(row, 7, t.status.as_str()).ok();
    }
    let path = "report_export.xlsx";
    wb.save(path).map_err(|e| e.to_string())?;
    Ok(path.to_string())
}
