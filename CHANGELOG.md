# Changelog

All notable changes to this project will be documented in this file.

## [v1.0.0] - 2026-06-21

First public release of BloatwareHatao, a Rust-powered Android bloatware removal and device optimization toolkit with a TUI-first workflow and a secondary CLI surface for headless checks.

### Highlights

- First stable release for GitHub Releases, Homebrew, and Scoop distribution.
- Full-screen terminal UI as the primary product experience.
- Cross-platform release archives for macOS, Linux, and Windows.
- Packaged local package database included with release artifacts.
- GPL-3.0-only licensed release with security and safe-usage guidance.

### TUI Experience

- Keyboard-driven terminal interface built with Ratatui and Crossterm.
- Home dashboard with device status and primary workflow navigation.
- Package browser with search, filtering, multi-select, and batch actions.
- Dedicated screens for device information, health checks, presets, rescue history, settings, wireless ADB, user guide, support, and about/license details.
- Global help overlay and practical navigation shortcuts.
- Dry-run mode support for previewing actions without changing the device.
- Background package label fetching so the UI can load quickly while app names are refined.

### CLI Experience

- `bloatwarehatao` launches the TUI by default.
- `bloatwarehatao --device-info` prints connected Android device details.
- `bloatwarehatao --list-packages` lists installed packages.
- `bloatwarehatao --dry-run` starts the app in simulation mode.
- `bloatwarehatao --offline` is accepted as an offline-mode flag; offline behavior is currently managed through TUI settings/config.
- `bloatwarehatao --verbose` enables more detailed logs.
- `bloatwarehatao --device <serial>` targets a specific device when multiple devices are connected.
- `bloatwarehatao --version` reports the installed release version.

### ADB And Device Management

- Async ADB command execution with device serial targeting.
- ADB availability detection with clear errors when Android platform-tools are missing.
- Connected device discovery with ready, unauthorized, offline, recovery, sideload, and unknown status handling.
- Device information retrieval, including brand, model, manufacturer, Android version, SDK version, build ID, serial, security patch, and detected OEM.
- Multi-device support through explicit serial selection.
- Wireless ADB support for TCP/IP mode, IP detection, connect, and disconnect workflows.

### Package Management

- Installed package listing.
- System package and third-party package discovery.
- Package metadata retrieval, including APK path, version name, version code, enabled state, and system/user classification.
- Current-user uninstall operations.
- Disable, enable, clear data, and reinstall operations.
- Batch package operations from the TUI.
- Operation results with success/failure status and error details.
- Smart app name extraction from package names for faster browsing.

### Package Database

- Local JSON package database bundled into release archives.
- 5,000+ Android package records across OEM and functional categories.
- Package names, labels, descriptions, dependencies, alternatives, and metadata.
- Safety ratings: User, Recommended, Advanced, Unsafe, and Danger.
- OEM/category support for Samsung, Google, Xiaomi, Huawei, OnePlus, OPPO, Vivo, Realme, Motorola, Nothing, Meizu, Infinix, Tecno, Itel, Amazon, Meta, Microsoft, ASUS, Sony, LG, Nokia, AOSP, System, Chipset, Carrier, Ads, Social, Productivity, Entertainment, Security, Finance, Health, Gaming, Shopping, News, Education, Misc, UserInstalled, and Other.
- Installed-status matching between the connected device and the local database.

### Presets

- Built-in presets for privacy, performance, minimal setup, social media cleanup, and De-Google workflows.
- Custom preset creation and deletion.
- JSON preset import and export.
- Preset package selection and review from the TUI.
- Preset application support with dry-run preview behavior.

### Rescue And Restore

- Rescue point creation from the current installed package list.
- Rescue history persisted as JSON in the platform data directory.
- Session tracking for removed packages with timestamps and operation method.
- Restore packages from rescue points.
- Restore packages from removal sessions.
- Dry-run restore previews.

### Device Health

- Battery level reporting.
- Battery temperature reporting.
- RAM usage reporting.
- Storage usage reporting.
- Real-time health screen in the TUI.

### Configuration And Data Storage

- Platform-appropriate data directory handling.
- TOML-based configuration support.
- Persistent locations for settings, presets, rescue points, sessions, and package database data.
- Structured logging with `tracing`.
- Typed error handling with project-specific error categories.

### Release Engineering

- GitHub Actions release workflow for tagged releases.
- Pre-release checks for formatting, clippy, tests, dependency/license checks, and multi-target builds.
- Release artifacts include the binary, `README.md`, `LICENSE`, `CHANGELOG.md`, and `packages/`.
- `SHA256SUMS` generation for release artifacts.
- Homebrew tap workflow support for creating/updating `Formula/bloatwarehatao.rb` from GitHub Release assets.
- Scoop bucket workflow support for creating/updating `bucket/bloatwarehatao.json` from GitHub Release assets.
- Release builds remap local filesystem paths to avoid embedding developer machine paths in binaries.

### Safety And Privacy

- No telemetry or network service is required by the application itself.
- Device interaction is performed through local ADB commands.
- Package removal is scoped to the connected Android device and selected packages.
- Dry-run mode and rescue points are available for safer workflows.
- Security policy documents ADB usage, backup expectations, sensitive file exclusions, and responsible reporting.

### Known Notes

- ADB must be installed and available on `PATH` for device-driven workflows.
- Homebrew installs Android platform-tools automatically through the formula.
- Scoop users should install Android platform-tools separately when ADB workflows are needed.
- Removing system packages can affect device functionality; review safety ratings and create rescue points before bulk operations.
