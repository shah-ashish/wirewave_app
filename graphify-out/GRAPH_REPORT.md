# Graph Report - wirewave-stage2  (2026-11-23)

## Corpus Check
- 28 files · ~31,405 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1323 nodes · 2540 edges · 99 communities (46 shown, 53 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 53 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Android Crypto & Keys
- Updatelogic RetryTarget
- Lib librs
- Application Lifecycle & Entry Points
- Settings Management & UI
- Desktop Auto-Update Downloader
- GitHub CI Workflows
- Update_logic arc
- Android Crypto & Keys
- Brand Identity & Vector Assets
- Desktop Idle View & Pairing
- Windows Bluetooth RFCOMM Listener
- Android MainActivity & Permissions
- Discoveryscreen bluetooth
- Settings Management & UI
- Android About & Info Screen
- Desktop Connected View & UI
- Session_manager session_managerrs
- Compatibilitydialog arrangement
- Android Foreground Connection Service
- Connectedhomescreen circleshape
- Device Pairing & OTP Auth
- Device Pairing & OTP Auth
- Desktop Auto-Update Downloader
- Session_manager librs
- Updatemanifest UpdateManifestkt
- ChaCha20-Poly1305 Cipher Suite
- Android MainActivity & Permissions
- Updatesection clip
- Device Pairing & OTP Auth
- Apkinstaller ApkInstaller
- ChaCha20-Poly1305 Cipher Suite
- Application Lifecycle & Entry Points
- Lib asyncread
- Desktop Idle View & Pairing
- ChaCha20-Poly1305 Cipher Suite
- Desktop Idle View & Pairing
- Mod modrs
- Connectionsection alignment
- Desktop AppShell & Navigation
- Updatemanifest UpdateManifestkt
- Appconfig AppConfigkt
- Data Storage & Persistence
- Settings Management & UI
- Android Crypto & Keys
- Ic_launcher_round WireWave Android App
- Desktop Idle View & Pairing
- Desktop Connected View & UI
- ChaCha20-Poly1305 Cipher Suite
- Release Automation Workflows
- Cargo wirewave-core-compat
- Updatemanifest UpdateManifestkt
- Release Automation Workflows
- Release Automation Workflows
- Release Automation Workflows
- Appconfig AppConfigkt
- Release Automation Workflows
- UI Theme & Design Tokens
- Release Automation Workflows
- Release Automation Workflows
- Application Lifecycle & Entry Points
- Home Home Icon
- Phone Phone Icon
- Window-close Window Close Icon
- Window-maximize Window Maximize Icon
- Window-minimize Window Minimize Icon

## God Nodes (most connected - your core abstractions)
1. `SessionController` - 29 edges
2. `SettingsScreen` - 26 edges
3. `SessionManager` - 25 edges
4. `AppContainer` - 21 edges
5. `UpdateError` - 19 edges
6. `CryptoError` - 17 edges
7. `TransportKind` - 17 edges
8. `TransportPriority` - 17 edges
9. `UpdateController` - 16 edges
10. `ControlMessage` - 16 edges

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
- 2-file cycle: `core/crypto/rust/src/keys.rs -> core/crypto/rust/src/lib.rs -> core/crypto/rust/src/keys.rs`
- 2-file cycle: `core/crypto/rust/src/cipher.rs -> core/crypto/rust/src/lib.rs -> core/crypto/rust/src/cipher.rs`

## Hyperedges (group relationships)
- **Stage 2 Active Hardware Services UI Placeholders** — stage_2_active_services_placeholder, assets_brand_icons_speaker_icon, assets_brand_icons_mic_icon, assets_brand_icons_keyboard_icon, assets_brand_icons_mouse_icon, assets_brand_icons_camera_icon [EXTRACTED 1.00]
- **Desktop Window Titlebar Chrome Controls** — assets_brand_icons_window_minimize_icon, assets_brand_icons_window_maximize_icon, assets_brand_icons_window_close_icon [EXTRACTED 1.00]
- **Android Launcher Icon Density Suite** — apps_android_app_src_main_res_drawable_app_icon_app_icon, apps_android_app_src_main_res_mipmap_hdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_hdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_mdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_mdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_xhdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_xhdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_xxhdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_xxhdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_xxxhdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_xxxhdpi_ic_launcher_round_ic_launcher_round [INFERRED 0.95]
- **Single Source of Truth Monorepo Governance** — stage_1_single_source_of_truth, agent_single_sources_of_truth, developer_monorepo_layout [INFERRED 0.95]

## Communities (99 total, 53 thin omitted)

### Community 0 - "Android Crypto & Keys"
Cohesion: 0.05
Nodes (16): CipherRole, ANDROID, DESKTOP, EphemeralX25519, hexToBytesOrNull(), IdentityKeypair, SessionCipher, toHex() (+8 more)

### Community 1 - "Updatelogic RetryTarget"
Cohesion: 0.05
Nodes (27): ConnectionPrefs, TrustAndPrefsStore, TrustedDesktop, DiscoveryController, DiscoveryUiState, RetryTarget, Check, Download (+19 more)

### Community 2 - "Lib librs"
Cohesion: 0.06
Nodes (38): AssetsConfig, ConnectionConfigFile, DesktopConfig, GlobalConfigFile, main(), parse_hex_rgba(), ThemeColors, ThemeFile (+30 more)

### Community 3 - "Application Lifecycle & Entry Points"
Cohesion: 0.05
Nodes (25): EmbeddedAssets, check_compatibility(), CompatibilityReport, CompatibilityStatus, Supported, Unsupported, evaluate_windows_build(), query_windows_version() (+17 more)

### Community 5 - "Desktop Auto-Update Downloader"
Cohesion: 0.08
Nodes (35): CHUNK_SIZE, DOWNLOAD_TIMEOUT, download_update(), DownloadProgress, installer_path(), MIN_INSTALLER_BYTES, stream_to_file(), UpdateError (+27 more)

### Community 6 - "GitHub CI Workflows"
Cohesion: 0.05
Nodes (47): if, name, needs, runs-on, steps, name, outputs, runs-on (+39 more)

### Community 7 - "Update_logic arc"
Cohesion: 0.08
Nodes (22): failed_phase(), HasUpdateController, RetryTarget, Check, Download, Install, OpenStore, UpdateAction (+14 more)

### Community 8 - "Android Crypto & Keys"
Cohesion: 0.07
Nodes (7): DiscoveredHost, HostScanner, SignedAnnouncement, FrameCodec, TransportKind, BLUETOOTH, WIFI

### Community 9 - "Brand Identity & Vector Assets"
Cohesion: 0.06
Nodes (28): AI Coding Rules for WireWave, Bluetooth Icon, Camera Icon, Checkmark Icon, Keyboard Icon, Microphone Icon, Monitor Icon, Mouse Icon (+20 more)

### Community 10 - "Desktop Idle View & Pairing"
Cohesion: 0.11
Nodes (13): SessionSnapshot, IdleViewState, DeviceRowModel, DeviceTab, .ALL, Blocked, Connected, Trusted (+5 more)

### Community 13 - "Discoveryscreen bluetooth"
Cohesion: 0.14
Nodes (6): DiscoveryScreen(), EmptyComputersView(), HostRow(), NearbyComputersView(), NumericKeypad(), PairingCodeScreen()

### Community 14 - "Settings Management & UI"
Cohesion: 0.12
Nodes (9): ConnectionPreferences, PreferencesStore, TransportPriority, BluetoothFirst, WifiFirst, ALL_PRIORITIES, ConnectionSettingsState, HasConnectionSettingsActions (+1 more)

### Community 15 - "Android About & Info Screen"
Cohesion: 0.13
Nodes (11): WireWaveTheme, Colors, Metrics, Typography, AboutRowSpec, AboutSection(), SectionLabel(), SettingRow() (+3 more)

### Community 16 - "Desktop Connected View & UI"
Cohesion: 0.14
Nodes (5): SharedSession, CanDisconnect, HomeScreen, CanCancelPairing, render_pairing_modal()

### Community 20 - "Connectedhomescreen circleshape"
Cohesion: 0.16
Nodes (4): ComingSoonBadge(), ConnectedHomeScreen(), ServiceCircleSpec, UsePhoneRowSpec

### Community 21 - "Device Pairing & OTP Auth"
Cohesion: 0.20
Nodes (9): compute_exchange_hash(), derive_session_key(), exchange_hash_binds_all_parameters(), session_cipher_roundtrips_and_blocks_replay(), decode_x25519(), send_control(), send_encrypted(), read_frame() (+1 more)

### Community 22 - "Device Pairing & OTP Auth"
Cohesion: 0.11
Nodes (18): ControlMessage, Challenge, ChallengeReply, Disconnect, HeartbeatPing, HeartbeatPong, Hello, OtpProof (+10 more)

### Community 23 - "Desktop Auto-Update Downloader"
Cohesion: 0.18
Nodes (4): DownloadOutcome, Failure, Success, UpdateDownloader

### Community 24 - "Session_manager librs"
Cohesion: 0.18
Nodes (10): BEACON_MAGIC, discovery_signing_payload(), encode_probe(), encode_signed_announcement(), HostAnnouncement, is_probe(), probe_has_wirewave_magic_and_exact_payload(), PUBLIC_KEY_LEN (+2 more)

### Community 25 - "Updatemanifest UpdateManifestkt"
Cohesion: 0.21
Nodes (6): UpdateCheckOutcome, Available, Failure, UpToDate, UpdateManifestClient, UpdateRelease

### Community 26 - "ChaCha20-Poly1305 Cipher Suite"
Cohesion: 0.21
Nodes (8): CryptProtectData(), CryptUnprotectData(), DataBlob, KEYPAIR_FILE_NAME, LocalFree(), protect(), StoredKeypair, unprotect()

### Community 27 - "Android MainActivity & Permissions"
Cohesion: 0.31
Nodes (5): AppContainer, CompatibilityReport, KeypairStore, AboutLogic, AboutSnapshot

### Community 28 - "Updatesection clip"
Cohesion: 0.22
Nodes (3): DetailFooter(), ProgressFooter(), UpdateSection()

### Community 29 - "Device Pairing & OTP Auth"
Cohesion: 0.19
Nodes (7): ActiveSession, PendingPairing, SessionState, Connected, Idle, Pairing, PairingModalState

### Community 30 - "Apkinstaller ApkInstaller"
Cohesion: 0.21
Nodes (5): ApkInstaller, InstallOutcome, Failure, Launched, NeedsInstallPermission

### Community 31 - "ChaCha20-Poly1305 Cipher Suite"
Cohesion: 0.17
Nodes (4): CipherRole, Android, Desktop, derive_directional_key()

### Community 33 - "Lib asyncread"
Cohesion: 0.15
Nodes (7): FRAME_MAGIC, MAX_FRAME_BYTES, TransportError, Codec, FrameTooLarge, InvalidProtocolMagic, Io

### Community 34 - "Desktop Idle View & Pairing"
Cohesion: 0.24
Nodes (3): listener_pill(), render_idle_view(), render_incompatibility_popup()

### Community 35 - "ChaCha20-Poly1305 Cipher Suite"
Cohesion: 0.18
Nodes (11): build_nonce(), SessionCipher, CryptoError, DecryptFailed, EncryptFailed, InvalidPublicKey, InvalidSignature, Io (+3 more)

### Community 36 - "Desktop Idle View & Pairing"
Cohesion: 0.41
Nodes (6): render_about_section(), section_label(), setting_row(), status_pill(), surface_card(), value_text()

### Community 37 - "Mod modrs"
Cohesion: 0.32
Nodes (4): detail_banner(), dispatch_action(), progress_footer(), render_update_section()

### Community 39 - "Desktop AppShell & Navigation"
Cohesion: 0.30
Nodes (6): HasNavigation, nav_button(), NavTab, Home, Settings, render_sidebar()

### Community 41 - "Appconfig AppConfigkt"
Cohesion: 0.23
Nodes (5): CompatibilityChecker, CompatibilityStatus, Supported, Unsupported, CompatibilityCheckerTest

### Community 43 - "Settings Management & UI"
Cohesion: 0.20
Nodes (7): IncomingDecision, Reject, RequireOtp, TrustedChallenge, TransportKind, Bluetooth, WiFi

### Community 45 - "Ic_launcher_round WireWave Android App"
Cohesion: 0.18
Nodes (11): WireWave Android App Icon Mascot, Launcher Icon HDPI, Launcher Round Icon HDPI, Launcher Icon MDPI, Launcher Round Icon MDPI, Launcher Icon XHDPI, Launcher Round Icon XHDPI, Launcher Icon XXHDPI (+3 more)

### Community 48 - "ChaCha20-Poly1305 Cipher Suite"
Cohesion: 0.29
Nodes (7): fetch_or_create_keypair(), fetch_or_create_keypair_at(), IdentityKeypair, keypair_persists_and_verifies_signatures(), load_keypair(), save_keypair(), verify_peer_signature()

### Community 49 - "Release Automation Workflows"
Cohesion: 0.18
Nodes (11): default, description, options, required, type, channel, on, push (+3 more)

### Community 50 - "Cargo wirewave-core-compat"
Cohesion: 0.49
Nodes (11): wirewave-core-compatibility, wirewave-core-config, wirewave-core-crypto, wirewave-core-discovery, wirewave-core-identity, wirewave-core-session, wirewave-core-transport, wirewave-core-updater (+3 more)

### Community 52 - "Release Automation Workflows"
Cohesion: 0.22
Nodes (8): jobs, publish-release, name, permissions, contents, name, needs, runs-on

### Community 53 - "Release Automation Workflows"
Cohesion: 0.33
Nodes (9): steps, steps, steps, id, name, run, shell, uses (+1 more)

### Community 54 - "Release Automation Workflows"
Cohesion: 0.22
Nodes (9): with, distribution, files, generate_release_notes, java-version, make_latest, name, path (+1 more)

### Community 55 - "Appconfig AppConfigkt"
Cohesion: 0.38
Nodes (5): AppConfig, BuildChannel, DEV, STORE, ConnectionConfig

### Community 57 - "Release Automation Workflows"
Cohesion: 0.33
Nodes (6): env, name, runs-on, CARGO_TARGET_DIR, CARGO_TERM_COLOR, build-desktop-release

### Community 58 - "UI Theme & Design Tokens"
Cohesion: 0.70
Nodes (4): ColorTokens, MetricTokens, ThemeTokens, TypographyTokens

### Community 59 - "Release Automation Workflows"
Cohesion: 0.40
Nodes (5): env, name, runs-on, WIREWAVE_BUILD_CHANNEL, build-android-release

### Community 60 - "Release Automation Workflows"
Cohesion: 0.40
Nodes (5): ANDROID_KEY_ALIAS, ANDROID_KEY_PASSWORD, ANDROID_KEYSTORE_BASE64, ANDROID_KEYSTORE_PASSWORD, env

## Knowledge Gaps
- **123 isolated node(s):** `Supported`, `Launched`, `NeedsInstallPermission`, `UpToDate`, `Check` (+118 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 468 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **53 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AppContainer` connect `Android MainActivity & Permissions` to `Android Crypto & Keys`, `Updatelogic RetryTarget`, `Android Crypto & Keys`, `Appconfig AppConfigkt`, `Data Storage & Persistence`, `Android Foreground Connection Service`, `Desktop Auto-Update Downloader`, `Updatemanifest UpdateManifestkt`, `Application Lifecycle & Entry Points`, `Apkinstaller ApkInstaller`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **What connects `Supported`, `Launched`, `NeedsInstallPermission` to the rest of the system?**
  _123 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Android Crypto & Keys` be split into smaller, more focused modules?**
  _Cohesion score 0.05465765501028504 - nodes in this community are weakly interconnected._
- **Why does `SessionController` connect `Android Crypto & Keys` to `Updatelogic RetryTarget`, `Android MainActivity & Permissions`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **Should `Updatelogic RetryTarget` be split into smaller, more focused modules?**
  _Cohesion score 0.05454545454545454 - nodes in this community are weakly interconnected._
- **Why does `UpdateManifestClient` connect `Updatemanifest UpdateManifestkt` to `Updatemanifest UpdateManifestkt`, `Android MainActivity & Permissions`?**
  _High betweenness centrality (0.029) - this node is a cross-community bridge._
- **Should `Lib librs` be split into smaller, more focused modules?**
  _Cohesion score 0.05888376856118792 - nodes in this community are weakly interconnected._