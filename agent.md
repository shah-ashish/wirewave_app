# AGENT.md — AI Coding Rules for WireWave

Read this file before modifying any code in this repository. These rules apply to **every stage and every feature**.

## 1. Architecture Boundaries (Hard Rules)
- **Vertical-slice monorepo**:
  - `core/<module>/{rust,android}` — shared infrastructure & contracts only. Must NEVER import `features/*` or `apps/*`.
  - `features/<slice>/{rust,android}` — self-contained vertical slices. May depend on `core/*`, NEVER on other `features/*`.
  - `apps/{desktop,android}` — thin composition shells only (wiring DI, navigation, window chrome, future ads/entitlements). Keep business logic out of `apps/*`.
- **Logic and UI live side-by-side in separate files** inside each section folder:
  - Example: `features/<slice>/<platform>/<section>/<section>_logic` + `<section>_ui`.
  - Related operations stay together in one cohesive module (e.g., `fetch` + `generate` live in the same identity module).

## 2. Single Sources of Truth (Never Duplicate)
- **Theme**: All colors, font sizes, radii, and spacing come from `config/theme.json` (via `THEME` in Rust, `WireWaveTheme` in Kotlin). Never hardcode hex colors or magic layout numbers in UI files.
- **Branding & IDs**: App name, icons, `app_id`, OS minimums, and store URLs live in `config/wirewave.config.json`.
- **Versioning & Updates**: `version_name`, `version_code`, `update_endpoint`, and `update_app_ids` live in `version.json`.
- **ID Invariant**: `app_id` (OS package) and `update_app_id` (updater channel) MUST always remain distinct.

## 3. Coding Standards (Senior Level)
- **No `if-else` ladders**: Model states and variants with Rust `enum` or Kotlin `sealed interface` / `enum class` and use exhaustive `match` / `when`.
- **DRY**: Extract shared UI primitives (cards, rows, pills) and shared logic once used in 2+ places. Never copy-paste styling or validation blocks.
- **Small, focused files**: Keep each file single-responsibility (target ≤ 150 non-blank lines). Split by subtask when a file grows.
- **Safety & Concurrency**:
  - Rust: Typed errors (`thiserror`), no `.unwrap()` in runtime paths (only tests or build-time invariants), run blocking I/O on background executors.
  - Kotlin: No `!!`, no empty `catch`, rethrow `CancellationException`, mutate `MutableStateFlow` strictly via `.update { }` (never `.value = .value.copy(...)`), no `GlobalScope`.
- **Idiomatic platform APIs**: Use built-in Rust (`std`, `gpui`) and Kotlin/Compose APIs before adding third-party crates or libraries. Never add dependencies without clear need.
- **No secrets in Git**: Never commit keystores, passwords, tokens, or API keys.

## 4. Before Finishing Any Task
1. Verify no cross-feature imports or `core -> feature` dependencies were introduced.
2. Verify zero hardcoded hex colors in UI code, zero `else if` ladders, and zero Kotlin `!!`.
3. Add or update unit tests for pure logic (`cargo test --workspace --lib` and `./gradlew test`).
4. Report honestly what was built/tested vs. what requires a target OS (Windows 11 / Android 11+) to run.
