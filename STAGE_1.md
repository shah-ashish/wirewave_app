# WireWave — Stage 1 (`v1.0.0`) Guide

## 1. What Stage 1 Is About
Stage 1 establishes the **production-grade foundation** for both **Windows 11+ (Rust + GPUI)** and **Android 11+ (Kotlin + Jetpack Compose)** so real test users can install `v1.0.0` once and pull every upcoming stage (Stage 2 connection, Stage 3 features) directly through the in-app manual updater.

### Core Capabilities in Stage 1
1. **Background Bootstrap (Runs in both `dev` and `store` modes)**:
   - **OS Compatibility Check**:
     - Windows: Queries real build number via `RtlGetVersion` (`>= 22000`, Windows 11+). NSIS installer also blocks Windows 10 at install time.
     - Android: Enforces `minSdk = 30` (Android 11+) at install time and verifies `Build.VERSION.SDK_INT >= 30` at runtime.
     - If incompatible, a **non-dismissible popup** (`Device Not Compatible`) blocks usage cleanly.
   - **Persistent Unique Device ID**:
     - Windows: Stored in `%LOCALAPPDATA%\WireWave\identity.json` (preserved by NSIS on uninstall). Initial seed hashes Windows `MachineGuid` with an app-specific salt (`wwd_<24-hex>`), so even a manual folder wipe yields the same ID on the same PC.
     - Android: Hashes `Settings.Secure.ANDROID_ID` (SSAID, scoped to device + release signing key) with an app-specific salt and caches it in `SharedPreferences`. Survives uninstall/reinstall when signed with the same release keystore.
2. **Three Distinct IDs (Zero Coupling)**:
   - **App ID** (OS package): `oklum.com.wirewave.desktop` / `oklum.com.wirewave.android`
   - **Update App ID** (Updater channel): `ww-upd-win` / `ww-upd-and` (asserted distinct at compile time!)
   - **Device ID** (Persistent hardware-scoped identity): `wwd_<24-hex>`
3. **Single Source of Truth for Config, Assets & Theme**:
   - `version.json`: `version_name`, `version_code`, `update_endpoint`, `update_app_ids`.
   - `config/wirewave.config.json`: `app_name`, brand icon paths, `app_id`, min OS rules, store URLs.
   - `config/theme.json`: Global colors, typography, radii, and spacing.
   - Both Rust (`core/config/rust/build.rs` + `apps/desktop/build.rs`) and Android (`core/config/android/build.gradle.kts` + `apps/android/app/build.gradle.kts`) read these files automatically at build time.
4. **Settings Screen (Exact UI Match)**:
   - **Update Section** (`features/settings/*/update/`): Manual flow in `dev` mode:
     `Check for updates` → `Download` → live progress bar (`%`) → SHA-256 verification → `Install`.
     In `store` mode (`WIREWAVE_BUILD_CHANNEL=store`), redirects straight to Microsoft Store / Google Play.
   - **About Section** (`features/settings/*/about/`): Displays `App name`, `Device name`, `Connected phone` / `Connected PC` (`Not connected` in Stage 1), `App ID`, and green `✓ Supported` pill.

---

## 2. Repository Architecture (Logic + UI Paired Side-by-Side)

```text
├── version.json                      # Single version & update_app_id authority
├── config/
│   ├── wirewave.config.json          # Brand, icon paths, App IDs, OS gates, store URLs
│   └── theme.json                    # Global OLED theme tokens
├── assets/brand/                     # Single folder for icons & SVGs
├── core/
│   ├── config/{rust,android}         # Build-time codegen for AppConfig + Theme
│   ├── identity/{rust,android}       # fetch_or_create + generate in one cohesive module
│   ├── compatibility/{rust,android}  # Win11 >= 22000 & Android 11 >= API 30 checks
│   └── updater/{rust,android}        # Manifest check, atomic .part download + SHA-256, install
├── features/
│   └── settings/{rust,android}
│       ├── about/                    # about_logic + about_ui (sibling files)
│       ├── update/                   # update_logic + update_ui (sibling files)
│       ├── components                # DRY cards, rows, pills, labels
│       ├── compat_popup              # Blocking incompatibility modal
│       └── settings_screen           # Composes sections
└── apps/
    ├── desktop/                      # Pure Rust + GPUI shell + NSIS installer
    └── android/                      # Thin Kotlin + Compose app shell + AppContainer DI
```

---

## 3. What We Expect When Running Stage 1

| Check | Expected Result |
| :--- | :--- |
| **Desktop launch on Windows 11** | Dark OLED window opens with custom titlebar (yellow WireWave icon + `WireWave` + `— □ ×`), left sidebar (blue active gear + monitor with green dot), `Version 1.0.0` card, and `ABOUT` card showing `oklum.com.wirewave.desktop` and green `✓ Supported` pill. |
| **Android launch on Android 11+** | Pure black edge-to-edge screen with `< Back`, `Version 1.0.0` card (`Check for updates`), and `ABOUT` card showing `oklum.com.wirewave.android` and green `✓ Supported` pill. |
| **Uninstall & reinstall persistence** | On Windows, `%LOCALAPPDATA%\WireWave\identity.json` remains intact and logs the exact same `device_id`. On Android (same keystore), `WireWaveBootstrap` logs the exact same `device_id`. |
| **In-app update (`dev` mode)** | Clicking `Check for updates` queries `<update_endpoint>/latest.json`. If `version_code > 10000` and `update_app_id` matches, button changes to `Download`. Clicking `Download` streams to `.part` with live `%` progress bar, verifies SHA-256, and switches to `Install`. Clicking `Install` opens the NSIS `.exe` or Android package installer. |
| **Store build (`store` mode)** | Building with `WIREWAVE_BUILD_CHANNEL=store` makes `Check for updates` open Microsoft Store / Google Play directly. |
| **Incompatible OS** | Non-dismissible `Device Not Compatible` popup appears over the screen, and the Compatibility pill shows red `Unsupported`. |

---

## 4. How to Build, Run & Debug

### Windows Desktop (Rust + GPUI)
**Prerequisites**: Windows 11, Rust stable (`1.85+` for edition 2024), Visual Studio C++ Build Tools, NSIS 3.x (only for packaging `.exe`).

```powershell
# 1. Run all Rust unit tests (config, identity, compatibility, updater)
cargo test --workspace --lib

# 2. Run Desktop app with verbose bootstrap logs
$env:RUST_LOG="info"
$env:WIREWAVE_BUILD_CHANNEL="dev"   # or "store"
cargo run -p wirewave-desktop

# 3. Verify persisted Device ID file
Get-Content "$env:LOCALAPPDATA\WireWave\identity.json"

# 4. Build release installer (.exe)
cargo build --release -p wirewave-desktop
makensis apps/desktop/installer/wirewave.nsi
```

### Android (Kotlin + Jetpack Compose)
**Prerequisites**: JDK 17, Android SDK 34, Android 11+ device or emulator.

```bash
cd apps/android

# 1. Run JVM unit tests (identity, compatibility, update manifest)
./gradlew test

# 2. Build & install debug APK (dev mode)
WIREWAVE_BUILD_CHANNEL=dev ./gradlew :app:installDebug

# 3. Watch bootstrap & updater logs via adb
adb logcat -s WireWaveBootstrap:I
```

### Debugging Common Issues
1. **Testing the Updater Locally Without Publishing a Release**:
   - Serve a local `latest.json` (or point `version.json -> update_endpoint` to a test Cloudflare Worker) with `"version_code": 10100`, matching `"update_app_id"` (`"ww-upd-win"` / `"ww-upd-and"`), and a valid `"sha256"`.
   - Test failure paths by changing `"update_app_id"` (should show mismatch error) or `"sha256"` (should delete `.part` file and show checksum failure).
2. **Testing the Incompatibility Popup**:
   - Temporarily set `min_windows_build` to `99999` or `min_sdk` check in `CompatibilityChecker` to `99` and launch to verify the blocking modal and red `Unsupported` pill.
3. **Android Release Signing in CI**:
   - Store your keystore as GitHub Actions secrets (`ANDROID_KEYSTORE_BASE64`, `ANDROID_KEYSTORE_PASSWORD`, `ANDROID_KEY_ALIAS`, `ANDROID_KEY_PASSWORD`) instead of committing `.jks` files to Git.
4. **Switching Branding or Icons**:
   - Edit `app_name` or icon paths in `config/wirewave.config.json`, or colors/sizes in `config/theme.json`, then rebuild—both platforms regenerate their constants automatically.
