<div align="center">

<img src="https://bloatwarehatao.vercel.app/icon.png" width="75" alt="BloatwareHatao Icon"/>

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

[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-brightgreen)]()
[![Release](https://img.shields.io/badge/Release-v1.0.0-red)]()
[![License](https://img.shields.io/badge/License-GPLv3-blue)]()

<a href="https://bloatwarehatao.vercel.app">
    <img width="180" src="https://img.shields.io/badge/Website-BloatwareHatao-blue?logo=google-chrome&style=square" alt="Website"/>
</a>

<br>
</div>

## Modern Era: Rust + Ratatui Native (v1.x)

- **Rust Primitives**: Rust powers the entire application from CLI option parsing to terminal drawing.
- **Async Engine**: High-performance asynchronous ADB orchestration keeps the UI responsive during long device scans.
- **Local Registry**: Bundles a rich, offline-first JSON registry detailing 5,000+ package profiles, categories, and safety ratings.

## Installation

### Homebrew (macOS)

macOS Apple Silicon is distributed through the Homebrew tap. The formula installs Android platform-tools automatically.

```bash
brew tap ImKKingshuk/tap
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

### TUI positioning vs Universal Android Debloater (UAD), ADB CLI, and Android Settings

BloatwareHatao is designed as a **guided operator workspace** that spans classification, batch processing, safety safeguards, and device metrics. The tools below are still valuable, but they solve narrower slices of the Android debloating workflow.

| Tool                 | Primary strength                                                                           | Main surface         | Best at                                                                                                       | Gaps relative to BloatwareHatao                                                                          |
| -------------------- | ------------------------------------------------------------------------------------------ | -------------------- | ------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| **BloatwareHatao**   | Unified terminal workspace for device audits, package classification, and batch operations | Terminal TUI + CLI   | Managing package uninstalls, safety ratings, custom presets, and rescue logs from a single terminal interface | N/A                                                                                                      |
| **UAD**              | Cross-platform desktop GUI for debloating                                                  | Desktop GUI (GTK/Qt) | Normalizing debloat lists into an easy-to-use desktop application                                             | No TUI interface, no integrated CLI for headless checks, lacks built-in terminal wireless ADB configs    |
| **ADB CLI**          | Low-level command-line execution                                                           | CLI shell            | Raw uninstalls (`pm uninstall`) and low-level debugging                                                       | No package database, no safety ratings, no visual selection UI, no safety nets (automatic rescue points) |
| **Android Settings** | Built-in system management UI                                                              | Mobile touchscreen   | Basic user app deletion and cache clearing                                                                    | Cannot uninstall system packages, tedious one-by-one flow, no batch debloating, no preset profiles       |

### Capability comparison: BloatwareHatao vs alternatives

| Capability                                            | BloatwareHatao |    UAD    |      ADB CLI      |  Android Settings   |
| ----------------------------------------------------- | :------------: | :-------: | :---------------: | :-----------------: |
| Terminal TUI operator dashboard                       |    ✅ Native    |     ❌     |         ❌         |          ❌          |
| Headless CLI automation / scripting                   |       ✅        |     ❌     |         ✅         |          ❌          |
| Bundled database of 5,000+ package records            |       ✅        |     ✅     |         ❌         |          ❌          |
| Safety Ratings classification (Recommended to Danger) |       ✅        |     ✅     |         ❌         |          ❌          |
| Automatic rescue point & removal session history      |       ✅        | ⚠️ Limited |         ❌         |          ❌          |
| Real-time device health metrics (RAM, storage, temp)  |       ✅        |     ❌     | ⚠️ Manual commands |     ⚠️ Sub-menus     |
| Custom Profile & Preset imports/exports               |       ✅        |     ✅     |         ❌         |          ❌          |
| Wireless ADB configuration helper                     |       ✅        |     ❌     | ⚠️ Manual pairing  | ⚠️ Developer Options |

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
- ✅ Structured configuration management loading from `config.toml`.
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
- 🔧 Wireless ADB configuration helper with automated IP scanning.

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
- ✅ Removed session logs detailing timestamp, operation type, and package status.
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

- **No Telemetry**: BloatwareHatao operates entirely offline. No data is collected, and no network calls are made.
- **Local Action**: Device interaction is restricted to local ADB commands executed on the host machine.
- **Non-Destructive Defaults**: Includes dry-run support, removal confirmations, and automatic rescue points before uninstalls.
- **Authorization**: Only authorize ADB connections from hosts you fully trust. Keep USB debugging disabled when not in use.

## Disclaimer

**BloatwareHatao : The Ultimate Android Bloatware Removal Tool** is developed for research, customization, and educational purposes. It should be used responsibly and in compliance with all applicable laws and regulations. The developer of this tool is not responsible for any misuse, bricked devices, data loss, or illegal activities conducted with this tool.

Removing system packages can impact device stability and safety updates. Ensure proper backups are created and review package safety ratings before initiating bulk operations. Always adhere to best device management practices.

## License

This project is licensed under the GPL-3.0-only License. See [LICENSE](LICENSE) for details.

<h3 align="center">Happy Android Debloating with BloatwareHatao! 📱⚡</h3>
