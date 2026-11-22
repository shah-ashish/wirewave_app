use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::{Path, PathBuf}};
use thiserror::Error;
use uuid::Uuid;
use wirewave_core_config::APP_CONFIG;

const IDENTITY_SCHEMA_VERSION: u32 = 1;
const IDENTITY_FILE_NAME: &str = "identity.json";
const DEVICE_ID_SALT: &str = "wirewave:device:win:v1:";

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("unable to resolve local application data directory")]
    NoDataDir,
    #[error("identity I/O failure: {0}")]
    Io(#[from] std::io::Error),
    #[error("identity serialization failure: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceIdentity {
    pub schema_version: u32,
    pub device_id: String,
}

/// Fetches the persisted device ID, or generates and saves one if absent.
///
/// The file is stored in `%LOCALAPPDATA%\WireWave\identity.json`, which the
/// NSIS installer preserves across uninstall/reinstall. On Windows, initial
/// generation hashes the machine GUID so even a manual data wipe yields the
/// same stable ID on the same machine.
pub fn fetch_or_create_device_id() -> Result<DeviceIdentity, IdentityError> {
    let path = default_identity_path()?;
    fetch_or_create_at(&path)
}

pub fn fetch_or_create_at(path: &Path) -> Result<DeviceIdentity, IdentityError> {
    if let Some(existing) = load_identity(path) {
        return Ok(existing);
    }

    if path.exists() {
        let backup_path = path.with_extension(format!("json.corrupt-{}", Uuid::new_v4()));
        fs::rename(path, backup_path)?;
    }

    let created = DeviceIdentity {
        schema_version: IDENTITY_SCHEMA_VERSION,
        device_id: generate_device_id(),
    };
    save_identity(path, &created)?;
    Ok(created)
}

pub fn load_identity(path: &Path) -> Option<DeviceIdentity> {
    let raw = fs::read_to_string(path).ok()?;
    let parsed: DeviceIdentity = serde_json::from_str(&raw).ok()?;
    (!parsed.device_id.trim().is_empty()).then_some(parsed)
}

pub fn generate_device_id() -> String {
    let seed = read_platform_machine_seed().unwrap_or_else(|| Uuid::new_v4().to_string());
    derive_salted_id(&seed)
}

fn derive_salted_id(seed: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(DEVICE_ID_SALT.as_bytes());
    hasher.update(seed.trim().as_bytes());
    let digest = hex::encode(hasher.finalize());
    format!("wwd_{}", &digest[..24])
}

fn save_identity(path: &Path, identity: &DeviceIdentity) -> Result<(), IdentityError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp_path = path.with_extension("json.tmp");
    let json = serde_json::to_string_pretty(identity)?;
    fs::write(&tmp_path, json)?;
    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn default_identity_path() -> Result<PathBuf, IdentityError> {
    let base = BaseDirs::new().ok_or(IdentityError::NoDataDir)?;
    Ok(base
        .data_local_dir()
        .join(APP_CONFIG.app_name)
        .join(IDENTITY_FILE_NAME))
}

#[cfg(windows)]
fn read_platform_machine_seed() -> Option<String> {
    use windows::core::w;
    use windows::Win32::System::Registry::{
        RegGetValueW, HKEY_LOCAL_MACHINE, RRF_RT_REG_SZ,
    };

    let mut buffer = [0u16; 128];
    let mut byte_len = (buffer.len() * std::mem::size_of::<u16>()) as u32;
    let status = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            w!("SOFTWARE\\Microsoft\\Cryptography"),
            w!("MachineGuid"),
            RRF_RT_REG_SZ,
            None,
            Some(buffer.as_mut_ptr().cast()),
            Some(&mut byte_len),
        )
    };
    if status.is_err() {
        return None;
    }
    let len = (byte_len as usize / 2).saturating_sub(1);
    let value = String::from_utf16_lossy(&buffer[..len]);
    (!value.trim().is_empty()).then_some(value)
}

#[cfg(not(windows))]
fn read_platform_machine_seed() -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_or_create_is_idempotent() {
        let dir = std::env::temp_dir().join(format!("ww_id_test_{}", Uuid::new_v4()));
        let file = dir.join(IDENTITY_FILE_NAME);

        let first = fetch_or_create_at(&file).expect("first create");
        let second = fetch_or_create_at(&file).expect("second fetch");

        assert_eq!(first, second);
        assert!(first.device_id.starts_with("wwd_"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn salted_hash_is_deterministic_and_non_raw() {
        let id_a = derive_salted_id("test-machine-guid");
        let id_b = derive_salted_id("test-machine-guid");
        assert_eq!(id_a, id_b);
        assert!(!id_a.contains("test-machine-guid"));
    }

    #[test]
    fn corrupted_identity_is_backed_up_and_replaced() {
        let dir = std::env::temp_dir().join(format!("ww_id_repair_test_{}", Uuid::new_v4()));
        fs::create_dir_all(&dir).expect("create test directory");
        let file = dir.join(IDENTITY_FILE_NAME);
        fs::write(&file, "not valid json").expect("write corrupt identity");

        let repaired = fetch_or_create_at(&file).expect("repair identity");
        let loaded = load_identity(&file).expect("load repaired identity");

        assert_eq!(repaired, loaded);
        assert!(dir.read_dir().expect("read test directory").any(|entry| {
            entry
                .expect("directory entry")
                .file_name()
                .to_string_lossy()
                .starts_with("identity.json.corrupt-")
        }));

        let _ = fs::remove_dir_all(dir);
    }
}
