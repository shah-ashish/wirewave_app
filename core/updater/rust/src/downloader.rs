use crate::manifest::UpdateRelease;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};
use thiserror::Error;
use wirewave_core_config::APP_CONFIG;

const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(300);
const MIN_INSTALLER_BYTES: u64 = 100_000;
const CHUNK_SIZE: usize = 32 * 1024;

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("network request failed: {0}")]
    Network(String),
    #[error("invalid update manifest: {0}")]
    InvalidManifest(String),
    #[error("update app id mismatch (expected {expected}, got {actual})")]
    UpdateAppIdMismatch { expected: String, actual: String },
    #[error("checksum verification failed")]
    ChecksumMismatch,
    #[error("downloaded installer is incomplete")]
    IncompleteFile,
    #[error("file system error: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to launch installer: {0}")]
    Launch(String),
}

impl UpdateError {
    pub fn user_message(&self) -> &'static str {
        match self {
            Self::Network(_) => "Couldn’t connect. Check your internet and try again.",
            Self::InvalidManifest(_) => "Update information is unavailable. Try again later.",
            Self::UpdateAppIdMismatch { .. } => "This update is not compatible with this app.",
            Self::ChecksumMismatch => "Update verification failed. Download it again.",
            Self::IncompleteFile => "The update download was incomplete. Try again.",
            Self::Io(_) => "Couldn’t save the update. Try again.",
            Self::Launch(_) => "Couldn’t open the installer. Try again.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DownloadProgress {
    pub bytes_downloaded: u64,
    pub total_bytes: Option<u64>,
}

impl DownloadProgress {
    pub fn percent(&self) -> u8 {
        match self.total_bytes {
            Some(total) if total > 0 => ((self.bytes_downloaded * 100) / total).min(100) as u8,
            _ => 0,
        }
    }
}

/// Downloads an update installer to a `.part` file, verifies its size and optional
/// SHA-256 digest, and atomically promotes it to the final `.exe` path.
pub fn download_update<F>(release: &UpdateRelease, mut on_progress: F) -> Result<PathBuf, UpdateError>
where
    F: FnMut(DownloadProgress),
{
    let final_path = installer_path(&release.version_name);
    let part_path = final_path.with_extension("exe.part");
    let _ = fs::remove_file(&part_path);

    let result = stream_to_file(release, &part_path, &mut on_progress).and_then(|()| {
        let expected = release.sha256.as_deref().ok_or_else(|| {
            UpdateError::InvalidManifest("desktop artifact is missing SHA-256".to_string())
        })?;
        verify_sha256(&part_path, expected)?;
        let _ = fs::remove_file(&final_path);
        fs::rename(&part_path, &final_path)?;
        Ok(final_path)
    });

    if result.is_err() {
        let _ = fs::remove_file(&part_path);
    }
    result
}

fn stream_to_file<F>(
    release: &UpdateRelease,
    part_path: &Path,
    on_progress: &mut F,
) -> Result<(), UpdateError>
where
    F: FnMut(DownloadProgress),
{
    let client = reqwest::blocking::Client::builder()
        .timeout(DOWNLOAD_TIMEOUT)
        .build()
        .map_err(|e| UpdateError::Network(e.to_string()))?;
    let mut response = client
        .get(&release.download_url)
        .header("X-Update-App-Id", APP_CONFIG.update_app_id)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|e| UpdateError::Network(e.to_string()))?;

    let total_bytes = response.content_length();

    let mut file = File::create(part_path)?;
    let mut buffer = [0u8; CHUNK_SIZE];
    let mut downloaded: u64 = 0;

    loop {
        let read_bytes = response.read(&mut buffer)?;
        if read_bytes == 0 {
            break;
        }
        file.write_all(&buffer[..read_bytes])?;
        downloaded += read_bytes as u64;
        on_progress(DownloadProgress {
            bytes_downloaded: downloaded,
            total_bytes,
        });
    }
    file.flush()?;

    match (total_bytes, downloaded >= MIN_INSTALLER_BYTES) {
        (Some(expected), _) if downloaded != expected => Err(UpdateError::IncompleteFile),
        (_, false) => Err(UpdateError::IncompleteFile),
        _ => Ok(()),
    }
}

pub fn verify_sha256(path: &Path, expected_hex: &str) -> Result<(), UpdateError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; CHUNK_SIZE];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let actual = hex::encode(hasher.finalize());
    match actual.eq_ignore_ascii_case(expected_hex.trim()) {
        true => Ok(()),
        false => Err(UpdateError::ChecksumMismatch),
    }
}

fn installer_dir() -> PathBuf {
    std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .map(|p| p.join(APP_CONFIG.app_name).join("updates"))
        .unwrap_or_else(|| std::env::temp_dir().join(APP_CONFIG.app_name).join("updates"))
}

fn installer_path(version_name: &str) -> PathBuf {
    let safe_ver: String = version_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '.' || *c == '-')
        .collect();
    let dir = installer_dir();
    let _ = fs::create_dir_all(&dir);
    dir.join(format!("{}-Setup-{safe_ver}.exe", APP_CONFIG.app_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_sha256_accepts_matching_digest_and_rejects_mismatch() {
        let path = std::env::temp_dir().join("ww_sha_test.bin");
        fs::write(&path, b"wirewave").expect("write");
        let expected = hex::encode(Sha256::digest(b"wirewave"));

        assert!(verify_sha256(&path, &expected).is_ok());
        assert!(matches!(
            verify_sha256(&path, "0000"),
            Err(UpdateError::ChecksumMismatch)
        ));
        let _ = fs::remove_file(path);
    }
}
