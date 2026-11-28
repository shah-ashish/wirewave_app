# Changelog

## [2.0.0] - 2026-11-28

### Changed

- Redesigned Desktop Home (Idle, Pairing, and Connected views) with responsive layout, balanced 3-column active service tiles, and high-contrast Disconnect button.
- Unified Desktop Settings into a single scrollable screen, merged Virtual Microphone Driver inline, redesigned Devices without inner tabs or raw IDs, and moved software update controls to a bottom sidebar icon and overlay.
- Redesigned Android Discovery, Connected Home, Settings, and Paired Computers screens with surface cards, aligned audio/input controls, 44dp+ touch targets, and `WireWaveTheme` token consistency.

### Added

- Desktop sidebar update icon and floating update overlay.
- Version row in Desktop and Android About sections.

## [1.2.6] - 2026-11-27

### Changed

- Reused shared UI components across desktop feature tiles and Android controls and navigation.
- Separated session models, transport handling, feature state, and audio host adapters into focused modules.
- Simplified Android app composition and desktop feature registration.
- Separated mouse packet encoding and decoding from network delivery and input injection.
- Consolidated remote keyboard input and microphone-toggle handling.

### Added

- Mouse protocol and keyboard input regression tests.
- Contributor guide covering module boundaries, feature integration, and verification.

## [1.2.5] - 2026-11-25

### Added

- Complete project, architecture, security, build, test, packaging, and release documentation.

### Removed

- Camera streaming, virtual-camera drivers, camera UI, camera protocol state, camera assets, and camera packaging.

### Changed

- Desktop and Android session, settings, configuration, and release paths now expose only speaker, microphone, mouse, and keyboard capabilities.
