# Graph Report - wirewave-stage2  (2026-11-23)

## Corpus Check
- 32 files · ~30,644 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1299 nodes · 2511 edges · 101 communities (43 shown, 58 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 56 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Lib librs
- Desktop Auto-Update Downloader
- Settings Management & UI
- GitHub CI Workflows
- ChaCha20-Poly1305 Cipher Suite
- Desktop Idle View & Pairing
- Update_logic arc
- Updatemanifest UpdateManifestkt
- Android Crypto & Keys
- Updatelogic RetryTarget
- Brand Identity & Vector Assets
- Settings Management & UI
- Windows Bluetooth RFCOMM Listener
- Application Lifecycle & Entry Points
- Discoveryscreen bluetooth
- Android MainActivity & Permissions
- Android About & Info Screen
- Compatibilitydialog arrangement
- Desktop Connected View & UI
- Android Foreground Connection Service
- Device Pairing & OTP Auth
- ChaCha20-Poly1305 Cipher Suite
- Device Pairing & OTP Auth
- Connectedhomescreen circleshape
- Android MainActivity & Permissions
- Android Crypto & Keys
- Lib asyncread
- Android Session Controller State
- Android Crypto & Keys
- Session_manager session_managerrs
- Data Storage & Persistence
- Desktop Auto-Update Downloader
- Updatesection clip
- UI Theme & Design Tokens
- Session_manager librs
- Android Crypto & Keys
- Mod modrs
- Connectionsection alignment
- Desktop AppShell & Navigation
- Hostscanner DiscoveredHost
- Appconfig AppConfigkt
- Desktop Connected View & UI
- Android Session Controller State
- Device Pairing & OTP Auth
- Ic_launcher_round WireWave Android App
- Release Automation Workflows
- Cargo wirewave-core-compat
- Assets app_config
- Desktop Idle View & Pairing
- Release Automation Workflows
- Release Automation Workflows
- Release Automation Workflows
- Desktop Idle View & Pairing
- Data Storage & Persistence
- Apkinstaller ApkInstaller
- Hostscanner DiscoveredHost
- Desktop Idle View & Pairing
- Release Automation Workflows
- About_logic aboutsnapshot
- Release Automation Workflows
- Release Automation Workflows
- Home Home Icon
- Phone Phone Icon
- Window-close Window Close Icon
- Window-maximize Window Maximize Icon
- Window-minimize Window Minimize Icon

## God Nodes (most connected - your core abstractions)
1. `SessionController` - 29 edges
2. `SettingsScreen` - 26 edges
3. `SessionManager` - 25 edges
4. `DiscoveredHost` - 23 edges
5. `AppContainer` - 21 edges
6. `UpdateError` - 19 edges
7. `UpdateController` - 18 edges
8. `CryptoError` - 17 edges
9. `TransportPriority` - 17 edges
10. `TrustStore` - 16 edges

## Surprising Connections (you probably didn't know these)
- `Active Services UI Placeholders` --references--> `Camera Icon`  [EXTRACTED]
  STAGE_2.md → assets/brand/icons/camera.svg
- `Active Services UI Placeholders` --references--> `Keyboard Icon`  [EXTRACTED]
  STAGE_2.md → assets/brand/icons/keyboard.svg
- `Active Services UI Placeholders` --references--> `Microphone Icon`  [EXTRACTED]
  STAGE_2.md → assets/brand/icons/mic.svg
- `Active Services UI Placeholders` --references--> `Mouse Icon`  [EXTRACTED]
  STAGE_2.md → assets/brand/icons/mouse.svg
- `Active Services UI Placeholders` --references--> `Speaker Icon`  [EXTRACTED]
  STAGE_2.md → assets/brand/icons/speaker.svg

## Import Cycles
- 2-file cycle: `core/crypto/rust/src/cipher.rs -> core/crypto/rust/src/lib.rs -> core/crypto/rust/src/cipher.rs`
- 2-file cycle: `core/crypto/rust/src/keys.rs -> core/crypto/rust/src/lib.rs -> core/crypto/rust/src/keys.rs`

## Hyperedges (group relationships)
- **Stage 2 Active Hardware Services UI Placeholders** — stage_2_active_services_placeholder, assets_brand_icons_speaker_icon, assets_brand_icons_mic_icon, assets_brand_icons_keyboard_icon, assets_brand_icons_mouse_icon, assets_brand_icons_camera_icon [EXTRACTED 1.00]
- **Desktop Window Titlebar Chrome Controls** — assets_brand_icons_window_minimize_icon, assets_brand_icons_window_maximize_icon, assets_brand_icons_window_close_icon [EXTRACTED 1.00]
- **Android Launcher Icon Density Suite** — apps_android_app_src_main_res_drawable_app_icon_app_icon, apps_android_app_src_main_res_mipmap_hdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_hdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_mdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_mdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_xhdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_xhdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_xxhdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_xxhdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_xxxhdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_xxxhdpi_ic_launcher_round_ic_launcher_round [INFERRED 0.95]
- **Single Source of Truth Monorepo Governance** — stage_1_single_source_of_truth, agent_single_sources_of_truth, developer_monorepo_layout [INFERRED 0.95]

## Communities (101 total, 58 thin omitted)

### Community 0 - "Lib librs"
Cohesion: 0.06
Nodes (38): AssetsConfig, ConnectionConfigFile, DesktopConfig, GlobalConfigFile, main(), parse_hex_rgba(), ThemeColors, ThemeFile (+30 more)

### Community 1 - "Desktop Auto-Update Downloader"
Cohesion: 0.08
Nodes (35): CHUNK_SIZE, DOWNLOAD_TIMEOUT, download_update(), DownloadProgress, installer_path(), MIN_INSTALLER_BYTES, stream_to_file(), UpdateError (+27 more)

### Community 3 - "GitHub CI Workflows"
Cohesion: 0.05
Nodes (47): if, name, needs, runs-on, steps, name, outputs, runs-on (+39 more)

### Community 4 - "ChaCha20-Poly1305 Cipher Suite"
Cohesion: 0.07
Nodes (24): build_nonce(), CipherRole, Android, Desktop, derive_directional_key(), session_cipher_roundtrips_and_blocks_replay(), SessionCipher, fetch_or_create_keypair() (+16 more)

### Community 5 - "Desktop Idle View & Pairing"
Cohesion: 0.10
Nodes (19): SessionSnapshot, IdleViewState, render_about_section(), section_label(), setting_row(), status_pill(), surface_card(), value_text() (+11 more)

### Community 6 - "Update_logic arc"
Cohesion: 0.08
Nodes (22): failed_phase(), HasUpdateController, RetryTarget, Check, Download, Install, OpenStore, UpdateAction (+14 more)

### Community 7 - "Updatemanifest UpdateManifestkt"
Cohesion: 0.08
Nodes (9): CompatibilityCheckerTest, DeviceIdentityStoreTest, UpdateCheckOutcome, Available, Failure, UpToDate, UpdateManifestClient, UpdateRelease (+1 more)

### Community 8 - "Android Crypto & Keys"
Cohesion: 0.08
Nodes (11): CipherRole, ANDROID, DESKTOP, EphemeralX25519, hexToBytesOrNull(), IdentityKeypair, KeypairStore, SessionCipher (+3 more)

### Community 9 - "Updatelogic RetryTarget"
Cohesion: 0.10
Nodes (22): RetryTarget, Check, Download, Install, OpenStore, UpdateAction, CHECK, DOWNLOAD (+14 more)

### Community 10 - "Brand Identity & Vector Assets"
Cohesion: 0.06
Nodes (28): AI Coding Rules for WireWave, Bluetooth Icon, Camera Icon, Checkmark Icon, Keyboard Icon, Microphone Icon, Monitor Icon, Mouse Icon (+20 more)

### Community 11 - "Settings Management & UI"
Cohesion: 0.09
Nodes (12): ConnectionPreferences, PreferencesStore, TransportKind, Bluetooth, WiFi, TransportPriority, BluetoothFirst, WifiFirst (+4 more)

### Community 13 - "Application Lifecycle & Entry Points"
Cohesion: 0.09
Nodes (12): WireWaveApp, acquire_single_instance(), main(), check_compatibility(), CompatibilityReport, CompatibilityStatus, Supported, Unsupported (+4 more)

### Community 14 - "Discoveryscreen bluetooth"
Cohesion: 0.13
Nodes (6): DiscoveryScreen(), EmptyComputersView(), HostRow(), NearbyComputersView(), NumericKeypad(), PairingCodeScreen()

### Community 16 - "Android About & Info Screen"
Cohesion: 0.15
Nodes (11): WireWaveTheme, Colors, Metrics, Typography, AboutRowSpec, AboutSection(), SectionLabel(), SettingRow() (+3 more)

### Community 18 - "Desktop Connected View & UI"
Cohesion: 0.15
Nodes (4): SharedSession, HomeScreen, CanCancelPairing, render_pairing_modal()

### Community 20 - "Device Pairing & OTP Auth"
Cohesion: 0.13
Nodes (15): send_control(), send_encrypted(), Connected, ControlMessage, Challenge, ChallengeReply, Disconnect, HeartbeatPing (+7 more)

### Community 21 - "ChaCha20-Poly1305 Cipher Suite"
Cohesion: 0.16
Nodes (7): derive_session_key(), verify_peer_signature(), compute_otp_proof(), generate_otp(), HmacSha256, otp_proof_matches_only_exact_code_and_transcript(), verify_otp_proof()

### Community 22 - "Device Pairing & OTP Auth"
Cohesion: 0.16
Nodes (8): ActiveSession, PendingPairing, SessionState, Idle, Pairing, ConnectedViewState, ServiceTileSpec, PairingModalState

### Community 23 - "Connectedhomescreen circleshape"
Cohesion: 0.18
Nodes (4): ComingSoonBadge(), ConnectedHomeScreen(), ServiceCircleSpec, UsePhoneRowSpec

### Community 24 - "Android MainActivity & Permissions"
Cohesion: 0.25
Nodes (5): AppContainer, CompatibilityChecker, CompatibilityReport, AboutLogic, AboutSnapshot

### Community 27 - "Lib asyncread"
Cohesion: 0.13
Nodes (8): FRAME_MAGIC, MAX_FRAME_BYTES, read_frame(), TransportError, Codec, FrameTooLarge, InvalidProtocolMagic, Io

### Community 30 - "Session_manager session_managerrs"
Cohesion: 0.16
Nodes (3): decode_x25519(), HandshakeReservation, SessionInner

### Community 32 - "Desktop Auto-Update Downloader"
Cohesion: 0.22
Nodes (4): DownloadOutcome, Failure, Success, UpdateDownloader

### Community 33 - "Updatesection clip"
Cohesion: 0.22
Nodes (3): DetailFooter(), ProgressFooter(), UpdateSection()

### Community 34 - "UI Theme & Design Tokens"
Cohesion: 0.27
Nodes (10): AppConfig, AssetPaths, BuildChannel, Dev, Store, ConnectionConfig, ColorTokens, MetricTokens (+2 more)

### Community 35 - "Session_manager librs"
Cohesion: 0.23
Nodes (10): BEACON_MAGIC, discovery_signing_payload(), encode_probe(), encode_signed_announcement(), HostAnnouncement, is_probe(), probe_has_wirewave_magic_and_exact_payload(), PUBLIC_KEY_LEN (+2 more)

### Community 36 - "Android Crypto & Keys"
Cohesion: 0.21
Nodes (4): FrameCodec, TransportKind, BLUETOOTH, WIFI

### Community 37 - "Mod modrs"
Cohesion: 0.32
Nodes (4): detail_banner(), dispatch_action(), progress_footer(), render_update_section()

### Community 39 - "Desktop AppShell & Navigation"
Cohesion: 0.30
Nodes (6): HasNavigation, nav_button(), NavTab, Home, Settings, render_sidebar()

### Community 41 - "Appconfig AppConfigkt"
Cohesion: 0.20
Nodes (8): CompatibilityStatus, Supported, Unsupported, AppConfig, BuildChannel, DEV, STORE, ConnectionConfig

### Community 42 - "Desktop Connected View & UI"
Cohesion: 0.27
Nodes (3): CanDisconnect, render_connected_view(), service_placeholder_tile()

### Community 43 - "Android Session Controller State"
Cohesion: 0.32
Nodes (5): DiscoveredHost, ClientSessionState, AwaitingOtp, Connecting, Disconnected

### Community 44 - "Device Pairing & OTP Auth"
Cohesion: 0.18
Nodes (11): IncomingDecision, Reject, RequireOtp, TrustedChallenge, RejectCode, AuthFailed, Blocked, Busy (+3 more)

### Community 45 - "Ic_launcher_round WireWave Android App"
Cohesion: 0.18
Nodes (11): WireWave Android App Icon Mascot, Launcher Icon HDPI, Launcher Round Icon HDPI, Launcher Icon MDPI, Launcher Round Icon MDPI, Launcher Icon XHDPI, Launcher Round Icon XHDPI, Launcher Icon XXHDPI (+3 more)

### Community 46 - "Release Automation Workflows"
Cohesion: 0.18
Nodes (11): default, description, options, required, type, channel, on, push (+3 more)

### Community 47 - "Cargo wirewave-core-compat"
Cohesion: 0.49
Nodes (11): wirewave-core-compatibility, wirewave-core-config, wirewave-core-crypto, wirewave-core-discovery, wirewave-core-identity, wirewave-core-session, wirewave-core-transport, wirewave-core-updater (+3 more)

### Community 51 - "Release Automation Workflows"
Cohesion: 0.22
Nodes (8): jobs, publish-release, name, permissions, contents, name, needs, runs-on

### Community 52 - "Release Automation Workflows"
Cohesion: 0.33
Nodes (9): steps, steps, steps, id, name, run, shell, uses (+1 more)

### Community 53 - "Release Automation Workflows"
Cohesion: 0.22
Nodes (9): with, distribution, files, generate_release_notes, java-version, make_latest, name, path (+1 more)

### Community 56 - "Apkinstaller ApkInstaller"
Cohesion: 0.43
Nodes (5): ApkInstaller, InstallOutcome, Failure, Launched, NeedsInstallPermission

### Community 60 - "Release Automation Workflows"
Cohesion: 0.33
Nodes (6): env, name, runs-on, CARGO_TARGET_DIR, CARGO_TERM_COLOR, build-desktop-release

### Community 62 - "Release Automation Workflows"
Cohesion: 0.40
Nodes (5): env, name, runs-on, WIREWAVE_BUILD_CHANNEL, build-android-release

### Community 63 - "Release Automation Workflows"
Cohesion: 0.40
Nodes (5): ANDROID_KEY_ALIAS, ANDROID_KEY_PASSWORD, ANDROID_KEYSTORE_BASE64, ANDROID_KEYSTORE_PASSWORD, env

## Knowledge Gaps
- **123 isolated node(s):** `UpToDate`, `Supported`, `Launched`, `NeedsInstallPermission`, `Android` (+118 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 462 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **58 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SessionController` connect `Android Session Controller State` to `Android Session Controller State`, `Application Lifecycle & Entry Points`, `Android MainActivity & Permissions`, `Android Crypto & Keys`, `Hostscanner DiscoveredHost`, `Data Storage & Persistence`?**
  _High betweenness centrality (0.040) - this node is a cross-community bridge._
- **What connects `UpToDate`, `Supported`, `Launched` to the rest of the system?**
  _123 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Lib librs` be split into smaller, more focused modules?**
  _Cohesion score 0.058699101004759384 - nodes in this community are weakly interconnected._
- **Why does `UpdateController` connect `Updatelogic RetryTarget` to `Android MainActivity & Permissions`, `Compatibilitydialog arrangement`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **Should `Desktop Auto-Update Downloader` be split into smaller, more focused modules?**
  _Cohesion score 0.07755102040816327 - nodes in this community are weakly interconnected._
- **Why does `DiscoveredHost` connect `Android Session Controller State` to `Hostscanner DiscoveredHost`, `Discoveryscreen bluetooth`, `Android Crypto & Keys`, `Android Crypto & Keys`, `Android Session Controller State`, `Hostscanner DiscoveredHost`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **Should `Settings Management & UI` be split into smaller, more focused modules?**
  _Cohesion score 0.06802721088435375 - nodes in this community are weakly interconnected._