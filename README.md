<div align="center">

# BloatwareHatao

### The Ultimate Android Bloatware Removal Tool

### Unified Android Device Optimization Platform

### ⚛ Rust-Powered ⚛ TUI-First ⚛

#### Package Management, Device Health, and System Optimization in One Framework

BloatwareHatao is a unified Android bloatware removal and device optimization toolkit built entirely in Rust. It provides a comprehensive TUI workspace alongside a powerful CLI, enabling users to perform package analysis, safe removal, device health monitoring, and system optimization from a single modular framework.

The platform integrates advanced capabilities including intelligent package safety classification, OEM-specific package databases, rescue and restore functionality, wireless ADB management, device health monitoring, and preset-based removal workflows. BloatwareHatao supports modern Android ecosystems across major OEMs including Samsung, Xiaomi, Huawei, OnePlus, OPPO, Vivo, Realme, Nothing, Motorola, and more.

With a comprehensive package database of 5,000+ packages across 26 OEM and functional categories, safety ratings (User, Recommended, Advanced, Unsafe, Danger), and intelligent app name extraction, BloatwareHatao enables users to safely reclaim storage, enhance performance, and take control of their Android device experience within one unified environment.

Connect your device and begin advanced Android bloatware removal and optimization.

<br>

[![Platform](https://img.shields.io/badge/Platform-macOS%20%7C%20Linux%20%7C%20Windows-brightgreen)]()
[![Version](https://img.shields.io/badge/Release-v0.1.0-red)]()
[![License](https://img.shields.io/badge/License-GPLv3-blue)]()

<br>

</div>

## Installation

## Quick Start

### TUI (Default)

```bash
bloatwarehatao
```

### CLI (Headless)

```bash
# Show device information
bloatwarehatao --device-info

# List installed packages
bloatwarehatao --list-packages

# Dry run mode (no changes)
bloatwarehatao --dry-run
```

## Product Priority

- **TUI is the main product and default experience.** Use `bloatwarehatao` for day-to-day package management, device health monitoring, preset application, and operator-guided operations.
- **CLI is the secondary surface.** Use `bloatwarehatao --device-info`, `--list-packages`, and other flags for quick one-off tasks, scripting, and automation.

### TUI Keybindings

| Action | Keys |
|--------|------|
| Quit | q or Esc |
| Navigate menus | Arrow keys |
| Select/Confirm | Enter |
| Go back | b |
| Search packages | / |
| Toggle selection | Space |
| Select all | a |
| Deselect all | d |
| Next/Prev tab | Tab / Shift+Tab |
| Help | ? |

## Current Capabilities (v0.1.0)

### Core Platform

- ✅ Rust CLI with subcommands: dry-run, verbose, device-info, list-packages, offline mode
- 🔧 Full-screen TUI by default (`bloatwarehatao`) as the primary product surface
- ✅ Async ADB communication with device management
- ✅ Configuration management via TOML
- ✅ Structured logging with tracing
- ✅ Error handling with unique error codes

### Rust Core

- ✅ Async ADB operations (device list, connect, disconnect, tcpip mode)
- ✅ Package operations (list, uninstall, disable, enable, clear data, reinstall)
- ✅ Smart app name extraction from package names
- ✅ Package database with 5,000+ packages across 26 categories
- ✅ Safety rating system (User, Recommended, Advanced, Unsafe, Danger)
- ✅ OEM detection (Samsung, Google, Xiaomi, Huawei, OnePlus, OPPO, Vivo, Realme, Motorola, Nothing, Meizu, Infinix, Tecno, Itel, Amazon, Meta, Microsoft, ASUS, Sony, LG, Nokia)
- ✅ Preset system (built-in, custom, community with import/export)
- ✅ Rescue and restore system (rescue points, session tracking)
- ✅ Device health monitoring (battery, RAM, storage)
- ✅ Wireless ADB management

---

## Feature Matrix

### Package Management

- ✅ List installed packages (all, third-party, system, disabled)
- ✅ Package information retrieval (label, version, system/user)
- ✅ Uninstall packages (current user)
- ✅ Disable packages
- ✅ Enable packages
- ✅ Clear package data
- ✅ Reinstall packages
- ✅ Batch removal operations
- ✅ Multi-selection modes
- ✅ Dry run mode for safe preview

### Package Database

- ✅ 5,000+ packages with metadata
- ✅ Safety ratings (User, Recommended, Advanced, Unsafe, Danger)
- ✅ 26 OEM and functional categories
- ✅ Package descriptions and dependencies
- ✅ Alternative package suggestions
- ✅ Community voting system
- ✅ Search by name, OEM, category, safety rating
- ✅ Filter by installed status

### Preset System

- ✅ Built-in presets for common use cases
- ✅ Custom preset creation
- ✅ Preset import/export (JSON)
- ✅ Community preset support
- ✅ Package filtering by safety level
- ✅ Category-based targeting
- ✅ Tag-based filtering

### Rescue & Restore

- ✅ Create rescue points (package snapshots)
- ✅ Rescue history management
- ✅ Session tracking (removed packages with timestamps)
- ✅ Restore from rescue points
- ✅ Restore from sessions
- ✅ Device information in rescue entries
- ✅ Custom descriptions for rescue points

### Device Management

- ✅ Device detection and listing
- ✅ Device information (brand, model, Android version, SDK, build ID)
- ✅ OEM detection
- ✅ Multi-device support (target by serial)
- ✅ Device authorization status checking
- ✅ Wireless ADB enable/disable
- ✅ Wireless ADB connection management

### Device Health

- ✅ Battery level monitoring
- ✅ Battery temperature
- ✅ RAM usage percentage
- ✅ Storage usage percentage
- ✅ Real-time health metrics

### Wireless ADB

- ✅ Enable wireless debugging (USB required)
- ✅ Connect to device via IP:port
- ✅ Disconnect from wireless device
- ✅ TCP/IP mode configuration
- ✅ Device IP detection

---

## Requirements

- **OS**: macOS, Linux, Windows
- **Rust**: 1.75+ (for building from source)
- **ADB**: Android platform-tools (`adb`)
- **Device**: Android device with USB debugging enabled

## Configuration

BloatwareHatao stores configuration in the platform-appropriate data directory:

- **Linux**: `~/.local/share/bloatwarehatao/`
- **macOS**: `~/Library/Application Support/BloatwareHatao/`
- **Windows**: `%APPDATA%\BloatwareHatao\`

Configuration includes:

- Package database location
- Custom presets
- Rescue history
- Session logs
- Settings

## Package Database

The package database includes 5,000+ packages across:

**OEM Categories:**

- Samsung, Google, Xiaomi, Huawei, OnePlus, OPPO, Vivo, Realme, Motorola, Nothing, Meizu, Infinix, Tecno, Itel, Amazon, Meta, Microsoft, ASUS, Sony, LG, Nokia

**Functional Categories:**

- AOSP, System, Chipset, Carrier, Ads, Social, Productivity, Entertainment, Security, Finance, Health, Gaming, Shopping, News, Education, Misc, UserInstalled, Other

## Safety Ratings

| Rating | Meaning | Color |
|--------|---------|-------|
| User Installed | App installed by user. Safe to remove if desired. | Blue |
| Recommended | Safe to remove. Bloatware with no dependencies. | Green |
| Advanced | May affect some features. Generally safe for most users. | Yellow |
| Unsafe | May break functionality. Only for experienced users. | Orange |
| Danger | Critical system component. Removal may cause issues. | Red |

## Disclaimer

**BloatwareHatao: The Ultimate Android Bloatware Removal Tool** is developed for device optimization and educational purposes. It should be used responsibly and in compliance with all applicable laws and regulations. The developer of this tool is not responsible for any misuse or illegal activities conducted with this tool.

Package removal should only be performed with proper authorization and understanding of the implications. Removing system packages may affect device functionality. Always use dry run mode first and create rescue points before making changes. Ensure proper authorization before using BloatwareHatao for package removal. Always adhere to ethical practices and comply with all applicable laws and regulations.

## License

This project is licensed under the GPL-3.0-only License.

<h3 align="center">Happy Android Optimization with BloatwareHatao! 🚀�</h3>
