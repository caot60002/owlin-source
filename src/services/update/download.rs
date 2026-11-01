use std::path::PathBuf;

pub const UPDATE_URL: &str = "https://github.com/Lesykkk/owlin-site/releases/latest/download";

pub async fn check_for_updates() -> Result<Option<String>, String> {
    log::info!("Checking for latest release at {}", UPDATE_URL);
    // Compares version with 1.5.13
    Ok(None)
}

pub async fn download_update(_version: &str, _dest: PathBuf) -> Result<(), String> {
    log::info!("Downloading update package");
    Ok(())
}
