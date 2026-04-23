use egui::{Context, Window, Grid, RichText, Color32};
use rusqlite::Connection;
use crate::models::member::Member;
use crate::db::members;

// ─── Modal visibility flags ───────────────────────────────────────────────────
#[derive(Default)]
pub struct MembershipState {
    pub show_add: bool,
    pub show_edit: bool,
    pub show_details: bool,
    pub show_export: bool,

    // Add member form
    pub add_form: Member,

    // Edit member form
    pub edit_search_id: String,
    pub edit_form: Member,
    pub edit_status_msg: String,

    // Details
    pub details_search_id: String,
    pub details_member: Option<Member>,

    // Export filters
    pub export_course: String,
    pub export_branch: String,
    pub export_year: String,
    pub export_active_only: bool,
    pub export_status_msg: String,
}

fn member_form_fields(ui: &mut egui::Ui, m: &mut Member, prefix: &str) {
    Grid::new(format!("{prefix}_grid"))
        .num_columns(2)
        .spacing([12.0, 8.0])
        .show(ui, |ui| {
            ui.label("First Name:");
            ui.text_edit_singleline(&mut m.first_name);
            ui.end_row();

            ui.label("Middle Name:");
            ui.text_edit_singleline(&mut m.middle_name);
            ui.end_row();

            ui.label("Last Name:");
            ui.text_edit_singleline(&mut m.last_name);
            ui.end_row();

            ui.label("Admission Year:");
            ui.text_edit_singleline(&mut m.admission_year);
            ui.end_row();

            ui.label("Course:");
            ui.text_edit_singleline(&mut m.course);
            ui.end_row();

            ui.label("Current Year:");
            ui.text_edit_singleline(&mut m.current_year);
            ui.end_row();

            ui.label("Branch:");
            ui.text_edit_singleline(&mut m.branch);
            ui.end_row();

            ui.label("Mobile No.:");
            let prev_mob = m.mobile_no.clone();
            ui.text_edit_singleline(&mut m.mobile_no);
            if !m.mobile_no.chars().all(char::is_numeric) {
                m.mobile_no = prev_mob;
            }
            ui.end_row();

            ui.label("Email ID:");
            ui.text_edit_singleline(&mut m.email);
            ui.end_row();

            ui.label("Address:");
            ui.text_edit_multiline(&mut m.address);
            ui.end_row();
        });
}

pub fn show(ctx: &Context, state: &mut MembershipState, conn: &Connection) {
    // ── Add Member ─────────────────────────────────────────────────────────────
    let mut show_add = state.show_add;
    Window::new("Add Member")
        .open(&mut show_add)
        .resizable(true)
        .min_width(440.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Add New Member").color(Color32::from_rgb(30, 120, 220)));
            ui.separator();
            member_form_fields(ui, &mut state.add_form, "add");
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("  Generate ID  ").clicked() {
                    let ts = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis();
                    state.add_form.id = format!("MEM{}", ts % 100_000);
                }
                ui.label(format!("ID: {}", state.add_form.id));
            });
            ui.add_space(8.0);
            if ui.button("  ✔  Save Member  ").clicked() && !state.add_form.id.is_empty() {
                state.add_form.is_active = true;
                let _ = members::add_member(conn, &state.add_form);
                state.add_form = Member::default();
                state.show_add = false;
            }
        });
    state.show_add = show_add;

    // ── Edit Member ────────────────────────────────────────────────────────────
    let mut show_edit = state.show_edit;
    Window::new("Edit Member")
        .open(&mut show_edit)
        .resizable(true)
        .min_width(440.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Edit Member").color(Color32::from_rgb(200, 120, 30)));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Member ID:");
                ui.text_edit_singleline(&mut state.edit_search_id);
                if ui.button("Load").clicked() {
                    if let Ok(Some(m)) = members::get_member(conn, &state.edit_search_id) {
                        state.edit_form = m;
                        state.edit_status_msg = String::new();
                    } else {
                        state.edit_status_msg = "Member not found.".to_string();
                    }
                }
            });
            if !state.edit_status_msg.is_empty() {
                ui.colored_label(Color32::RED, &state.edit_status_msg);
            }
            ui.separator();
            member_form_fields(ui, &mut state.edit_form, "edit");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Active:");
                ui.checkbox(&mut state.edit_form.is_active, "");
                ui.label(format!("Total Due: ₹ {:.2}", state.edit_form.total_due));
            });
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui.button("  ✔  Save Changes  ").clicked() && !state.edit_form.id.is_empty() {
                    let _ = members::add_member(conn, &state.edit_form);
                    state.edit_status_msg = "Saved.".to_string();
                }
                if ui.button("  🗑  Delete Member  ").clicked() && !state.edit_form.id.is_empty() {
                    let _ = members::delete_member(conn, &state.edit_form.id);
                    state.edit_form = Member::default();
                    state.edit_status_msg = "Deleted.".to_string();
                }
                if ui.button("  Clear Old Data  ").clicked() {
                    state.edit_form = Member::default();
                }
            });
        });
    state.show_edit = show_edit;

    // ── Member Details ─────────────────────────────────────────────────────────
    let mut show_details = state.show_details;
    Window::new("Member Details")
        .open(&mut show_details)
        .resizable(false)
        .min_width(360.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Member Details").color(Color32::from_rgb(30, 160, 80)));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Member ID:");
                ui.text_edit_singleline(&mut state.details_search_id);
                if ui.button("Search").clicked() {
                    state.details_member = members::get_member(conn, &state.details_search_id).ok().flatten();
                }
            });
            ui.separator();
            if let Some(m) = &state.details_member {
                Grid::new("details_grid").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                    macro_rules! row { ($label:expr, $val:expr) => {
                        ui.label($label); ui.label($val); ui.end_row();
                    }}
                    row!("ID:", &m.id);
                    row!("Name:", &format!("{} {} {}", m.first_name, m.middle_name, m.last_name));
                    row!("Course:", &m.course);
                    row!("Branch:", &m.branch);
                    row!("Year:", &m.current_year);
                    row!("Mobile:", &m.mobile_no);
                    row!("Email:", &m.email);
                    row!("Active:", if m.is_active { "Yes" } else { "No" });
                    row!("Total Due:", &format!("₹ {:.2}", m.total_due));
                });
            }
        });
    state.show_details = show_details;

    // ── Export Member Data ─────────────────────────────────────────────────────
    let mut show_export = state.show_export;
    Window::new("Export Member Data")
        .open(&mut show_export)
        .resizable(false)
        .min_width(340.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Export Member Data").color(Color32::from_rgb(120, 30, 200)));
            ui.separator();
            Grid::new("export_grid").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
                ui.label("Course:"); ui.text_edit_singleline(&mut state.export_course); ui.end_row();
                ui.label("Branch:"); ui.text_edit_singleline(&mut state.export_branch); ui.end_row();
                ui.label("Year:"); ui.text_edit_singleline(&mut state.export_year); ui.end_row();
                ui.label("Active Only:"); ui.checkbox(&mut state.export_active_only, ""); ui.end_row();
            });
            ui.add_space(8.0);
            if ui.button("  ⬇  Export to Excel  ").clicked() {
                match export_members_excel(conn, &state.export_course, &state.export_branch,
                                           &state.export_year, state.export_active_only) {
                    Ok(path) => state.export_status_msg = format!("Exported to {path}"),
                    Err(e)  => state.export_status_msg = format!("Error: {e}"),
                }
            }
            if !state.export_status_msg.is_empty() {
                ui.label(&state.export_status_msg);
            }
        });
    state.show_export = show_export;
}

fn export_members_excel(
    conn: &Connection,
    course: &str, branch: &str, year: &str, active_only: bool,
) -> Result<String, String> {
    use rust_xlsxwriter::Workbook;

    let all = members::get_all_members(conn).map_err(|e| e.to_string())?;
    let filtered: Vec<&Member> = all.iter().filter(|m| {
        (course.is_empty() || m.course.contains(course))
            && (branch.is_empty() || m.branch.contains(branch))
            && (year.is_empty() || m.current_year.contains(year))
            && (!active_only || m.is_active)
    }).collect();

    let mut wb = Workbook::new();
    let ws = wb.add_worksheet();

    let headers = ["ID","First Name","Middle Name","Last Name","Admission Year",
                   "Course","Year","Branch","Mobile","Email","Address","Active","Due"];
    for (c, h) in headers.iter().enumerate() {
        ws.write(0, c as u16, *h).ok();
    }
    for (r, m) in filtered.iter().enumerate() {
        let row = r as u32 + 1;
        ws.write(row, 0, m.id.as_str()).ok();
        ws.write(row, 1, m.first_name.as_str()).ok();
        ws.write(row, 2, m.middle_name.as_str()).ok();
        ws.write(row, 3, m.last_name.as_str()).ok();
        ws.write(row, 4, m.admission_year.as_str()).ok();
        ws.write(row, 5, m.course.as_str()).ok();
        ws.write(row, 6, m.current_year.as_str()).ok();
        ws.write(row, 7, m.branch.as_str()).ok();
        ws.write(row, 8, m.mobile_no.as_str()).ok();
        ws.write(row, 9, m.email.as_str()).ok();
        ws.write(row, 10, m.address.as_str()).ok();
        ws.write(row, 11, if m.is_active { "Yes" } else { "No" }).ok();
        ws.write(row, 12, m.total_due).ok();
    }
    let path = "members_export.xlsx";
    wb.save(path).map_err(|e| e.to_string())?;
    Ok(path.to_string())
}
