# Changelog

All notable changes to `BloatwareHatao : The Ultimate Android Bloatware Removal Tool` will be documented in this file.

## [v1.0.0] - 2026-09-11

### The Debut Release

This release introduces the initial public release of BloatwareHatao, a high-performance Rust-powered Android bloatware removal and device optimization toolkit featuring a TUI-first workflow and a streamlined headless CLI.

### 🎨 TUI Experience

- **Full-Screen Terminal Workspace**: Interactive dashboard with real-time UI components built on Ratatui and Crossterm.
- **Visual Package Browser**: Split list-and-details view with safety ratings, descriptions, and keyboard-driven selection.
- **Dynamic Search & Filtering**: Instant case-insensitive package search (`/` key) with safety and category filters.
- **Global Help Overlay**: Accessible keyboard shortcut overlay (`?` key) detailing all TUI actions and navigation options.
- **Dedicated Health Checks**: Terminal UI dashboard to monitor device health, hardware temperatures, battery level, RAM, and storage partition layouts.
- **Settings Controller**: Persistent controls for dry-run simulation, removal confirmation, and automatic pre-uninstall rescue points.
- **Interactive Guides**: Built-in visual user guides, sponsor details, and licensing references inside the TUI dashboard.
- **Background App Name Resolution**: Asynchronous name fetching to keep the UI buttery smooth while package labels resolve in the background.

### ⚡ CLI Experience

- **TUI Launcher**: Starting the app without arguments launches the full-screen terminal TUI by default.
- **Device Audits**: The `--device-info` flag prints connected Android identity, build, security patch, and OEM details and exits.
- **Fast Package Listings**: The `--list-packages` flag prints installed package names on standard output.
- **Dry-Run Simulation**: Global `--dry-run` flag starts the application in simulation mode to safely test workflows.
- **Target Selection**: CLI flag `-s` / `--device <serial>` overrides the default target when multiple ADB devices are plugged in.
- **Log Verbosities**: Detailed runtime diagnostic printing via the `-v` / `--verbose` flag.

### 📱 ADB & Device Management

- **Asynchronous ADB Core**: Fully asynchronous process execution loops to communicate with ADB without blocking user interactions.
- **ADB Availability Checks**: Diagnostic checks during startup to report clear instructions if Android platform-tools are missing from system `PATH`.
- **Intelligent Status Detection**: Automated categorization of device statuses including ready, unauthorized, offline, recovery, and unknown.
- **OEM Auto-Detection**: Extracting build attributes to detect device manufacturers (Samsung, Xiaomi, Google, etc.) and tailor package listings.
- **Wireless ADB Helper**: Automated TCP/IP mode pairing on port 5555 with local IP scanning, connection, and disconnection workflows.

### 📦 Package Management

- **User-Level Debloating**: Safe system uninstalls via `pm uninstall --user 0` without requiring root permissions.
- **Package Disablement**: Support for disabling system packages via `pm disable-user` and re-enabling them via `pm enable`.
- **Data Cleanup**: Storage space restoration via `pm clear` to purge cache and application data.
- **Batch Processing**: Sequential execution of uninstall, disable, enable, or clear actions on multiple selected packages.
- **System Restoration**: Reinstallation helpers (`cmd package install-existing`) to restore previously uninstalled system packages.

### 🗄️ Package Database

- **Native Package Registry**: Bundled offline package database featuring 5,089 unique application profiles.
- **Comprehensive Classifications**: Mapping across 36 categories representing OEMs, chipset components, carriers, and user applications.
- **Safety Assessments**: Categorization into User Installed, Recommended, Advanced, Unsafe, and Danger to prevent critical brick issues.
- **Package Attribute Mapping**: Real-time cross-referencing between device package names and bundled database entries.

### 📑 Presets & Custom Profiles

- **Built-in Templates**: Pre-configured profiles for De-Google, Privacy enhancement, Performance optimization, and Social Media cleanups.
- **Profile Managers**: Create, export, reload, and delete custom JSON presets; import by placing JSON files in the custom preset directory.
- **Review Before Action**: Applying a preset selects matching installed packages in the browser for review before any operation runs.

### 🛡️ Rescue, Backup & Restore

- **Automated Rescue Points**: Snapshotting the device's installed package inventory before performing uninstalls.
- **Removal History Logs**: Local JSON logs tracking successful package uninstalls, methods, and timestamps.
- **One-Click Restoration**: Easy recovery workflows to restore packages from rescue points or historical removal sessions.

### 📊 Device Health Audit

- **Battery Statistics**: On-demand battery status, capacity percentage, and temperature readings.
- **Memory Auditing**: Hardware RAM analysis including total, used, and free memory metrics.
- **Storage Partitions**: Space analysis mapping System and Data mount utilization.

### 🔒 Safety & Privacy

- **Zero Telemetry**: No analytics, tracking, update checker, cloud service, or device-data upload.
- **Local Interaction Only**: Device data stays on the host; wireless ADB connects only when the operator explicitly requests it.
- **Safeguard Prompts**: Confirmation is enabled by default before package operations.

### 📋 Known Notes

- **System Requirements**: ADB must be installed and globally accessible on system `PATH`. Homebrew users can install it with `brew install --cask android-platform-tools`.
- **OEM Limitations**: Some OEM packages might automatically re-enable or restrict disablement based on active Android version constraints.
