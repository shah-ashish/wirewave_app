# DEVELOPER.md — WireWave Contributor Guide

WireWave is a **vertical-slice monorepo** targeting **Windows 11+ (Rust + GPUI)** and **Android 11+ (Kotlin + Jetpack Compose)**.

## 1. Repository Layout

```text
├── version.json                  # Single version & update_app_id authority
├── config/
│   ├── wirewave.config.json      # App name, icons, app_ids, OS gates, store URLs
│   └── theme.json                # Shared OLED color, typography & metric tokens
├── assets/brand/                 # Brand icons and vector marks
├── core/<name>/{rust,android}    # Shared infrastructure & contracts
├── features/<name>/{rust,android}# Isolated vertical slices (logic + UI paired)
└── apps/{desktop,android}        # Thin application shells & packaging
```

**Dependency direction**: `apps` → `features` → `core` → `config`. Features never import each other.

## 2. How to Add a New Feature Slice
1. Create `features/<name>/rust/` (Cargo crate) and `features/<name>/android/` (Gradle library).
2. Register the crate in root `Cargo.toml` and the module in `apps/android/settings.gradle.kts`.
3. Split every screen section into sibling **logic** and **UI** files (e.g., `foo/foo_logic.rs` + `foo/foo_ui.rs`).
4. Pull all styling from `THEME` / `WireWaveTheme` and all branding from `APP_CONFIG` / `AppConfig`.
5. Wire the slice into `apps/desktop` and `apps/android/app` shells only.

## 3. Changing Branding, Theme, or Version
- **Rename app / swap icon**: Edit `config/wirewave.config.json` and place assets in `assets/brand/`. Both Rust (`build.rs`) and Android (Gradle) sync automatically on the next build.
- **Adjust colors / typography**: Edit `config/theme.json` once; both platforms regenerate typed tokens.
- **Bump release version**: Update `version_name` and `version_code` in `version.json`, then push a `vX.Y.Z` tag.
- **Switch distribution mode**: Set `WIREWAVE_BUILD_CHANNEL=dev` (in-app manual updater) or `store` (Microsoft Store / Google Play redirect).

## 4. Local Setup & Commands

**Desktop (Windows 11+, Rust 1.85+, Visual Studio C++ Build Tools, NSIS 3.x for installer)**
```powershell
cargo test --workspace --lib
cargo run -p wirewave-desktop
```

**Android (JDK 17, Android SDK 34, Android 11+ device/emulator)**
```bash
cd apps/android
./gradlew test :app:assembleDebug
```

## 5. Code & PR Conventions
- Prefer enums / sealed interfaces with exhaustive `match` / `when` over `if-else` chains.
- Keep files small, DRY, and single-responsibility; separate pure logic from UI so it can be unit-tested on the JVM / `cargo test`.
- Never commit signing keystores or credentials; CI loads release secrets from GitHub Actions Secrets.
- CI is path-filtered—PRs touching only desktop or only Android run only the relevant pipeline.
