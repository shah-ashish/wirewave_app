# Working on WireWave

WireWave uses Rust structs and traits on desktop, Kotlin classes and interfaces on Android,
and composition on both sides. Give each object one responsibility and pass its dependencies
explicitly. Add an interface when it defines a useful boundary, not for every class.

## Module boundaries

| Location | Responsibility | Dependencies |
| --- | --- | --- |
| `core/<module>/{rust,android}` | Infrastructure, contracts, shared UI | Other core modules |
| `features/<feature>/{rust,android}` | Feature behavior and feature-specific UI | Core modules; no sibling features |
| `apps/desktop`, `apps/android/app` | Construct dependencies, compose screens, manage platform lifecycle | Core and feature modules |

Read [agent.md](../agent.md) for repository coding rules.
Shared UI lives in `core/ui`; feature-specific controls stay with their feature.
Android mouse code currently lives inside `features/connection/android`, while desktop mouse
code has its own `features/mouse/rust` crate. Do not assume every feature has symmetric paths.

## Where to find the refactored code

| Concern | Desktop | Android |
| --- | --- | --- |
| App composition | `apps/desktop/src/desktop_features.rs` | `AppContainer.kt`, `WireWaveRoot.kt` |
| Engine lifecycle wiring | `DesktopFeatures` and session feature registry | `SessionFeatureEffects.kt` |
| Shared UI | `FeatureStatusTile` | `FeatureCircleTile`, `CircleIconButton`, `FeatureNavigationList` |
| Session models | `core/session/rust/src/session_state.rs` | `ClientSessionState.kt` and preference model files |
| Session orchestration | `SessionManager` | `SessionController` |
| Transport setup | Existing transport listeners | `SessionTransportRunner` |
| Feature integration | `FeatureHost`, `FeatureRegistry`, `stream_hosts/` | `RemoteFeatureStateStore` |
| Mouse protocol | `features/mouse/rust/src/packet.rs` | `MousePacketEncoder.kt` |
| Remote keyboard input | Existing session keyboard handling | `RemoteKeyboardInput`, `KeyboardInputController` |

Android app composition files are in
`apps/android/app/src/main/java/com/wirewave/android/`.
Android session files are in
`core/session/android/src/main/java/com/wirewave/core/session/`.

## Adding a feature

1. Put feature-specific work in its feature module. Separate rendering from controllers,
   services, and persistence only where those responsibilities exist.
2. Register new Rust crates in the root Cargo workspace. Register Android modules in
   `apps/android/settings.gradle.kts` and declare dependencies in the consuming module.
3. Construct services in the app composition layer. Pass state and callbacks to UI.
4. Reuse shared UI components when their behavior and appearance fit. Keep ownership of
   permissions, navigation, network calls, and persistence outside those components.
5. Verify the affected behavior and update documentation for any changed contracts.

For desktop peer streaming, implement `FeatureHost` and register the service with
`FeatureRegistry`. `DesktopFeatures` owns the current service instances and their tiles.
The registry is keyed by channel: a second registration for the same channel replaces the first.

A new peer stream still requires matching channel IDs, control messages, permission policy,
and dispatch support on both platforms. Registration alone does not add protocol support.
The Android controller currently dispatches explicit speaker, microphone, and mouse messages.
`RemoteFeatureStateStore` shares their parsing and reset behavior; it is not a plugin loader.

## Future collection and server upload modules

These modules are not implemented by the refactoring. A suitable starting design is a feature
controller coordinating a collector and an upload client. Keep collection rules and payload
models in the feature. Move networking into core only when it is shared infrastructure.
Construct dependencies in `AppContainer` or the desktop composition layer.

An upload to a server is separate from the phone-to-desktop UDP stream protocol.
Do not force it through `FeatureHost` unless it actually participates in that protocol.
Define collection scope, user controls, server authentication, retry behavior, and retention
before implementing the feature.

## Refactoring rules

- Preserve public import paths when moving existing types.
- Preserve cancellation, cleanup order, error behavior, and wire formats.
- Keep UI stateless where possible; use explicit callbacks rather than matching display labels.
- Prefer clear names and focused methods over inheritance trees or generic frameworks.
- Use configuration and theme tokens for new styling and constants. Some extracted UI still
  retains existing literal dimensions and colors; extraction does not mean all styling is normalized.
- Measure before claiming performance improvements. These changes primarily improve organization.
- Avoid unrelated edits and broad formatting changes in an individual chunk.

## Verification

On Windows, from the repository root:

```powershell
cargo check --workspace --all-targets
cargo test --workspace --lib
```

From `apps/android`:

```powershell
.\gradlew.bat test :app:assembleDebug
```

Unit tests cover packet encoding/decoding and remote keyboard edit translation without
injecting desktop input. A successful build does not verify device interaction.

After relevant changes, check pairing, trusted reconnect, cancellation, disconnect, feature
toggles, mouse gestures, keyboard input, settings, and foreground/background transitions on
the actual devices. Check both Wi-Fi and Bluetooth when transport code changes.

During this refactoring session, the user runs build/test commands. Agent diff checks are
reported separately from user-reported build results and manual device verification.
