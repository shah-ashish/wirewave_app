# developer.md — WireWave Contributor Guide

WireWave is a **vertical-slice monorepo** targeting **Windows 11+ (Rust + GPUI)** and **Android 11+ (Kotlin + Jetpack Compose)**.

## 1. Repository Layout
```text
├── version.json                  # Single version & update_app_id authority
├── config/
│   ├── wirewave.config.json      # Brand, icons, app_ids, connection config, OS gates, store URLs
│   └── theme.json                # Shared OLED color, typography & metric tokens
├── assets/brand/                 # Brand icons and SVG marks
├── core/<name>/{rust,android}    # Shared infrastructure (config, identity, crypto, transport, discovery, session, updater)
├── features/<name>/{rust,android}# Isolated vertical slices (logic + UI paired)
└── apps/{desktop,android}        # Thin application shells, foreground service & NSIS installer
```
**Dependency direction**: `apps` → `features` → `core` → `config`. Features never import each other.

## 2. Adding a Feature Slice
1. Create `features/<name>/rust/` and `features/<name>/android/` and register them in root `Cargo.toml` and `apps/android/settings.gradle.kts`.
2. Pair every screen section as sibling logic + UI files (`<section>_logic` + `<section>_ui`).
3. Pull all styling from `THEME` / `WireWaveTheme` and all constants from `APP_CONFIG` / `AppConfig`.
4. Wire navigation and DI only in `apps/desktop` and `apps/android/app`.

## 3. Config, Branding & Releases
- **Rename / swap icons**: Update `config/wirewave.config.json` and `assets/brand/`. Rust `build.rs` and Gradle auto-sync on build.
- **Theme changes**: Edit `config/theme.json` once; both platforms regenerate typed tokens.
- **Release**: Bump `version_name` & `version_code` in `version.json`, then push tag `vX.Y.Z`.
- **Channel**: Set `WIREWAVE_BUILD_CHANNEL=dev` (in-app updater) or `store` (MS Store / Play Store redirect).

## 4. Local Build & Test
- **Desktop (Windows 11+, Rust 1.85+, VS C++ Build Tools)**: `cargo test --workspace --lib` and `cargo run -p wirewave-desktop`
- **Android (JDK 17, Android SDK 34, Android 11+)**: `cd apps/android && ./gradlew test :app:assembleDebug`

## 5. Conventions
- Prefer enums / sealed interfaces + exhaustive `match` / `when` over `if-else` chains.
- Never log `device_id`, OTP codes, or cryptographic keys; never commit signing keystores.
- CI is path-filtered by platform and cached via `sccache`, `rust-cache`, and Gradle build cache.
