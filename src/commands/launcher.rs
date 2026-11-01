use tauri::State;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleStatus {
    pub is_running: bool,
    pub is_injected: bool,
    pub game_pid: Option<u32>,
    pub client_type: String, // "ets2mp" | "ets2" | "amtrucks"
}

#[tauri::command]
pub async fn get_lifecycle_status(_state: State<'_, AppState>) -> Result<LifecycleStatus, String> {
    // Check if eurotrucks2.exe or TruckersMP launcher is currently active
    Ok(LifecycleStatus {
        is_running: false,
        is_injected: false,
        game_pid: None,
        client_type: "ets2mp".to_string(),
    })
}

#[tauri::command]
pub async fn launch_truckersmp(_state: State<'_, AppState>) -> Result<bool, String> {
    log::info!("Launching TruckersMP client launcher");
    crate::platform::launcher::launch_truckersmp_process().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn open_smart_app_control_guide() -> Result<(), String> {
    log::info!("Opening Windows Smart App Control guide in browser");
    let _ = open::that("https://support.microsoft.com/windows/what-is-smart-app-control-285ea03d-fa express-49bb-a292-62283daec07f");
    Ok(())
}
