# WireWave — Stage 3.0 (Encrypted UDP Data Plane) + Stage 3B (Speaker)

## 1. What This Stage Adds
- **Stage 3.0 (`core/stream/{rust,android}`)**: Moves real-time feature data off TCP onto an **encrypted UDP data plane** while keeping auth, pairing, and feature start/stop on Stage 2's encrypted TCP control channel.
- **Stage 3B (`features/speaker/{rust,android}`)**: Streams Windows PC audio to the Android phone with low latency, auto-selecting a **virtual speaker** when installed or falling back to the **current Windows default playback device**, with automatic hot-swap if the user switches output devices mid-stream.

---

## 2. Protocol & Architecture

### Control Plane vs. Data Plane
- **Control Plane (Stage 2 Encrypted TCP)**: Exchanges `FeatureStart(Speaker, udp_port, stream_id)`, `FeatureStop(Speaker)`, and `FeatureStateSync`.
- **Data Plane (Encrypted UDP — `WW2U`)**:
  - Datagram header (17 bytes, authenticated as AEAD AAD): `WW2U (4B)` | `channel_id (1B)` | `stream_id (4B BE)` | `seq (8B BE)`.
  - Cipher: **ChaCha20-Poly1305** using `derive_udp_key(session_key, stream_id)` (zeroized on close).
  - Anti-replay: **64-packet sliding window** tolerating minor UDP reordering while blocking replayed packets.
  - Firewall punch: Android sends signed `WW2U_HELLO` packets so Desktop locks onto the phone's UDP endpoint.

### Speaker Audio Pipeline (MTU-Safe)
```text
Windows WASAPI Loopback (Virtual Speaker if present, else Default)
  → Stereo Downmix → 48 kHz Resample → S16LE
  → 256-frame LAUD v1 packet (1,024 B PCM + 20 B header)
  → Encrypted WW2U UDP datagram (~1,077 B < 1,400 B Wi-Fi MTU)
  → Android UdpStreamClient → LAUD parser → Sequence JitterBuffer → Low-Latency AudioTrack
```

---

## 3. What to Expect & How to Debug

| Check | Expected Result |
| :--- | :--- |
| **Virtual speaker present** | Desktop logs `kind = "Virtual Speaker"` and streams audio from that virtual playback endpoint. |
| **No virtual speaker** | Desktop falls back cleanly to the current Windows default playback device (`kind = "Default Speaker"`). |
| **Switch output mid-stream** | Changing Windows default playback device causes `capture_loop` to detect the change within 2s and seamlessly re-attach WASAPI loopback. |
| **Toggle Speaker** | Clicking Speaker on Desktop or Android toggles it `On`/`Off` (blue highlight when `On`); other 4 services remain `Coming soon`. |
| **UDP packet loss** | Minor Wi-Fi drops produce a brief 5ms silence fade instead of a 200ms TCP stall. |

### Unit Tests & Debug Commands
```powershell
# Desktop tests (UDP cipher/replay, LAUD MTU, DSP downmix/resample, virtual/default selector)
cargo test -p wirewave-core-stream -p wirewave-feature-speaker --lib
```

```bash
# Android JVM tests (UDP cipher/replay, LAUD parser, out-of-order JitterBuffer)
cd apps/android
./gradlew :core:stream:test :features:speaker:test
```
