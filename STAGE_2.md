# WireWave — Stage 2: Connection, Persistence & Security

## 1. What Stage 2 Adds (On Top of Stage 1)
Stage 2 introduces **secure, WireWave-exclusive device discovery, OTP pairing, persistent Trust Agreements, priority arbitration, and background session persistence** over **Wi-Fi (LAN)** and **Bluetooth (RFCOMM SPP)**—without touching Stage 1's `about/`, `update/`, `core/identity`, `core/compatibility`, or `core/updater`.

### Key Capabilities
1. **App-to-App Exclusivity (No Open HTTP/CORS)**:
   - Binary length-prefixed framing (`WW2F` magic) over raw TCP `:3030` and private Bluetooth SPP UUID (`8f6a2e40-9b1c-4d3e-a712-5c4e9f8a1b20`).
   - Discovery uses HMAC-authenticated binary UDP packets (`WW2D` magic on `:3032`). Non-WireWave clients and browsers are rejected on byte 1.
   - All decrypted session data stays strictly in private process memory; Android services/providers are `exported="false"` with `allowBackup="false"`.
2. **1-to-1 Active Lock & Priority Preemption**:
   - Desktop holds a single `SessionState` (`Idle | Pairing | Connected`). Untrusted requests while busy are silently rejected (`Busy`).
   - Server priority (`Wi-Fi (Default)` or `Bluetooth`, configurable in Desktop Settings) allows a higher-priority link from a trusted device to cleanly preempt a lower-priority link.
3. **6-Digit OTP First-Time Pairing & Direct Trusted Reconnect**:
   - **First connect**: Desktop displays a 6-digit OTP (60s TTL, max 3 attempts) with countdown and Cancel. Phone enters the code on the custom keypad. Verification uses ephemeral **X25519 + OTP-bound HMAC-SHA256** (raw OTP is never sent over the wire) + mutual **Ed25519** signatures.
   - **Trust Agreement**: Persisted by `device_id` (never IP) in `%LOCALAPPDATA%\WireWave\trust.json` on Windows and private `SharedPreferences` on Android.
   - **Reconnect**: Trusted devices authenticate via a 32-byte random challenge signature—zero OTP re-entry across IP changes or transports.
   - **Encryption**: All post-handshake frames use **ChaCha20-Poly1305** with monotonic replay-blocking nonces; keys are zeroized on disconnect.
4. **Persistence**:
   - **Android**: `WireWaveConnectionService` (`foregroundServiceType="connectedDevice"`) holds `WifiLock` + partial `WakeLock` while connected so app switches and screen-off do not drop the link. Controlled by Settings toggles (`Auto-connect`, `Keep alive in background`).
   - **Desktop**: Dedicated background Tokio runtime decoupled from GPUI UI + `Keep alive in background` setting.
5. **Active Services (UI-Only Placeholders)**:
   - Both Android and Desktop Connected Home screens render `Speaker`, `Mic`, `Keyboard`, and `Mouse` with a **`Coming soon`** badge until Stage 3.

---

## 2. New Vertical Modules Added
```text
core/
├── crypto/{rust,android}       # Ed25519 identity keys, X25519, OTP HMAC, ChaCha20-Poly1305
├── transport/{rust,android}    # WW2F binary framing, TransportKind, TransportPriority
├── discovery/{rust,android}    # Signed WW2D UDP responder/scanner + Bluetooth SPP discovery
└── session/{rust,android}      # 1-to-1 SessionLock, TrustStore, PriorityArbitrator, Heartbeat

features/
├── connection/{rust,android}   # Desktop Home (Idle/Pairing/Connected) & Android (Discovery/OTP/Connected)
└── settings/{rust,android}     # Added connection/ and devices/ sections (about/ and update/ untouched)
```

---

## 3. What to Expect & How to Debug

| Scenario | Expected Behavior |
| :--- | :--- |
| **Desktop idle** | Opens on Home screen showing `Ready to connect`, PC hostname, and `Wi-Fi` / `Bluetooth` listener pills. |
| **Android scan** | Automatically scans on launch. Shows `No Computer Found` if none detected, or `Nearby Computers` with `Ready on Wi-Fi` / `Ready on Bluetooth`. |
| **First-time pairing** | Tap `Connect` → Desktop pops up 6-digit OTP + 60s bar → Phone opens `Enter pairing code` → Enter 6 digits → Both transition to Connected Home (`Coming soon` service badges). |
| **Direct reconnect** | Disconnect, then tap `Connect` again → Connects immediately with **no OTP**. |
| **Block by `device_id`** | Desktop Settings → `DEVICES` → `Connected` or `Trusted` → click `Block`. Phone moves to `Blocked` tab and future attempts return *"This phone is blocked by the computer."* |
| **Priority switch** | Desktop Settings → `CONNECTION` → change `Connection priority` dropdown (`Wi-Fi (Default)` / `Bluetooth`); persisted in `%LOCALAPPDATA%\WireWave\connection_settings.json`. |
| **App switch / screen off** | Android shows low-priority ongoing notification (`WireWave connected`) and keeps heartbeat alive until explicit `Disconnect`. |

### Debug Commands
```powershell
# Desktop unit tests (crypto, transport, discovery, trust_store, plus all Stage 1 tests)
cargo test --workspace --lib

# Run Desktop with info logs (note: device_id, OTPs, and keys are never logged)
$env:RUST_LOG="info"
cargo run -p wirewave-desktop
```

```bash
# Android unit tests + debug APK
cd apps/android
./gradlew test :app:assembleDebug
```

> **Release tip**: When ready to cut the Stage 2 release (`v1.1.0`), bump `version_name` to `"1.1.0"` and `version_code` to `10100` in `version.json` and update the Stage 1 version assertion in `core/config/rust/src/lib.rs`, then push tag `v1.1.0`.
