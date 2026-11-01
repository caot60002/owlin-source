use std::process::Command;

pub fn launch_truckersmp_process() -> Result<bool, String> {
    // Locates TruckersMP launcher executable from Registry or standard AppData/Local path:
    // %LOCALAPPDATA%\TruckersMP\Launcher.exe
    let local_appdata = std::env::var("LOCALAPPDATA").map_err(|e| e.to_string())?;
    let launcher_path = std::path::Path::new(&local_appdata)
        .join("TruckersMP")
        .join("Launcher.exe");

    if launcher_path.exists() {
        Command::new(launcher_path)
            .spawn()
            .map_err(|e| format!("Failed to spawn TruckersMP launcher: {}", e))?;
        Ok(true)
    } else {
        Err("TruckersMP Launcher not found in standard directory".to_string())
    }
}
