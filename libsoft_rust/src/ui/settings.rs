use egui::{Context, Window, RichText, Color32};
use rusqlite::Connection;
use crate::db::members;

#[derive(Default)]
pub struct SettingsState {
    pub show_activation: bool,
    pub show_branch: bool,

    pub activation_msg: String,
    pub new_branch: String,
    pub branches: Vec<String>,
    pub branch_msg: String,
}

pub fn show(ctx: &Context, state: &mut SettingsState, conn: &Connection) {
    // ── Activation Settings ────────────────────────────────────────────────────
    let mut show_activation = state.show_activation;
    Window::new("Activation Settings")
        .open(&mut show_activation)
        .resizable(false)
        .min_width(320.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Activation Settings").color(Color32::from_rgb(30, 120, 220)));
            ui.separator();
            ui.label("Apply activation status to all members:");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("  ✔  Activate All  ").clicked() {
                    match members::set_all_active(conn, true) {
                        Ok(_) => state.activation_msg = "All members activated.".into(),
                        Err(e) => state.activation_msg = format!("Error: {e}"),
                    }
                }
                if ui.button("  ✕  Deactivate All  ").clicked() {
                    match members::set_all_active(conn, false) {
                        Ok(_) => state.activation_msg = "All members deactivated.".into(),
                        Err(e) => state.activation_msg = format!("Error: {e}"),
                    }
                }
            });
            if !state.activation_msg.is_empty() {
                ui.label(&state.activation_msg);
            }
        });
    state.show_activation = show_activation;

    // ── Edit Branch ────────────────────────────────────────────────────────────
    let mut show_branch = state.show_branch;
    Window::new("Edit Branch")
        .open(&mut show_branch)
        .resizable(false)
        .min_width(320.0)
        .show(ctx, |ui| {
            ui.heading(RichText::new("Edit Branch").color(Color32::from_rgb(120, 30, 200)));
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Branch Name:");
                ui.text_edit_singleline(&mut state.new_branch);
                if ui.button("+ Add").clicked() {
                    let branch = state.new_branch.trim().to_string();
                    if !branch.is_empty() && !state.branches.contains(&branch) {
                        state.branches.push(branch);
                        state.new_branch.clear();
                        state.branch_msg = "Branch added.".into();
                    }
                }
            });
            if !state.branch_msg.is_empty() {
                ui.label(&state.branch_msg);
            }
            ui.separator();
            let mut to_remove = None;
            for (i, b) in state.branches.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(b);
                    if ui.button("✕").clicked() { to_remove = Some(i); }
                });
            }
            if let Some(i) = to_remove { state.branches.remove(i); }
        });
    state.show_branch = show_branch;
}
