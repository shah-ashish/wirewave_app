# agent.md — AI Coding Rules for WireWave

Read this file before modifying any code in this repository. These rules apply to **every stage and every feature**.

## 1. Architecture Boundaries (Hard Rules)
- **Vertical-slice monorepo**:
  - `core/<module>/{rust,android}` — shared infrastructure & contracts only. Must NEVER import `features/*` or `apps/*`.
  - `features/<slice>/{rust,android}` — self-contained vertical slices. May depend on `core/*`, NEVER on other `features/*`.
  - `apps/{desktop,android}` — thin composition shells only (DI, navigation, window chrome, foreground services, future ads/entitlements). Keep business logic out of `apps/*`.
- **Logic and UI live side-by-side in separate files** inside each section folder (e.g. `about_logic` + `about_ui`, `update_logic` + `update_ui`, `devices_logic` + `devices_ui`).
- **Additive stage work**: Never rewrite or regress working prior-stage modules when adding a new stage; extend via new sibling folders or thin shell wiring.

## 2. Single Sources of Truth (Never Duplicate)
- **Theme**: All colors, font sizes, radii, and spacing come from `config/theme.json` (`THEME` in Rust, `WireWaveTheme` in Kotlin). Never hardcode hex colors in UI files.
- **Config & Assets**: App name, icons, `app_id`, connection constants, OS minimums, and store URLs live in `config/wirewave.config.json`.
- **Versioning**: `version_name`, `version_code`, `update_endpoint`, and `update_app_ids` live in `version.json`. `app_id` and `update_app_id` MUST always remain distinct.

## 3. Coding, Security & Privacy Standards
- **No `if-else` ladders**: Use Rust `enum` / Kotlin `sealed interface` or `enum class` with exhaustive `match` / `when`.
- **DRY & Small Files**: Reuse shared primitives; keep files single-responsibility (target ≤ 150 non-blank lines).
- **Safety & Concurrency**:
  - Rust: Typed errors (`thiserror`), no `.unwrap()` in runtime paths, blocking I/O off the UI thread, zeroize sensitive keys on drop.
  - Kotlin: No `!!`, no empty `catch`, rethrow `CancellationException`, mutate `MutableStateFlow` strictly via `.update { }`, no `GlobalScope`.
- **Privacy & Isolation**: Never log `device_id`, OTPs, private keys, or session secrets. Never expose open HTTP/CORS endpoints or `exported="true"` internal Android services. Never commit keystores or credentials.

## 4. Before Finishing Any Task
1. Verify no cross-feature imports or `core -> feature` dependencies.
2. Verify zero hardcoded UI hex colors, zero `else if` ladders, and zero Kotlin `!!`.
3. Add/update unit tests (`cargo test --workspace --lib` and `./gradlew test`).
4. Report honestly what was verified locally vs. what requires Windows 11 / Android 11+ to run.
