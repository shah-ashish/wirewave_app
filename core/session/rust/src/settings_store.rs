use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use wirewave_core_config::APP_CONFIG;
use wirewave_core_transport::TransportPriority;

const PREFS_FILE_NAME: &str = "connection_settings.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionPreferences {
    pub priority: TransportPriority,
    pub keep_alive_in_background: bool,
}

impl Default for ConnectionPreferences {
    fn default() -> Self {
        Self {
            priority: TransportPriority::from_config(APP_CONFIG.connection.default_priority),
            keep_alive_in_background: true,
        }
    }
}

pub struct PreferencesStore {
    path: PathBuf,
    prefs: ConnectionPreferences,
}

impl PreferencesStore {
    pub fn load_default() -> Self {
        let path = BaseDirs::new()
            .map(|b| b.data_local_dir().join(APP_CONFIG.app_name).join(PREFS_FILE_NAME))
            .unwrap_or_else(|| PathBuf::from(PREFS_FILE_NAME));
        Self::load_at(path)
    }

    pub fn load_at(path: PathBuf) -> Self {
        let prefs = fs::read_to_string(&path)
            .ok()
            .and_then(|raw| serde_json::from_str::<ConnectionPreferences>(&raw).ok())
            .unwrap_or_default();
        Self { path, prefs }
    }

    pub fn get(&self) -> ConnectionPreferences {
        self.prefs
    }

    pub fn set_priority(&mut self, priority: TransportPriority) {
        self.prefs.priority = priority;
        self.persist();
    }

    pub fn set_keep_alive(&mut self, enabled: bool) {
        self.prefs.keep_alive_in_background = enabled;
        self.persist();
    }

    fn persist(&self) {
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&self.prefs) {
            let tmp = self.path.with_extension("json.tmp");
            if fs::write(&tmp, json).is_ok() {
                let _ = fs::rename(&tmp, &self.path);
            }
        }
    }
}
