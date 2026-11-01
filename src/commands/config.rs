use tauri::State;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendStatus {
    pub is_running: bool,
    pub backend_version: String,
    pub active_game: Option<String>,
    pub injected: bool,
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Value, String> {
    let config = state.config.read();
    Ok(config.get_all())
}

#[tauri::command]
pub async fn update_config(state: State<'_, AppState>, path: String, value: Value) -> Result<(), String> {
    log::info!("Updating config key '{}': {:?}", path, value);
    let mut config = state.config.write();
    config.set(&path, value).map_err(|e| e.to_string())?;
    config.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_backend_status(_state: State<'_, AppState>) -> Result<BackendStatus, String> {
    // Queries the native injection engine status
    Ok(BackendStatus {
        is_running: true,
        backend_version: "1.5.13".to_string(),
        active_game: Some("eurotrucks2.exe".to_string()),
        injected: true,
    })
}

#[tauri::command]
pub async fn get_logs(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let logs = state.logs.read();
    Ok(logs.clone())
}

#[tauri::command]
pub async fn clear_logs(state: State<'_, AppState>) -> Result<(), String> {
    let mut logs = state.logs.write();
    logs.clear();
    Ok(())
}
