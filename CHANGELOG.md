# Changelog

All notable changes to this project will be documented in this file.

## [v0.2.0] - 2026-01-09

### 🚀 New Features

- **Advanced Presets System**:
  - Create custom presets from selected packages.
  - Import/Export presets (JSON format) for sharing.
  - Built-in presets for common OEMs (Samsung, Xiaomi, etc.).
  - "Dry Run" support for testing presets safely.
- **Rescue Mode**:
  - Automatically backups removed packages.
  - Restore uninstalled packages with a single click.
  - Filterable history of all operations.
- **Real-time Dashboard**:
  - Instant health monitoring (Battery, RAM, Storage).
  - Auto-reconnection handling for devices.
- **Smart Package Manager**:
  - "Safe to Remove" indicators based on community data.
  - Advanced filtering (bloatware, system, user apps).
  - Batch operations (Select multiple -> Uninstall/Disable).

### 🐛 Bug Fixes

- Fixed latency in displaying device health statistics.
- Improved error handling for ADB connection failures.

### 🛠️ Technical Improvements

- Reduced initial load time by parallelizing ADB queries.
- Improved TUI responsiveness and keyboard handling.

## [v0.1.0] - 2026-01-01

### Initial Release

- **Core TUI**: Robust terminal interface for bloatware removal.
- **Universal Database**: Support for 10,000+ package definitions.
- **ADB Integration**: Wireless and wired ADB connection support.
- **Safety Ratings**: Basic classification of packages (Unsafe, Advanced, Safe).
