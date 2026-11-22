#[cfg(windows)]
fn main() {
    use serde::Deserialize;
    use std::{env, fs, path::PathBuf};

    #[derive(Deserialize)]
    struct RootConfig {
        app_name: String,
        assets: AssetConfig,
    }
    #[derive(Deserialize)]
    struct AssetConfig {
        icon_desktop_ico: String,
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.join("../..").canonicalize().expect("repo root");
    let config_path = repo_root.join("config/wirewave.config.json");
    println!("cargo:rerun-if-changed={}", config_path.display());

    let cfg: RootConfig =
        serde_json::from_str(&fs::read_to_string(&config_path).expect("read config"))
            .expect("parse config");

    let ico_path = repo_root.join(&cfg.assets.icon_desktop_ico);
    println!("cargo:rerun-if-changed={}", ico_path.display());

    let mut res = winresource::WindowsResource::new();
    res.set_icon(ico_path.to_str().expect("utf-8 icon path"));
    res.set("ProductName", &cfg.app_name);
    res.set("FileDescription", &cfg.app_name);
    if let Err(err) = res.compile() {
        println!("cargo:warning=winresource skipped: {err}");
    }
}

#[cfg(not(windows))]
fn main() {}
