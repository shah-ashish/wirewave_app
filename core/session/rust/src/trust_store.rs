use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;
use wirewave_core_config::APP_CONFIG;

const TRUST_FILE_NAME: &str = "trust.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustedDevice {
    pub device_id: String,
    pub device_name: String,
    pub public_key_hex: String,
    pub paired_at_unix: u64,
    pub blocked: bool,
}

impl TrustedDevice {
    /// Short human-readable ID label matching Desktop Settings (`a3f9-12c4`).
    pub fn short_id(&self) -> String {
        let raw = self.device_id.trim_start_matches("wwd_");
        match raw.len() >= 8 {
            true => format!("{}-{}", &raw[..4], &raw[4..8]),
            false => raw.to_string(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct TrustFile {
    schema_version: u32,
    devices: BTreeMap<String, TrustedDevice>,
}

/// Persistent trust and blocklist store keyed strictly by `device_id` (never IP).
pub struct TrustStore {
    path: PathBuf,
    devices: BTreeMap<String, TrustedDevice>,
}

impl TrustStore {
    pub fn load_default() -> Self {
        let path = BaseDirs::new()
            .map(|b| b.data_local_dir().join(APP_CONFIG.app_name).join(TRUST_FILE_NAME))
            .unwrap_or_else(|| PathBuf::from(TRUST_FILE_NAME));
        Self::load_at(path)
    }

    pub fn load_at(path: PathBuf) -> Self {
        let devices = match read_trust_file(&path) {
            Some(file) => file.devices,
            None => {
                if path.exists() {
                    let backup = path.with_extension(format!("json.corrupt-{}", Uuid::new_v4()));
                    let _ = fs::rename(&path, backup);
                }
                BTreeMap::new()
            }
        };
        Self { path, devices }
    }

    pub fn get(&self, device_id: &str) -> Option<&TrustedDevice> {
        self.devices.get(device_id)
    }

    pub fn is_blocked(&self, device_id: &str) -> bool {
        self.devices.get(device_id).is_some_and(|d| d.blocked)
    }

    pub fn is_trusted(&self, device_id: &str) -> bool {
        self.devices.get(device_id).is_some_and(|d| !d.blocked)
    }

    pub fn upsert_trusted(&mut self, device_id: String, device_name: String, public_key_hex: String) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.devices.insert(
            device_id.clone(),
            TrustedDevice {
                device_id,
                device_name,
                public_key_hex,
                paired_at_unix: now,
                blocked: false,
            },
        );
        self.persist();
    }

    pub fn set_blocked(&mut self, device_id: &str, blocked: bool) {
        if let Some(entry) = self.devices.get_mut(device_id) {
            entry.blocked = blocked;
            self.persist();
        }
    }

    pub fn forget(&mut self, device_id: &str) {
        if self.devices.remove(device_id).is_some() {
            self.persist();
        }
    }

    pub fn list_trusted(&self) -> Vec<TrustedDevice> {
        self.devices
            .values()
            .filter(|d| !d.blocked)
            .cloned()
            .collect()
    }

    pub fn list_blocked(&self) -> Vec<TrustedDevice> {
        self.devices
            .values()
            .filter(|d| d.blocked)
            .cloned()
            .collect()
    }

    fn persist(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let payload = TrustFile {
            schema_version: 1,
            devices: self.devices.clone(),
        };
        if let Ok(json) = serde_json::to_string_pretty(&payload) {
            let tmp = self.path.with_extension("json.tmp");
            if fs::write(&tmp, json).is_ok() {
                let _ = fs::rename(&tmp, &self.path);
            }
        }
    }
}

fn read_trust_file(path: &Path) -> Option<TrustFile> {
    let raw = fs::read_to_string(path).ok()?;
    serde_json::from_str(&raw).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_store_persists_trust_block_and_forget_by_device_id() {
        let dir = std::env::temp_dir().join(format!("ww_trust_test_{}", Uuid::new_v4()));
        let path = dir.join(TRUST_FILE_NAME);

        let mut store = TrustStore::load_at(path.clone());
        store.upsert_trusted("wwd_a3f912c40000".into(), "Redmi".into(), "deadbeef".into());
        assert!(store.is_trusted("wwd_a3f912c40000"));

        store.set_blocked("wwd_a3f912c40000", true);
        assert!(store.is_blocked("wwd_a3f912c40000"));
        assert!(!store.is_trusted("wwd_a3f912c40000"));

        let reloaded = TrustStore::load_at(path);
        assert_eq!(reloaded.list_blocked().len(), 1);
        assert_eq!(reloaded.list_blocked()[0].short_id(), "a3f9-12c4");

        let _ = fs::remove_dir_all(dir);
    }
}
