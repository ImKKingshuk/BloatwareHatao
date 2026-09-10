<div align="center">

# BloatwareHatao

### The Ultimate Android Bloatware Removal Tool

### Unified Android Debloating & Optimization Platform

### ⚛ Rust Powered ⚛ Ratatui Accelerated ⚛

#### Classifying, Disabling, Removing, Backing Up, and Restoring in One Workspace

BloatwareHatao is a unified Android package management and device optimization toolkit built with a high-performance Rust core and an asynchronous execution engine. It provides a case-driven TUI workspace alongside a clean CLI, enabling users to perform package inspection, safety scoring, batch debloating, custom presets management, and wireless ADB configurations from a single modular terminal environment.

The platform integrates advanced capabilities including a local database of 5,000+ package definitions, OEM classification filters, granular rescue and restore points, real-time device health metrics, and modern Android safety ratings. BloatwareHatao supports modern Android ecosystems, including system package disablement constraints, OEM-specific behavior, and wireless debugging connections.

With a growing ecosystem of package categories and safety ratings, BloatwareHatao enables developers and power users to orchestrate complex Android debloating operations and restore devices to a clean, performant state within one unified environment.

Connect your device and begin optimizing your Android system.

<br>

![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-brightgreen)
[![Version](https://img.shields.io/badge/Version-v1.0.0-red)](https://github.com/ImKKingshuk/BloatwareHatao/releases)
[![License: GPL-3.0-only](https://img.shields.io/badge/License-GPL--3.0--only-blue.svg)](LICENSE)

<br>
</div>

## Modern Era: Rust + Ratatui Native (v1.x)

- **Rust Primitives**: Rust powers the entire application from CLI option parsing to terminal drawing.
- **Async Engine**: High-performance asynchronous ADB orchestration keeps the UI responsive during long device scans.
- **Local Registry**: Bundles a rich, offline-first JSON registry detailing 5,000+ package profiles, categories, and safety ratings.

## Installation

### GitHub Releases

Download the v1.0.0 archive for your platform from [GitHub Releases](https://github.com/ImKKingshuk/BloatwareHatao/releases):

| Platform | Release asset |
| --- | --- |
| Linux x86-64 | `bloatwarehatao-1.0.0-x86_64-unknown-linux-gnu.tar.gz` |
| Linux ARM64 | `bloatwarehatao-1.0.0-aarch64-unknown-linux-gnu.tar.gz` |
| macOS Apple Silicon | `bloatwarehatao-1.0.0-aarch64-apple-darwin.tar.gz` |
| Windows x86-64 | `bloatwarehatao-1.0.0-x86_64-pc-windows-msvc.zip` |

Each release includes `SHA256SUMS`. On macOS or Linux, verify a downloaded archive before extracting it:

```bash
ARCHIVE=bloatwarehatao-1.0.0-aarch64-apple-darwin.tar.gz
grep " ${ARCHIVE}$" SHA256SUMS | shasum -a 256 -c -
```

Keep the extracted `packages/` directory with the application directory; it contains the local package registry used by the TUI.

### Homebrew (macOS)

macOS Apple Silicon is distributed through the Homebrew tap. Install Android platform-tools separately for ADB-driven workflows.

```bash
brew tap ImKKingshuk/tap
brew install --cask android-platform-tools
brew install bloatwarehatao
```

### Scoop (Windows)

Windows x64 is distributed through the Scoop bucket. Install Android platform-tools separately if you need ADB-driven workflows.

```powershell
scoop bucket add imkkingshuk https://github.com/ImKKingshuk/scoop-bucket
scoop install imkkingshuk/bloatwarehatao
```

## Quick Start

### TUI (Default)

```bash
bloatwarehatao
```

### CLI (Quick Actions)

```bash
# Print connected device information
bloatwarehatao --device-info

# List installed packages
bloatwarehatao --list-packages
```

### Dry-Run Simulation

```bash
bloatwarehatao --dry-run
```

## Product Priority

- **TUI is the main product and default experience.** Use `bloatwarehatao` for day-to-day investigations, interactive package browsing, profile preset selection, and operator-guided debloating.
- **Headless CLI is the secondary surface.** Use `bloatwarehatao --list-packages` or `bloatwarehatao --device-info` for quick checks, scripting, automation, and headless environments.

### TUI

#### Keybindings

| Action                   | Keys                     |
| ------------------------ | ------------------------ |
| Navigate                 | Arrow keys, `j`, `k`     |
| Select/confirm           | `Enter`                  |
| Back/quit current screen | `q`, `Esc`               |
| Search packages          | `/`                      |
| Clear search             | `c`                      |
| Toggle package selection | `Space`                  |
| Refresh package list     | `r`                      |
| Page navigation          | `PgUp`, `PgDn`, `g`, `G` |
| Help                     | `?`                      |
| Force quit               | `Ctrl + C`               |

### TUI vs CLI

| Mode                       | Best for                                                                                                       | Command                          |
| -------------------------- | -------------------------------------------------------------------------------------------------------------- | -------------------------------- |
| TUI (primary)              | Interactive package selection, batch uninstalls, preset reviews, backup restoration, and configuration editing | `bloatwarehatao`                 |
| CLI / headless (secondary) | Quick device posture checks, pipeline automation, scripting, and offline queries                               | `bloatwarehatao --list-packages` |

---

## Features Status Legend

| Icon | Status             | Meaning                                                             |
| ---- | ------------------ | ------------------------------------------------------------------- |
| ✅    | `production-ready` | Stable core workflow with strong local/offline behavior             |
| 🔧    | `functional`       | Useful and working, with practical constraints                      |
| 🔬    | `best-effort`      | Works in some environments, but highly device/app/version dependent |
| 🚧    | `experimental`     | Early workflow with notable limitations                             |
| 🔑    | `dependency-gated` | Requires optional extras, external tools, or credentials            |

---

## Current Capabilities (v1.0.0)

### Core Platform

- ✅ High-performance Rust binary orchestrating the CLI and terminal workspace.
- ✅ Full-screen interactive TUI built with Ratatui and Crossterm for all-round operators.
- ✅ Persistent settings for dry-run mode, confirmations, and pre-uninstall rescue points.
- ✅ Built-in safety guards with dry-run support, automatic backups, and removal confirmations.
- ✅ Platform-appropriate configurations for Linux, macOS, and Windows.
- ✅ Clean error handling with project-specific diagnostics and structured logging.

### Rust Core (Native)

- ✅ Asynchronous command execution loops for high-speed ADB communication.
- ✅ High-speed package name mapping and OEM classification engine.
- ✅ Multi-select buffer management for smooth batch uninstalls and restores.
- ✅ Dynamic memory allocation for package records with robust safety checks.

### Device & Orchestration

- 🔧 Async ADB checks and serial number targeting (`bloatwarehatao --device <serial>`).
- 🔧 Real-time Android device status detection (ready, unauthorized, offline, recovery).
- 🔧 Device OEM extraction to tailor database categorization.
- 🔧 Wireless ADB configuration helper with device IP detection.

---

## Features

### Package Operations

- ✅ Current-user system package uninstalls (`pm uninstall --user 0`).
- ✅ Package disablement (`pm disable-user`) and enablement (`pm enable`).
- 🔧 Package data clearing (`pm clear`) to regain storage.
- ✅ Bulk/batch execution on selected rows.
- ✅ Reinstallation of uninstalled system apps (`cmd package install-existing`).

### Local Package Database

- ✅ 5,089 unique package records bundled directly.
- ✅ 36 categories covering major OEMs (Samsung, Google, Xiaomi, Huawei, OnePlus, Oppo, Vivo, Motorola, etc.) and subsystems (Carrier, Chipset, AOSP, Ads).
- ✅ Safety ratings ranging from **Recommended** to **Danger**.
- ✅ Live search and categorization filtering within the TUI.

### Presets & Profiles

- 🔧 Built-in presets for Privacy, Performance, De-Google, and Social Media cleanup.
- 🔧 Custom profile creation, editing, and deletion.
- 🔧 JSON preset import and export for sharing optimization templates.

### Rescue & Restore

- ✅ Automatic rescue point generation before starting batch removals.
- ✅ Local session logs for successful uninstalls, including timestamps and operation type.
- ✅ Dynamic restoration from rescue logs or package lists.

### Device Health Audit

- 🔧 Real-time battery status, level, and temperature checks.
- 🔧 Active RAM memory usage (Free, Used, Total) representation.
- 🔧 Storage partition space (System, Data) utilization.

### Wireless ADB Configuration

- 🔬 Wireless TCP/IP mode activation on port 5555.
- 🔬 Automated local IP detection and connecting workflows.

---

## Package Safety Ratings

| Rating             | Meaning                                                                              |
| ------------------ | ------------------------------------------------------------------------------------ |
| **User Installed** | App installed by the user. Safe to remove if desired.                                |
| **Recommended**    | Safe to remove. Intended for common bloatware with no known critical dependencies.   |
| **Advanced**       | May affect certain device features. Review description before removing.              |
| **Unsafe**         | Can break basic system functionality. Only for experienced users.                    |
| **Danger**         | Critical system component. Avoid removal unless you know exactly what you are doing. |

---

## Requirements

- **OS**: macOS, Linux, or Windows.
- **ADB**: Android platform-tools (`adb`) installed and available on system `PATH`.
- **Device**: Android device with USB debugging enabled and computer authorized.
- **Rust**: Required to build from source (stable toolchain).

## Safety & Privacy

- **No Telemetry**: BloatwareHatao has no analytics, tracking, update checker, or cloud service. It does not upload device or usage data.
- **Local Action**: Device data is processed locally. The only network-capable workflow is an explicitly initiated ADB connection to an address supplied or detected on the local network.
- **Safety Controls**: Confirmation and automatic rescue points are enabled by default before uninstalls. Dry-run mode is available with `--dry-run` or from Settings.
- **Authorization**: Only authorize ADB connections from hosts you fully trust. Keep USB debugging disabled when not in use.

## Disclaimer

**BloatwareHatao : The Ultimate Android Bloatware Removal Tool** is developed for research, customization, and educational purposes. It should be used responsibly and in compliance with all applicable laws and regulations. The developer of this tool is not responsible for any misuse, bricked devices, data loss, or illegal activities conducted with this tool.

Removing system packages can impact device stability and safety updates. Ensure proper backups are created and review package safety ratings before initiating bulk operations. Always adhere to best device management practices.

## License

This project is licensed under the GPL-3.0-only License. See [LICENSE](LICENSE) for details.

<h3 align="center">Happy Android Debloating with BloatwareHatao! 📱⚡</h3>
