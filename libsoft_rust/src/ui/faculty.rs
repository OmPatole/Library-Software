use egui::{Context, Window, Grid, RichText, Color32};
use rusqlite::Connection;
use crate::models::faculty::Faculty;
use crate::db::faculty;

#[derive(Default)]
pub struct FacultyState {
    pub show_add: bool,
    pub show_edit: bool,
    pub show_details: bool,

    pub add_form: Faculty,

    pub edit_search: String,
    pub edit_form: Faculty,
    pub edit_msg: String,

    pub details_search: String,
    pub details_faculty: Option<Faculty>,
}

fn faculty_form(ui: &mut egui::Ui, f: &mut Faculty, prefix: &str) {
    Grid::new(format!("{prefix}_fgrid"))
        .num_columns(2)
        .spacing([12.0, 8.0])
        .show(ui, |ui| {
            ui.label("First Name:");      ui.text_edit_singleline(&mut f.first_name);    ui.end_row();
            ui.label("Middle Name:");     ui.text_edit_singleline(&mut f.middle_name);   ui.end_row();
            ui.label("Last Name:");       ui.text_edit_singleline(&mut f.last_name);     ui.end_row();
            ui.label("Joining Date:");    ui.text_edit_singleline(&mut f.joining_date);  ui.end_row();
            ui.label("Joining Under:");   ui.text_edit_singleline(&mut f.joining_under); ui.end_row();
            ui.label("Branch:");          ui.text_edit_singleline(&mut f.branch);        ui.end_row();
            ui.label("Mobile No.:");
            let prev = f.mobile_no.clone();
            ui.text_edit_singleline(&mut f.mobile_no);
            if !f.mobile_no.chars().all(char::is_numeric) { f.mobile_no = prev; }
            ui.end_row();
            ui.label("Email:");   ui.text_edit_singleline(&mut f.email);   ui.end_row();
            ui.label("Address:"); ui.text_edit_multiline(&mut f.address);  ui.end_row();
        });
}

pub fn show(ctx: &Context, state: &mut FacultyState, conn: &Connection) {
    // ── Add Faculty ────────────────────────────────────────────────────────────
    let mut show_add = state.show_add;
    Window::new("Add Faculty")
        .open(&mut show_add)
        .resizable(true)
        .min_width(440.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Add Faculty").color(Color32::from_rgb(30, 120, 220)));
            ui.separator();
            faculty_form(ui, &mut state.add_form, "fac_add");
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("  Generate UID  ").clicked() {
                    let ts = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis();
                    state.add_form.uid = format!("FAC{}", ts % 100_000);
                }
                ui.label(format!("UID: {}", state.add_form.uid));
            });
            ui.add_space(6.0);
            if ui.button("  ✔  Save Faculty  ").clicked() && !state.add_form.uid.is_empty() {
                state.add_form.is_active = true;
                let _ = faculty::add_faculty(conn, &state.add_form);
                state.add_form = Faculty::default();
                state.show_add = false;
            }
        });
    state.show_add = show_add;

    // ── Edit Faculty ───────────────────────────────────────────────────────────
    let mut show_edit = state.show_edit;
    Window::new("Edit Faculty")
        .open(&mut show_edit)
        .resizable(true)
        .min_width(440.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Edit Faculty").color(Color32::from_rgb(200, 120, 30)));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("UID:");
                ui.text_edit_singleline(&mut state.edit_search);
                if ui.button("Load").clicked() {
                    match faculty::get_faculty(conn, &state.edit_search) {
                        Ok(Some(f)) => { state.edit_form = f; state.edit_msg.clear(); }
                        _ => state.edit_msg = "Faculty not found.".into(),
                    }
                }
            });
            if !state.edit_msg.is_empty() {
                ui.colored_label(Color32::RED, &state.edit_msg);
            }
            ui.separator();
            faculty_form(ui, &mut state.edit_form, "fac_edit");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Active:");
                ui.checkbox(&mut state.edit_form.is_active, "");
                ui.label(format!("Total Due: ₹ {:.2}", state.edit_form.total_due));
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui.button("  ✔  Save  ").clicked() && !state.edit_form.uid.is_empty() {
                    let _ = faculty::add_faculty(conn, &state.edit_form);
                    state.edit_msg = "Saved.".into();
                }
                if ui.button("  🗑  Delete  ").clicked() && !state.edit_form.uid.is_empty() {
                    let _ = faculty::delete_faculty(conn, &state.edit_form.uid);
                    state.edit_form = Faculty::default();
                    state.edit_msg = "Deleted.".into();
                }
            });
        });
    state.show_edit = show_edit;

    // ── Faculty Details ────────────────────────────────────────────────────────
    let mut show_details = state.show_details;
    Window::new("Faculty Details")
        .open(&mut show_details)
        .resizable(false)
        .min_width(380.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Faculty Details").color(Color32::from_rgb(30, 160, 80)));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("UID:");
                ui.text_edit_singleline(&mut state.details_search);
                if ui.button("Search").clicked() {
                    state.details_faculty = faculty::get_faculty(conn, &state.details_search).ok().flatten();
                }
            });
            ui.separator();
            if let Some(f) = &state.details_faculty {
                Grid::new("fac_details_grid").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                    macro_rules! row { ($l:expr, $v:expr) => {
                        ui.label($l); ui.label($v); ui.end_row();
                    }}
                    row!("UID:",          &f.uid);
                    row!("Name:",         &format!("{} {} {}", f.first_name, f.middle_name, f.last_name));
                    row!("Joining Date:", &f.joining_date);
                    row!("Joining Under:", &f.joining_under);
                    row!("Branch:",       &f.branch);
                    row!("Mobile:",       &f.mobile_no);
                    row!("Email:",        &f.email);
                    row!("Active:",       if f.is_active { "Yes" } else { "No" });
                    row!("Total Due:",    &format!("₹ {:.2}", f.total_due));
                });
                ui.add_space(8.0);
                if ui.button("  ✉  Generate Mail  ").clicked() {
                    let _ = open::that(format!(
                        "mailto:{}?subject=Library Notice", f.email
                    ));
                }
            }
        });
    state.show_details = show_details;
}
