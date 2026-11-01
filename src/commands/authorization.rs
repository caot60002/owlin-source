use tauri::State;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub is_authorized: bool,
    pub user: Option<UserInfo>,
    pub access: FeatureAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureAccess {
    pub ets2: bool,
    pub ets2mp: bool,
    pub spoofer: bool,
    pub dlc: bool,
}

#[tauri::command]
pub async fn restore_session(state: State<'_, AppState>) -> Result<SessionState, String> {
    log::info!("Attempting to restore session from persistent storage");
    let auth = state.auth.read();
    auth.check_cached_session().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sign_in_with_discord(state: State<'_, AppState>) -> Result<String, String> {
    log::info!("Starting Discord OAuth login flow");
    let mut auth = state.auth.write();
    auth.initiate_discord_oauth().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cancel_discord_authorization(state: State<'_, AppState>) -> Result<(), String> {
    log::info!("Cancelling Discord OAuth flow");
    let mut auth = state.auth.write();
    auth.cancel_oauth();
    Ok(())
}

#[tauri::command]
pub async fn sign_out(state: State<'_, AppState>) -> Result<(), String> {
    log::info!("User signing out");
    let mut auth = state.auth.write();
    auth.clear_session();
    Ok(())
}
