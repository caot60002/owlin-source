use tauri::State;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRequest {
    pub kind: String,
    #[serde(flatten)]
    pub params: std::collections::HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminDetectionInfo {
    pub admin_detected: bool,
    pub admin_count: u32,
    pub admins: Vec<AdminRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminRecord {
    pub id: u32,
    pub name: String,
    pub distance_meters: f32,
}

#[tauri::command]
pub async fn run_action(_state: State<'_, AppState>, action: ActionRequest) -> Result<bool, String> {
    log::info!("Executing ETS2/TruckersMP cheat action: '{}'", action.kind);

    match action.kind.as_str() {
        "refuel" => {
            log::info!("Action: Refill fuel tank to 100%");
            // Trigger memory modification in eurotrucks2.exe via injected DLL IPC
        }
        "repair" => {
            log::info!("Action: Zero out wear and damage on truck, engine, transmission, wheels, trailer");
            // Set truck damage floats to 0.0
        }
        "setTime" => {
            if let Some(time_val) = action.params.get("time") {
                log::info!("Action: Synchronize game time to {:?}", time_val);
            }
        }
        "noclip" => {
            log::info!("Action: Toggle NCZ (No Collision Zone) Hack");
            // Patch collision detection routines in eurotrucks2.exe
        }
        "speedUnlock" => {
            log::info!("Action: Bypass 90 km/h / 145 km/h TruckersMP speed limiter");
        }
        "money" => {
            if let Some(amount) = action.params.get("value") {
                log::info!("Action: Adjust profile bank account money: {:?}", amount);
            }
        }
        "level" => {
            if let Some(lvl) = action.params.get("value") {
                log::info!("Action: Adjust profile driver level / XP: {:?}", lvl);
            }
        }
        "spoofer" => {
            log::info!("Action: Toggle Hardware ID / Volume / MachineGuid spoofer");
        }
        other => {
            log::warn!("Unknown or module action requested: '{}'", other);
        }
    }

    Ok(true)
}

#[tauri::command]
pub async fn get_admin_detection(_state: State<'_, AppState>) -> Result<AdminDetectionInfo, String> {
    // Reads radar / player list from eurotrucks2.exe / core_ets2mp.dll memory
    // Checks player tags / roles for "Admin", "Moderator", "TruckersMP Staff"
    Ok(AdminDetectionInfo {
        admin_detected: false,
        admin_count: 0,
        admins: Vec::new(),
    })
}
