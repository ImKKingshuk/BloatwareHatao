# Changelog

All notable changes to `BloatwareHatao : The Ultimate Android Bloatware Removal Tool` will be documented in this file.

## [v1.0.0] - 2026-06-21

### The Debut Release

This release introduces the initial public release of BloatwareHatao, a high-performance Rust-powered Android bloatware removal and device optimization toolkit featuring a TUI-first workflow and a streamlined headless CLI.

### 🎨 TUI Experience

- **Full-Screen Terminal Workspace**: Interactive dashboard with real-time UI components built on Ratatui and Crossterm.
- **Visual Package Browser**: Advanced grid view with quick sorting, safety-rating badges, description tooltips, and keyboard-driven selection.
- **Dynamic Search & Filtering**: Instant case-insensitive package search (`/` key) with quick filters for system, user, and disabled packages.
- **Global Help Overlay**: Accessible keyboard shortcut overlay (`?` key) detailing all TUI actions and navigation options.
- **Dedicated Health Checks**: Terminal UI dashboard to monitor device health, hardware temperatures, battery level, RAM, and storage partition layouts.
- **Settings Controller**: Visual configuration editor to toggle dry-run simulation, default removal modes, and UI animation configurations on the fly.
- **Interactive Guides**: Built-in visual user guides, sponsor details, and licensing references inside the TUI dashboard.
- **Background App Name Resolution**: Asynchronous name fetching to keep the UI buttery smooth while package labels resolve in the background.

### ⚡ CLI Experience

- **TUI Launcher**: Starting the app without arguments launches the full-screen terminal TUI by default.
- **Device Audits**: Command line flags like `--device-info` print connected Android hardware, battery, and OEM parameters and exit.
- **Fast Package Listings**: Subcommand/flag `--list-packages` generates plain-text system and third-party package details on standard output.
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
- **Batch Processing**: Simultaneous execution of uninstall, disable, or clear actions on multiple selected packages.
- **System Restoration**: Reinstallation helpers (`cmd package install-existing`) to restore previously uninstalled system packages.

### 🗄️ Package Database

- **Native Package Registry**: Bundled offline package database featuring 5,089 unique application profiles.
- **Comprehensive Classifications**: Mapping across 36 categories representing OEMs, chipset components, carriers, and user applications.
- **Safety Assessments**: Categorization into User Installed, Recommended, Advanced, Unsafe, and Danger to prevent critical brick issues.
- **Package Attribute Mapping**: Real-time cross-referencing between device package names and bundled database entries.

### 📑 Presets & Custom Profiles

- **Built-in Templates**: Pre-configured profiles for De-Google, Privacy enhancement, Performance optimization, and Social Media cleanups.
- **Profile Managers**: Import and export mechanisms using standard JSON templates to save and share optimization profiles.
- **Dry-Run Previews**: Interactive preview sheets inside the TUI to review packages included in a profile before applying changes.

### 🛡️ Rescue, Backup & Restore

- **Automated Rescue Points**: Snapshotting the device's installed package inventory before performing uninstalls.
- **Removal History Logs**: Local JSON logs tracking package uninstalls, disablements, and timestamps.
- **One-Click Restoration**: Easy recovery workflows to restore packages from rescue points or historical removal sessions.

### 📊 Device Health Audit

- **Battery Statistics**: Live updates on battery status, capacity percentages, and operating temperatures.
- **Memory Auditing**: Hardware RAM analysis including total, used, and free memory metrics.
- **Storage Partitions**: Space analysis mapping System and Data mount utilization.

### 🔒 Safety & Privacy

- **Zero Telemetry**: Completely offline operations with no remote tracking, data gathering, or network phone-homes.
- **Local Interaction Only**: All communications occur locally through ADB commands on the connected host.
- **Safeguard Prompts**: Double-confirmation dialogs before committing destructive batch uninstalls.

### 📋 Known Notes

- **System Requirements**: ADB must be installed and globally accessible on system `PATH` (automatically bundled in Homebrew tap installation).
- **OEM Limitations**: Some OEM packages might automatically re-enable or restrict disablement based on active Android version constraints.
