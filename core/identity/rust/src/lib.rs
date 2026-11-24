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

/// Encapsulates persistent device identity management and hashing.
pub struct DeviceIdentityStore {
    path: PathBuf,
}

impl DeviceIdentityStore {
    /// Constructs the default persistent store located in `%LOCALAPPDATA%\WireWave\identity.json`.
    pub fn default_store() -> Result<Self, IdentityError> {
        let path = default_identity_path()?;
        Ok(Self::new(path))
    }

    /// Constructs an identity store at a specific filesystem path.
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Fetches the existing identity or securely generates and persists a new one.
    pub fn fetch_or_create(&self) -> Result<DeviceIdentity, IdentityError> {
        fetch_or_create_at(&self.path)
    }

    /// Loads the stored identity if valid.
    pub fn load(&self) -> Option<DeviceIdentity> {
        load_identity(&self.path)
    }

    /// Saves an identity to disk atomically.
    pub fn save(&self, identity: &DeviceIdentity) -> Result<(), IdentityError> {
        save_identity(&self.path, identity)
    }
}

/// Fetches the persisted device ID, or generates and saves one if absent.
pub fn fetch_or_create_device_id() -> Result<DeviceIdentity, IdentityError> {
    DeviceIdentityStore::default_store()?.fetch_or_create()
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

    let mut buf = [0u16; 64];
    let mut size = (buf.len() * std::mem::size_of::<u16>()) as u32;

    let status = unsafe {
        RegGetValueW(
            HKEY_LOCAL_MACHINE,
            w!("SOFTWARE\\Microsoft\\Cryptography"),
            w!("MachineGuid"),
            RRF_RT_REG_SZ,
            None,
            Some(buf.as_mut_ptr().cast()),
            Some(&mut size),
        )
    };

    if status.is_ok() {
        let len = (size as usize / std::mem::size_of::<u16>()).saturating_sub(1);
        String::from_utf16(&buf[..len]).ok()
    } else {
        None
    }
}

#[cfg(not(windows))]
fn read_platform_machine_seed() -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn salted_hash_is_deterministic_and_non_raw() {
        let seed = "{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}";
        let id1 = derive_salted_id(seed);
        let id2 = derive_salted_id(seed);

        assert_eq!(id1, id2);
        assert!(id1.starts_with("wwd_"));
        assert_eq!(id1.len(), 28);
        assert!(!id1.contains(seed));
    }

    #[test]
    fn fetch_or_create_is_idempotent() {
        let dir = std::env::temp_dir().join(format!("ww_test_id_{}", Uuid::new_v4()));
        let path = dir.join("identity.json");
        let store = DeviceIdentityStore::new(path.clone());

        let id1 = store.fetch_or_create().unwrap();
        let id2 = store.fetch_or_create().unwrap();

        assert_eq!(id1, id2);
        assert_eq!(id1.schema_version, 1);
        assert!(id1.device_id.starts_with("wwd_"));

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn corrupted_identity_is_backed_up_and_replaced() {
        let dir = std::env::temp_dir().join(format!("ww_test_corrupt_{}", Uuid::new_v4()));
        let path = dir.join("identity.json");
        let store = DeviceIdentityStore::new(path.clone());

        fs::create_dir_all(&dir).unwrap();
        fs::write(&path, b"not valid json").unwrap();

        let id = store.fetch_or_create().unwrap();
        assert!(id.device_id.starts_with("wwd_"));

        let entries: Vec<_> = fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        assert!(entries.iter().any(|name| name.contains("json.corrupt")));

        let _ = fs::remove_dir_all(dir);
    }
}
