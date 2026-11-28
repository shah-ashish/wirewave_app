use crate::{ChannelId, health::LiveLatency};
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

// Serialized, bounded local logs. Called only from the monitor's blocking worker.
pub(crate) fn record(
    channel: ChannelId,
    stream_id: u32,
    status: LiveLatency,
    rtt_ms: Option<u64>,
    event: &str,
) {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let _guard = LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
    let Some(base) = directories::BaseDirs::new() else {
        return;
    };
    let dir = base
        .data_local_dir()
        .join(wirewave_core_config::APP_CONFIG.app_name)
        .join("logs");
    let entry = serde_json::json!({
        "timestamp_unix_ms": SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis(),
        "service": format!("{channel:?}"), "stream_id": stream_id,
        "status": status.label(), "latency_ms": rtt_ms, "event": event,
    });
    if let Err(error) = append(&dir, &entry.to_string()) {
        tracing::warn!(%error, "Unable to write service latency log");
    }
}

fn append(dir: &Path, line: &str) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    let path = dir.join("service-health.jsonl");
    if fs::metadata(&path).is_ok_and(|meta| meta.len() >= 5 * 1024 * 1024) {
        let backup = dir.join("service-health.previous.jsonl");
        if backup.exists() {
            fs::remove_file(&backup)?;
        }
        fs::rename(&path, backup)?;
    }
    writeln!(
        OpenOptions::new().create(true).append(true).open(path)?,
        "{line}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn appends_valid_lines_and_rotates_bounded_log() {
        let dir = std::env::temp_dir().join(format!("wirewave-health-{}", std::process::id()));
        append(&dir, r#"{"event":"sample"}"#).unwrap();
        let path = dir.join("service-health.jsonl");
        let content = fs::read_to_string(&path).unwrap();
        assert!(
            content
                .lines()
                .all(|line| serde_json::from_str::<serde_json::Value>(line).is_ok())
        );
        OpenOptions::new()
            .write(true)
            .open(&path)
            .unwrap()
            .set_len(5 * 1024 * 1024)
            .unwrap();
        append(&dir, r#"{"event":"down"}"#).unwrap();
        assert!(dir.join("service-health.previous.jsonl").exists());
        assert_eq!(fs::read_to_string(path).unwrap(), "{\"event\":\"down\"}\n");
        fs::remove_dir_all(dir).unwrap();
    }
}
