pub mod app;
pub mod commands;
pub mod platform;
pub mod services;

use std::sync::Arc;
use parking_lot::RwLock;
use tauri::{Manager, State};

use commands::authorization::*;
use commands::config::*;
use commands::ets2::*;
use commands::launcher::*;

#[derive(Default)]
pub struct AppState {
    pub config: Arc<RwLock<services::config::service::ConfigService>>,
    pub auth: Arc<RwLock<services::authorization::callback::AuthService>>,
    pub logs: Arc<RwLock<Vec<String>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_lifecycle_status,
            get_logs,
            clear_logs,
            get_config,
            update_config,
            get_backend_status,
            restore_session,
            sign_in_with_discord,
            cancel_discord_authorization,
            sign_out,
            run_action,
            get_admin_detection,
            launch_truckersmp,
            open_smart_app_control_guide,
        ])
        .setup(|app| {
            let handle = app.handle();
            log::info!("Owlin Mod Menu v1.5.13 initialized");
            
            // Spawn background task to monitor ETS2/TruckersMP and update lifecycle
            let state = handle.state::<AppState>();
            tauri::async_runtime::spawn(async move {
                // Polling loop for process lifecycle and admin detection
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Owlin application");
}
