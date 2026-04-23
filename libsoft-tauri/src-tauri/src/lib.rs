use tauri::Manager;

mod db;
mod commands;

pub use db::Database;

/// Tauri entry point – called from main.rs
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialise SQLite database and store it in Tauri's managed state.
            let db_path = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir")
                .join("libsoft.db");

            // Ensure the directory exists before SQLite tries to create the file
            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent)
                    .expect("Failed to create app data directory");
            }

            let database = db::Database::new(db_path.to_str().unwrap())
                .expect("Failed to open/init database");

            app.manage(std::sync::Mutex::new(database));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::search_books,
            commands::issue_book,
            commands::return_book,
            commands::renew_book,
            commands::get_active_transactions,
            commands::add_book,
            commands::get_members,
            commands::add_member,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
