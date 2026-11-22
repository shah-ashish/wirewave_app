use crate::downloader::UpdateError;
use std::{path::Path, process::Command};
use wirewave_core_config::APP_CONFIG;

/// Launches the verified local NSIS installer (used in `BuildChannel::Dev`).
///
/// After successfully spawning the installer the current process exits immediately
/// so Windows releases its file lock on `wirewave-desktop.exe` before the NSIS
/// `File` instruction tries to overwrite it.
pub fn launch_installer(installer_path: &Path) -> Result<(), UpdateError> {
    if !installer_path.exists() {
        return Err(UpdateError::IncompleteFile);
    }
    Command::new(installer_path)
        .spawn()
        .map_err(|e| UpdateError::Launch(e.to_string()))?;
    // Exit immediately so the OS releases the file lock on this binary,
    // allowing the NSIS installer to replace wirewave-desktop.exe.
    std::process::exit(0);
}

/// Opens the Microsoft Store listing (used in `BuildChannel::Store`).
pub fn open_store_listing() -> Result<(), UpdateError> {
    open::that(APP_CONFIG.store_url).map_err(|e| UpdateError::Launch(e.to_string()))
}
