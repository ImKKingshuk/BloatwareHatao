<div align="center">

# BloatwareHatao

### Android bloatware removal and device optimization from a Rust terminal UI

[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-brightgreen)]()
[![Release](https://img.shields.io/badge/release-v1.0.0-red)]()
[![License](https://img.shields.io/badge/license-GPL--3.0--only-blue)]()

</div>

BloatwareHatao is a TUI-first Android package management tool for reviewing installed apps, classifying bloatware, removing or disabling selected packages, creating rescue points, restoring packages, checking device health, and managing wireless ADB workflows.

The project is built in Rust and ships a local package database with 5,000+ Android package records, safety ratings, OEM/functional categories, descriptions, dependencies, alternatives, and tags. The CLI is intentionally small and is mainly for quick device/package checks; day-to-day package operations happen in the terminal UI.

## Installation

### Homebrew

macOS Apple Silicon is distributed through the Homebrew tap. The formula installs Android platform-tools automatically.

```bash
brew tap ImKKingshuk/tap
brew install bloatwarehatao
```

### Scoop

Windows x64 is distributed through the Scoop bucket. Install Android platform-tools separately if you need ADB-driven workflows.

```powershell
scoop bucket add imkkingshuk https://github.com/ImKKingshuk/scoop-bucket
scoop install imkkingshuk/bloatwarehatao
```

### GitHub Release Archives

Download the archive for your platform from the GitHub release page, extract it, and place `bloatwarehatao` or `bloatwarehatao.exe` on your `PATH`.

Release archives include:

- `bloatwarehatao` or `bloatwarehatao.exe`
- `README.md`
- `LICENSE`
- `CHANGELOG.md`
- `packages/`

Published asset names use this format:

```text
bloatwarehatao-1.0.0-x86_64-unknown-linux-gnu.tar.gz
bloatwarehatao-1.0.0-aarch64-unknown-linux-gnu.tar.gz
bloatwarehatao-1.0.0-aarch64-apple-darwin.tar.gz
bloatwarehatao-1.0.0-x86_64-pc-windows-msvc.zip
SHA256SUMS
```

### Build From Source

```bash
cargo build --release --workspace
./target/release/bloatwarehatao --version
```

The repository uses the stable Rust toolchain with `rustfmt` and `clippy`.

## Quick Start

Launch the terminal UI:

```bash
bloatwarehatao
```

Run in dry-run mode:

```bash
bloatwarehatao --dry-run
```

Print connected device information:

```bash
bloatwarehatao --device-info
```

List installed packages:

```bash
bloatwarehatao --list-packages
```

Target a specific device when multiple devices are connected:

```bash
bloatwarehatao --device <serial> --device-info
bloatwarehatao -s <serial> --list-packages
```

Enable verbose logs:

```bash
bloatwarehatao --verbose
```

## CLI Options

| Option | Purpose |
|--------|---------|
| `--dry-run` | Preview actions without changing the connected device |
| `-v`, `--verbose` | Enable more detailed logs |
| `--device-info` | Print device information and exit |
| `--list-packages` | Print installed package names and exit |
| `--offline` | Accepted offline-mode flag; offline behavior is currently controlled from TUI settings/config |
| `-s`, `--device <serial>` | Target a specific ADB device |
| `-h`, `--help` | Show help |
| `-V`, `--version` | Show version |

Package uninstall, disable, clear-data, restore, preset, and wireless ADB operations are available through the TUI and core package manager, not as standalone CLI subcommands in v1.0.0.

## TUI Workflows

The TUI is the primary product surface. It includes:

- Package Browser: search, filter, inspect safety ratings, select packages, and perform batch operations.
- User Guide: in-app usage help.
- Profiles: built-in and custom preset workflows.
- Device Info: brand, model, manufacturer, Android version, SDK version, build ID, serial, security patch, and detected OEM.
- Health Check: battery level, battery temperature, RAM usage, and storage usage.
- Backup & Restore: rescue point history, removal sessions, and package restore workflows.
- Settings: dry-run, removal mode, offline mode, and UI preferences.
- Wireless ADB: TCP/IP mode, IP detection, connect, and disconnect workflows.
- Support/Sponsor and About screens.

Common keys:

| Action | Keys |
|--------|------|
| Navigate | Arrow keys, `j`, `k` |
| Select/confirm | `Enter` |
| Back/quit current screen | `q`, `Esc` |
| Search packages | `/` |
| Clear search | `c` |
| Toggle package selection | `Space` |
| Refresh package list | `r` |
| Page navigation | `PgUp`, `PgDn`, `g`, `G` |
| Help | `?` |
| Force quit | `Ctrl+C` |

## Current Capabilities

### Device And ADB

- Async ADB command execution.
- ADB availability checks with clear error reporting.
- Connected-device listing and authorization status handling.
- Multi-device targeting by serial.
- Device OEM detection.
- Wireless ADB enable, connect, disconnect, and device IP detection.

### Package Operations

- List installed, system, and third-party packages.
- Retrieve package metadata such as APK path, version name, version code, enabled state, and system/user classification.
- Current-user uninstall through `pm uninstall --user 0`.
- Disable, enable, clear data, and reinstall existing system packages.
- Batch package operations from selected TUI rows.
- Dry-run behavior for package-changing actions.
- Operation results with success/failure status and error details.

### Package Database

- 5,089 unique package records in the bundled database.
- 36 categories currently represented in `packages/all_packages.json`.
- Safety ratings: User, Recommended, Advanced, Unsafe, and Danger.
- Search across package name, label, and description.
- Metadata support for OEMs, dependencies, alternatives, tags, and vote counts.
- Installed-package matching against the connected device.

### Presets

- Built-in presets for privacy, performance, minimal setup, social media cleanup, and De-Google workflows.
- Custom preset creation and deletion.
- JSON preset import/export.
- Preset review and application through the TUI.

### Rescue And Restore

- Rescue point creation from the current installed package list.
- JSON rescue history.
- Removal session tracking with timestamps and operation method.
- Restore from rescue points or removal sessions.

### Configuration

BloatwareHatao stores configuration and runtime data in platform-appropriate application directories.

Common data locations:

- Linux: `~/.local/share/bloatwarehatao/`
- macOS: `~/Library/Application Support/com.imkkingshuk.bloatwarehatao/`
- Windows: `%APPDATA%\imkkingshuk\bloatwarehatao\`

Configuration supports:

- Default removal mode: uninstall, disable, or clear data.
- Dry-run mode.
- Backup-before-remove preference.
- Verbose logging.
- Offline mode.
- UI preferences.
- Custom ADB path.

## Package Safety Ratings

| Rating | Meaning |
|--------|---------|
| User Installed | App installed by the user. Safe to remove if desired. |
| Recommended | Usually safe to remove. Intended for common bloatware with no known critical dependency. |
| Advanced | May affect device features. Review before removing. |
| Unsafe | Can break functionality. Only for experienced users. |
| Danger | Critical or high-risk component. Avoid removal unless you know exactly why. |

## Requirements

- macOS, Linux, or Windows.
- Android platform-tools (`adb`) for device workflows.
- Android device with USB debugging enabled and this computer authorized.
- Rust stable if building from source.

## Release Flow

GitHub Releases are the source of truth for packaged binaries. Homebrew and Scoop entries consume those release assets and their SHA-256 hashes.

For `v1.0.0`, the release workflow builds:

- Linux x86_64
- Linux ARM64
- macOS Apple Silicon
- Windows x64

The release workflow also publishes `SHA256SUMS`.

## Safety And Privacy

- BloatwareHatao does not require telemetry or a hosted service.
- Device interaction is performed through local ADB commands.
- Package removal only affects the connected Android device and selected packages.
- Use dry-run mode before bulk actions.
- Create rescue points before removing packages.
- Review safety ratings and descriptions before removing system packages.
- Disable USB debugging when you are not using it.
- Only authorize ADB from computers you trust.

## License

BloatwareHatao is licensed under the GPL-3.0-only License. See [LICENSE](LICENSE) for details.
