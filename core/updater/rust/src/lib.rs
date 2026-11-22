pub mod downloader;
pub mod installer;
pub mod manifest;

pub use downloader::{download_update, verify_sha256, DownloadProgress, UpdateError};
pub use installer::{launch_installer, open_store_listing};
pub use manifest::{check_for_update, UpdateCheckResult, UpdateManifest, UpdateRelease};
