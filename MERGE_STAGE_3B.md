# MERGE_STAGE_3B.md — How to Merge Stage 3.0 + 3B Into Your Existing Repo

This add-on is **100% additive**—it does not overwrite any Stage 1 or Stage 2 files.

---

## Step 1: Copy the Folders Into Your Repo Root
Unzip this archive and copy:
- `core/stream/` → `<your-repo>/core/stream/`
- `features/speaker/` → `<your-repo>/features/speaker/`
- `STAGE_3B.md` → `<your-repo>/STAGE_3B.md`

---

## Step 2: Register the 2 New Rust Crates (`Cargo.toml`)
In your root `Cargo.toml`, add to `[workspace].members`:
```toml
"core/stream/rust",
"features/speaker/rust",
```
And add to `[workspace.dependencies]`:
```toml
wirewave-core-stream = { path = "core/stream/rust" }
wirewave-feature-speaker = { path = "features/speaker/rust" }
```

---

## Step 3: Register the 2 New Android Modules (`apps/android/settings.gradle.kts`)
Append to `apps/android/settings.gradle.kts`:
```kotlin
include(":core:stream")
project(":core:stream").projectDir = file("../../core/stream/android")

include(":features:speaker")
project(":features:speaker").projectDir = file("../../features/speaker/android")
```
And in `apps/android/app/build.gradle.kts` (`dependencies`), add:
```kotlin
implementation(project(":core:stream"))
implementation(project(":features:speaker"))
```

---

## Step 4: Android Foreground Service Type (`apps/android/app/src/main/AndroidManifest.xml`)
Add `FOREGROUND_SERVICE_MEDIA_PLAYBACK` permission and update `WireWaveConnectionService`:
```xml
<uses-permission android:name="android.permission.FOREGROUND_SERVICE_MEDIA_PLAYBACK" />
```
```xml
<service
    android:name=".service.WireWaveConnectionService"
    android:exported="false"
    android:foregroundServiceType="connectedDevice|mediaPlayback" />
```

---

## Step 5: Wire Speaker Into Your Stage 2 Session & Connected Home

### Desktop (`apps/desktop` + `features/connection/rust`)
1. Expose the active Stage 2 `session_key` (32 bytes) to derive `udp_key = wirewave_core_stream::derive_udp_key(&session_key, stream_id)`.
2. When Speaker is toggled ON:
   - Bind `let (udp_ch, handle) = UdpDataChannel::bind(udp_key, stream_id).await?;`
   - Send `FeatureControlMessage::FeatureStart { channel: ChannelId::Speaker, udp_port: handle.local_port, stream_id }` over Stage 2's encrypted TCP control channel.
   - Wait for `udp_ch.accept_peer_hello().await` and call `speaker_service.start(udp_ch, rt_handle)`.
3. When Speaker is toggled OFF or session disconnects: call `speaker_service.stop()`.
4. In `features/connection/rust/src/connected/connected_ui.rs`, replace the first `"Speaker"` placeholder tile with:
   ```rust
   wirewave_feature_speaker::render_speaker_tile(&speaker_tile_state, cx)
   ```

### Android (`apps/android/app` + `features/connection/android`)
1. Add `val speakerEngine = SpeakerEngine(appScope)` to `AppContainer`.
2. When Stage 2 receives `feature_start` (`channel = 1`, `udp_port`, `stream_id`):
   - Derive `val udpKey = UdpCipher.deriveUdpKey(sessionKey, streamId)`.
   - Call `speakerEngine.start(hostIp, udpPort, udpKey, streamId)`.
3. On `feature_stop` or disconnect: call `speakerEngine.stop()`.
4. In `ConnectedHomeScreen.kt`, replace the `"Speaker"` placeholder circle with:
   ```kotlin
   SpeakerCircleTile(isOn = speakerState.isOn, onToggle = onToggleSpeaker)
   ```

---

## Step 6: Verify Both Platforms
```powershell
cargo test --workspace --lib
```
```bash
cd apps/android && ./gradlew test :app:assembleDebug
```
