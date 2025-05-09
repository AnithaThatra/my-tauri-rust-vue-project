
mod utils;
mod tauri_handlers;
use std::sync::Arc;
use crate::utils::server::run_server;

use tauri_handlers::{register_tauri, login_tauri, fetch_all_users_tauri, create_user_tauri, update_user_tauri, delete_user_tauri, AppState};



#[tokio::main]
async fn main() {
    let pool = run_server().await;

    tauri::Builder::default()
        .manage(AppState { db: Arc::new(pool) })
        .invoke_handler(tauri::generate_handler![
            register_tauri,
            login_tauri,
            create_user_tauri,
            fetch_all_users_tauri,
            update_user_tauri,
            delete_user_tauri
        ])
        .run(tauri::generate_context!())
        .expect("error running Tauri app");
}
