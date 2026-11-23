# Graph Report - wirewave-stage2  (2026-11-23)

## Corpus Check
- 58 files · ~37,832 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1534 nodes · 2987 edges · 120 communities (60 shown, 60 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 61 edges (avg confidence: 0.87)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Android Crypto & Keys
- Android Crypto & Keys
- Desktop Auto-Update Downloader
- GitHub CI Workflows
- Update_logic arc
- Updatemanifest UpdateManifestkt
- Updatelogic RetryTarget
- Brand Identity & Vector Assets
- Windows Bluetooth RFCOMM Listener
- Settings Management & UI
- Connectedhomescreen circleshape
- Settings Management & UI
- Session_manager librs
- Desktop Idle View & Pairing
- Application Lifecycle & Entry Points
- Android About & Info Screen
- ChaCha20-Poly1305 Cipher Suite
- ChaCha20-Poly1305 Cipher Suite
- Android Crypto & Keys
- Compatibilitydialog arrangement
- Desktop Connected View & UI
- Android Crypto & Keys
- Desktop Connected View & UI
- Discoveryscreen bluetooth
- Lib librs
- Session_manager session_managerrs
- Lib librs
- Android Crypto & Keys
- Android Crypto & Keys
- Appconfig AppConfigkt
- ChaCha20-Poly1305 Cipher Suite
- Android Crypto & Keys
- Device Pairing & OTP Auth
- Android MainActivity & Permissions
- Android Foreground Connection Service
- SpeakerService
- laud.rs
- Device Pairing & OTP Auth
- Lib librs
- Settings Management & UI
- Updatemanifest UpdateManifestkt
- Lib asyncread
- Android Crypto & Keys
- Lib librs
- Desktop Idle View & Pairing
- Desktop Idle View & Pairing
- Android Crypto & Keys
- device_finder.rs
- Cargo wirewave-core-compat
- Apkinstaller ApkInstaller
- Connectionsection alignment
- Desktop AppShell & Navigation
- Updatesection clip
- Application Lifecycle & Entry Points
- Data Storage & Persistence
- Application Lifecycle & Entry Points
- Device Pairing & OTP Auth
- Mod modrs
- Ic_launcher_round WireWave Android App
- ChannelId
- Device Pairing & OTP Auth
- Desktop Connected View & UI
- Release Automation Workflows
- Application Lifecycle & Entry Points
- Release Automation Workflows
- Release Automation Workflows
- Release Automation Workflows
- Android MainActivity & Permissions
- Application Lifecycle & Entry Points
- Desktop Idle View & Pairing
- dsp.rs
- Desktop Idle View & Pairing
- Release Automation Workflows
- Appconfig AppConfigkt
- UI Theme & Design Tokens
- Application Lifecycle & Entry Points
- Release Automation Workflows
- Release Automation Workflows
- Home Home Icon
- Phone Phone Icon
- Window-close Window Close Icon
- Window-maximize Window Maximize Icon
- Window-minimize Window Minimize Icon

## God Nodes (most connected - your core abstractions)
1. `SessionController` - 29 edges
2. `SettingsScreen` - 25 edges
3. `SessionManager` - 24 edges
4. `TrustAndPrefsStore` - 19 edges
5. `UpdateError` - 19 edges
6. `TransportPriority` - 17 edges
7. `CryptoError` - 17 edges
8. `TransportKind` - 17 edges
9. `ControlMessage` - 16 edges
10. `UdpDataChannel` - 16 edges

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
- 2-file cycle: `core/stream/rust/src/lib.rs -> core/stream/rust/src/udp_channel.rs -> core/stream/rust/src/lib.rs`
- 2-file cycle: `core/stream/rust/src/lib.rs -> core/stream/rust/src/udp_cipher.rs -> core/stream/rust/src/lib.rs`
- 3-file cycle: `core/stream/rust/src/lib.rs -> core/stream/rust/src/udp_channel.rs -> core/stream/rust/src/udp_cipher.rs -> core/stream/rust/src/lib.rs`

## Hyperedges (group relationships)
- **Stage 2 Active Hardware Services UI Placeholders** — stage_2_active_services_placeholder, assets_brand_icons_speaker_icon, assets_brand_icons_mic_icon, assets_brand_icons_keyboard_icon, assets_brand_icons_mouse_icon, assets_brand_icons_camera_icon [EXTRACTED 1.00]
- **Desktop Window Titlebar Chrome Controls** — assets_brand_icons_window_minimize_icon, assets_brand_icons_window_maximize_icon, assets_brand_icons_window_close_icon [EXTRACTED 1.00]
- **Android Launcher Icon Density Suite** — apps_android_app_src_main_res_drawable_app_icon_app_icon, apps_android_app_src_main_res_mipmap_hdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_hdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_mdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_mdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_xhdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_xhdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_xxhdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_xxhdpi_ic_launcher_round_ic_launcher_round, apps_android_app_src_main_res_mipmap_xxxhdpi_ic_launcher_ic_launcher, apps_android_app_src_main_res_mipmap_xxxhdpi_ic_launcher_round_ic_launcher_round [INFERRED 0.95]
- **Single Source of Truth Monorepo Governance** — stage_1_single_source_of_truth, agent_single_sources_of_truth, developer_monorepo_layout [INFERRED 0.95]

## Communities (120 total, 60 thin omitted)

### Community 0 - "Android Crypto & Keys"
Cohesion: 0.07
Nodes (11): ClientSessionState, AwaitingOtp, Connected, Connecting, Disconnected, SessionController, ConnectionPrefs, TrustAndPrefsStore (+3 more)

### Community 1 - "Android Crypto & Keys"
Cohesion: 0.07
Nodes (10): AudioTrackPlayer, JitterBuffer, LatencyMode, BALANCED, LOW, STABLE, LaudPacket, SpeakerEngine (+2 more)

### Community 2 - "Desktop Auto-Update Downloader"
Cohesion: 0.08
Nodes (36): CHUNK_SIZE, DOWNLOAD_TIMEOUT, download_update(), DownloadProgress, installer_dir(), installer_path(), MIN_INSTALLER_BYTES, stream_to_file() (+28 more)

### Community 3 - "GitHub CI Workflows"
Cohesion: 0.05
Nodes (47): if, name, needs, runs-on, steps, name, outputs, runs-on (+39 more)

### Community 4 - "Update_logic arc"
Cohesion: 0.08
Nodes (22): failed_phase(), HasUpdateController, RetryTarget, Check, Download, Install, OpenStore, UpdateAction (+14 more)

### Community 5 - "Updatemanifest UpdateManifestkt"
Cohesion: 0.09
Nodes (11): DownloadOutcome, Failure, Success, UpdateDownloader, UpdateCheckOutcome, Available, Failure, UpToDate (+3 more)

### Community 6 - "Updatelogic RetryTarget"
Cohesion: 0.10
Nodes (22): RetryTarget, Check, Download, Install, OpenStore, UpdateAction, CHECK, DOWNLOAD (+14 more)

### Community 7 - "Brand Identity & Vector Assets"
Cohesion: 0.06
Nodes (28): AI Coding Rules for WireWave, Bluetooth Icon, Camera Icon, Checkmark Icon, Keyboard Icon, Microphone Icon, Monitor Icon, Mouse Icon (+20 more)

### Community 10 - "Connectedhomescreen circleshape"
Cohesion: 0.13
Nodes (5): ComingSoonBadge(), ConnectedHomeScreen(), ServiceCircleSpec, UsePhoneRowSpec, SpeakerCircleTile()

### Community 11 - "Settings Management & UI"
Cohesion: 0.11
Nodes (9): ConnectionPreferences, PreferencesStore, TransportPriority, BluetoothFirst, WifiFirst, ALL_PRIORITIES, ConnectionSettingsState, HasConnectionSettingsActions (+1 more)

### Community 12 - "Session_manager librs"
Cohesion: 0.12
Nodes (13): StreamError, DatagramTooLarge, DecryptFailed, EncryptFailed, InvalidHeader, Io, ReplayedSequence, StreamMismatch (+5 more)

### Community 13 - "Desktop Idle View & Pairing"
Cohesion: 0.14
Nodes (11): DeviceRowModel, DeviceTab, .ALL, Blocked, Connected, Trusted, rows_for_tab(), action_chip() (+3 more)

### Community 14 - "Application Lifecycle & Entry Points"
Cohesion: 0.09
Nodes (7): WireWaveApp, acquire_single_instance(), main(), SingleInstance, Acquired, AlreadyRunning, Unavailable

### Community 15 - "Android About & Info Screen"
Cohesion: 0.14
Nodes (11): WireWaveTheme, Colors, Metrics, Typography, AboutRowSpec, AboutSection(), SectionLabel(), SettingRow() (+3 more)

### Community 16 - "ChaCha20-Poly1305 Cipher Suite"
Cohesion: 0.15
Nodes (14): CryptProtectData(), CryptUnprotectData(), DataBlob, fetch_or_create_keypair(), fetch_or_create_keypair_at(), IdentityKeypair, KEYPAIR_FILE_NAME, keypair_persists_and_verifies_signatures() (+6 more)

### Community 17 - "ChaCha20-Poly1305 Cipher Suite"
Cohesion: 0.15
Nodes (6): build_udp_nonce(), derive_udp_key(), ReplayWindow, udp_cipher_handles_reordering_and_rejects_replay(), UDP_HEADER_BYTES, UdpCipher

### Community 18 - "Android Crypto & Keys"
Cohesion: 0.14
Nodes (9): ChannelId, CAMERA, KEYBOARD, MIC, MOUSE, SPEAKER, DecryptedDatagram, ReplayWindow (+1 more)

### Community 20 - "Desktop Connected View & UI"
Cohesion: 0.19
Nodes (7): ConnectedViewState, ServiceTileSpec, render_connected_view(), service_placeholder_tile(), SpeakerTileState, CanToggleSpeaker, render_speaker_tile()

### Community 21 - "Android Crypto & Keys"
Cohesion: 0.11
Nodes (8): CipherRole, ANDROID, DESKTOP, EphemeralX25519, hexToBytesOrNull(), IdentityKeypair, KeypairStore, toHex()

### Community 23 - "Discoveryscreen bluetooth"
Cohesion: 0.20
Nodes (6): DiscoveryScreen(), EmptyComputersView(), HostRow(), NearbyComputersView(), NumericKeypad(), PairingCodeScreen()

### Community 24 - "Lib librs"
Cohesion: 0.14
Nodes (9): BEACON_MAGIC, encode_probe(), HostAnnouncement, is_probe(), probe_has_wirewave_magic_and_exact_payload(), PUBLIC_KEY_LEN, SIGNATURE_LEN, PREFS_FILE_NAME (+1 more)

### Community 26 - "Lib librs"
Cohesion: 0.20
Nodes (19): corrupted_identity_is_backed_up_and_replaced(), default_identity_path(), derive_salted_id(), DEVICE_ID_SALT, DeviceIdentity, fetch_or_create_at(), fetch_or_create_device_id(), fetch_or_create_is_idempotent() (+11 more)

### Community 29 - "Android Crypto & Keys"
Cohesion: 0.20
Nodes (3): SessionCipher, WireWaveCrypto, WireWaveCryptoTest

### Community 30 - "Appconfig AppConfigkt"
Cohesion: 0.19
Nodes (8): CompatibilityChecker, CompatibilityReport, CompatibilityStatus, Supported, Unsupported, CompatibilityCheckerTest, AboutLogic, AboutSnapshot

### Community 31 - "ChaCha20-Poly1305 Cipher Suite"
Cohesion: 0.12
Nodes (16): build_nonce(), CipherRole, Android, Desktop, derive_directional_key(), session_cipher_roundtrips_and_blocks_replay(), SessionCipher, CryptoError (+8 more)

### Community 32 - "Android Crypto & Keys"
Cohesion: 0.15
Nodes (4): discovery_signing_payload(), encode_signed_announcement(), run_udp_discovery_responder(), SessionManager

### Community 33 - "Device Pairing & OTP Auth"
Cohesion: 0.14
Nodes (10): ActiveSession, PendingPairing, SessionState, Idle, Pairing, SharedSession, TransportKind, Bluetooth (+2 more)

### Community 36 - "SpeakerService"
Cohesion: 0.18
Nodes (6): run_capture_supervisor(), run_endpoint_loop(), SpeakerService, SpeakerStatus, Off, Streaming

### Community 37 - "laud.rs"
Cohesion: 0.18
Nodes (12): FLAG_DISCONTINUITY, FLAG_SILENCE, FRAMES_PER_PACKET, LAUD_HEADER_BYTES, LAUD_MAGIC, laud_packet_roundtrips_and_fits_wifi_mtu(), LAUD_VERSION, LaudHeader (+4 more)

### Community 38 - "Device Pairing & OTP Auth"
Cohesion: 0.19
Nodes (9): compute_exchange_hash(), derive_session_key(), exchange_hash_binds_all_parameters(), verify_peer_signature(), decode_x25519(), send_control(), send_encrypted(), Connected (+1 more)

### Community 39 - "Lib librs"
Cohesion: 0.21
Nodes (5): read_trust_file(), trust_store_persists_trust_block_and_forget_by_device_id(), TrustedDevice, TrustFile, TrustStore

### Community 42 - "Lib asyncread"
Cohesion: 0.14
Nodes (8): FRAME_MAGIC, MAX_FRAME_BYTES, read_frame(), TransportError, Codec, FrameTooLarge, InvalidProtocolMagic, Io

### Community 43 - "Android Crypto & Keys"
Cohesion: 0.27
Nodes (3): DiscoveredHost, HostScanner, SignedAnnouncement

### Community 44 - "Lib librs"
Cohesion: 0.23
Nodes (12): AssetsConfig, ConnectionConfigFile, DesktopConfig, GlobalConfigFile, main(), parse_hex_rgba(), ThemeColors, ThemeFile (+4 more)

### Community 45 - "Desktop Idle View & Pairing"
Cohesion: 0.37
Nodes (6): render_about_section(), section_label(), setting_row(), status_pill(), surface_card(), value_text()

### Community 46 - "Desktop Idle View & Pairing"
Cohesion: 0.24
Nodes (3): listener_pill(), render_idle_view(), render_incompatibility_popup()

### Community 47 - "Android Crypto & Keys"
Cohesion: 0.21
Nodes (4): FrameCodec, TransportKind, BLUETOOTH, WIFI

### Community 48 - "device_finder.rs"
Cohesion: 0.29
Nodes (10): AudioEndpointInfo, detect_active_playback_endpoint(), EndpointKind, SystemDefault, VirtualSpeaker, ep(), is_virtual_speaker_name(), prefers_virtual_speaker_when_present_else_default() (+2 more)

### Community 49 - "Cargo wirewave-core-compat"
Cohesion: 0.41
Nodes (13): wirewave-core-compatibility, wirewave-core-config, wirewave-core-crypto, wirewave-core-discovery, wirewave-core-identity, wirewave-core-session, wirewave-core-stream, wirewave-core-transport (+5 more)

### Community 50 - "Apkinstaller ApkInstaller"
Cohesion: 0.23
Nodes (5): ApkInstaller, InstallOutcome, Failure, Launched, NeedsInstallPermission

### Community 52 - "Desktop AppShell & Navigation"
Cohesion: 0.30
Nodes (6): HasNavigation, nav_button(), NavTab, Home, Settings, render_sidebar()

### Community 53 - "Updatesection clip"
Cohesion: 0.27
Nodes (4): ConnectionConfig, DetailFooter(), ProgressFooter(), UpdateSection()

### Community 54 - "Application Lifecycle & Entry Points"
Cohesion: 0.21
Nodes (6): AppConfig, AssetPaths, BuildChannel, Dev, Store, ConnectionConfig

### Community 56 - "Application Lifecycle & Entry Points"
Cohesion: 0.27
Nodes (9): check_compatibility(), CompatibilityReport, CompatibilityStatus, Supported, Unsupported, evaluate_windows_build(), query_windows_version(), windows_10_build_is_unsupported() (+1 more)

### Community 57 - "Device Pairing & OTP Auth"
Cohesion: 0.18
Nodes (11): IncomingDecision, Reject, RequireOtp, TrustedChallenge, RejectCode, AuthFailed, Blocked, Busy (+3 more)

### Community 58 - "Mod modrs"
Cohesion: 0.36
Nodes (4): detail_banner(), dispatch_action(), progress_footer(), render_update_section()

### Community 59 - "Ic_launcher_round WireWave Android App"
Cohesion: 0.18
Nodes (11): WireWave Android App Icon Mascot, Launcher Icon HDPI, Launcher Round Icon HDPI, Launcher Icon MDPI, Launcher Round Icon MDPI, Launcher Icon XHDPI, Launcher Round Icon XHDPI, Launcher Icon XXHDPI (+3 more)

### Community 60 - "ChannelId"
Cohesion: 0.18
Nodes (10): ChannelId, Camera, Keyboard, Mic, Mouse, Speaker, FeatureControlMessage, FeatureStart (+2 more)

### Community 61 - "Device Pairing & OTP Auth"
Cohesion: 0.18
Nodes (11): ControlMessage, Challenge, ChallengeReply, Disconnect, HeartbeatPing, HeartbeatPong, Hello, OtpProof (+3 more)

### Community 63 - "Release Automation Workflows"
Cohesion: 0.18
Nodes (11): default, description, options, required, type, channel, on, push (+3 more)

### Community 64 - "Application Lifecycle & Entry Points"
Cohesion: 0.27
Nodes (5): compute_otp_proof(), generate_otp(), HmacSha256, otp_proof_matches_only_exact_code_and_transcript(), verify_otp_proof()

### Community 65 - "Release Automation Workflows"
Cohesion: 0.22
Nodes (8): jobs, publish-release, name, permissions, contents, name, needs, runs-on

### Community 66 - "Release Automation Workflows"
Cohesion: 0.33
Nodes (9): steps, steps, steps, id, name, run, shell, uses (+1 more)

### Community 67 - "Release Automation Workflows"
Cohesion: 0.22
Nodes (9): with, distribution, files, generate_release_notes, java-version, make_latest, name, path (+1 more)

### Community 71 - "dsp.rs"
Cohesion: 0.36
Nodes (5): downmix_to_stereo(), f32_to_s16le_bytes(), mono_downmix_duplicates_to_stereo(), resampler_passthrough_at_48k(), StereoResampler

### Community 74 - "Release Automation Workflows"
Cohesion: 0.33
Nodes (6): env, name, runs-on, CARGO_TARGET_DIR, CARGO_TERM_COLOR, build-desktop-release

### Community 75 - "Appconfig AppConfigkt"
Cohesion: 0.50
Nodes (4): AppConfig, BuildChannel, DEV, STORE

### Community 76 - "UI Theme & Design Tokens"
Cohesion: 0.70
Nodes (4): ColorTokens, MetricTokens, ThemeTokens, TypographyTokens

### Community 78 - "Release Automation Workflows"
Cohesion: 0.40
Nodes (5): env, name, runs-on, WIREWAVE_BUILD_CHANNEL, build-android-release

### Community 79 - "Release Automation Workflows"
Cohesion: 0.40
Nodes (5): ANDROID_KEY_ALIAS, ANDROID_KEY_PASSWORD, ANDROID_KEYSTORE_BASE64, ANDROID_KEYSTORE_PASSWORD, env

## Knowledge Gaps
- **152 isolated node(s):** `Check`, `OpenStore`, `Checking`, `Idle`, `UpToDate` (+147 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 508 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **60 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SpeakerEngine` connect `Android Crypto & Keys` to `Android MainActivity & Permissions`, `Android Crypto & Keys`?**
  _High betweenness centrality (0.035) - this node is a cross-community bridge._
- **What connects `Check`, `OpenStore`, `Checking` to the rest of the system?**
  _152 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Android Crypto & Keys` be split into smaller, more focused modules?**
  _Cohesion score 0.06572769953051644 - nodes in this community are weakly interconnected._
- **Why does `SessionController` connect `Android Crypto & Keys` to `Android MainActivity & Permissions`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **Should `Android Crypto & Keys` be split into smaller, more focused modules?**
  _Cohesion score 0.0673758865248227 - nodes in this community are weakly interconnected._
- **Why does `UdpStreamClient` connect `Android Crypto & Keys` to `Android Crypto & Keys`, `Android Crypto & Keys`, `Android Crypto & Keys`?**
  _High betweenness centrality (0.020) - this node is a cross-community bridge._
- **Should `Desktop Auto-Update Downloader` be split into smaller, more focused modules?**
  _Cohesion score 0.08333333333333333 - nodes in this community are weakly interconnected._