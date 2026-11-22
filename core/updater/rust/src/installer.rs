use crate::downloader::UpdateError;
use std::{path::Path, process::Command};
use wirewave_core_config::APP_CONFIG;

/// Launches the verified local NSIS installer (used in `BuildChannel::Dev`).
pub fn launch_installer(installer_path: &Path) -> Result<(), UpdateError> {
    if !installer_path.exists() {
        return Err(UpdateError::IncompleteFile);
    }
    Command::new(installer_path)
        .spawn()
        .map_err(|e| UpdateError::Launch(e.to_string()))?;
    Ok(())
}

/// Opens the Microsoft Store listing (used in `BuildChannel::Store`).
pub fn open_store_listing() -> Result<(), UpdateError> {
    open::that(APP_CONFIG.store_url).map_err(|e| UpdateError::Launch(e.to_string()))
}
