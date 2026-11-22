use crate::downloader::UpdateError;
use serde::Deserialize;
use std::time::Duration;
use wirewave_core_config::APP_CONFIG;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateManifest {
    pub version_code: u32,
    pub version_name: String,
    pub notes: Option<String>,
    pub desktop: Option<PlatformArtifact>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlatformArtifact {
    pub update_app_id: String,
    pub download_url: String,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateRelease {
    pub version_name: String,
    pub version_code: u32,
    pub download_url: String,
    pub sha256: Option<String>,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateCheckResult {
    UpToDate,
    Available(UpdateRelease),
}

pub fn check_for_update() -> Result<UpdateCheckResult, UpdateError> {
    let url = format!("{}/latest.json", APP_CONFIG.update_endpoint.trim_end_matches('/'));
    let user_agent = format!(
        "{}/{} ({})",
        APP_CONFIG.app_name, APP_CONFIG.version_name, APP_CONFIG.update_app_id
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|e| UpdateError::Network(e.to_string()))?;
    let manifest: UpdateManifest = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .header("X-Update-App-Id", APP_CONFIG.update_app_id)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|e| UpdateError::Network(e.to_string()))?
        .json()
        .map_err(|e| UpdateError::InvalidManifest(e.to_string()))?;

    evaluate_manifest(&manifest, APP_CONFIG.version_code, APP_CONFIG.update_app_id)
}

/// Pure manifest evaluation separated for deterministic unit testing.
pub fn evaluate_manifest(
    manifest: &UpdateManifest,
    current_version_code: u32,
    expected_update_app_id: &str,
) -> Result<UpdateCheckResult, UpdateError> {
    if manifest.version_code <= current_version_code {
        return Ok(UpdateCheckResult::UpToDate);
    }

    let (download_url, sha256) = match &manifest.desktop {
        Some(artifact) if artifact.update_app_id == expected_update_app_id => {
            let sha256 = artifact
                .sha256
                .as_deref()
                .map(str::trim)
                .filter(|value| {
                    value.len() == 64 && value.chars().all(|character| character.is_ascii_hexdigit())
                })
                .ok_or_else(|| {
                    UpdateError::InvalidManifest(
                        "desktop artifact requires a valid SHA-256 checksum".to_string(),
                    )
                })?;
            (artifact.download_url.clone(), Some(sha256.to_string()))
        }
        Some(artifact) => {
            return Err(UpdateError::UpdateAppIdMismatch {
                expected: expected_update_app_id.to_string(),
                actual: artifact.update_app_id.clone(),
            });
        }
        None => {
            return Err(UpdateError::InvalidManifest(
                "manifest missing verified desktop artifact".to_string(),
            ))
        }
    };

    Ok(UpdateCheckResult::Available(UpdateRelease {
        version_name: manifest.version_name.clone(),
        version_code: manifest.version_code,
        download_url,
        sha256,
        notes: manifest.notes.clone().unwrap_or_default(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest(version_code: u32, update_app_id: &str) -> UpdateManifest {
        UpdateManifest {
            version_code,
            version_name: "1.1.0".to_string(),
            notes: Some("Stage 2".to_string()),
            desktop: Some(PlatformArtifact {
                update_app_id: update_app_id.to_string(),
                download_url: "https://example.com/wirewave.exe".to_string(),
                sha256: Some("0".repeat(64)),
            }),
        }
    }

    #[test]
    fn newer_matching_manifest_returns_available() {
        let m = sample_manifest(10100, "ww-upd-win");
        let res = evaluate_manifest(&m, 10000, "ww-upd-win").expect("valid");
        assert!(matches!(res, UpdateCheckResult::Available(_)));
    }

    #[test]
    fn mismatched_update_app_id_is_rejected() {
        let m = sample_manifest(10100, "wrong-id");
        let err = evaluate_manifest(&m, 10000, "ww-upd-win").unwrap_err();
        assert!(matches!(err, UpdateError::UpdateAppIdMismatch { .. }));
    }

    #[test]
    fn same_or_older_version_returns_up_to_date() {
        let m = sample_manifest(10000, "ww-upd-win");
        let res = evaluate_manifest(&m, 10000, "ww-upd-win").expect("valid");
        assert_eq!(res, UpdateCheckResult::UpToDate);
    }

    #[test]
    fn missing_checksum_is_rejected() {
        let mut manifest = sample_manifest(10100, "ww-upd-win");
        manifest.desktop.as_mut().unwrap().sha256 = None;
        assert!(matches!(
            evaluate_manifest(&manifest, 10000, "ww-upd-win"),
            Err(UpdateError::InvalidManifest(_))
        ));
    }

    #[test]
    fn malformed_checksum_is_rejected() {
        let mut manifest = sample_manifest(10100, "ww-upd-win");
        manifest.desktop.as_mut().unwrap().sha256 = Some("not-a-sha256".to_string());
        assert!(matches!(
            evaluate_manifest(&manifest, 10000, "ww-upd-win"),
            Err(UpdateError::InvalidManifest(_))
        ));
    }

    #[test]
    fn missing_desktop_artifact_is_rejected() {
        let mut manifest = sample_manifest(10100, "ww-upd-win");
        manifest.desktop = None;
        assert!(matches!(
            evaluate_manifest(&manifest, 10000, "ww-upd-win"),
            Err(UpdateError::InvalidManifest(_))
        ));
    }
}
