<h1 align="center">BloatwareHatao</h1>
<h3 align="center">v2.0.0</h3>

BloatwareHatao - Your Ultimate Android Bloatware Removal Tool! 📱✨ Say goodbye to unwanted bloatware on your Android device with BloatwareHatao. Cleanse your device effortlessly, reclaiming space and enhancing performance. 💪✨ Take control of your smartphone experience with BloatwareHatao!

## What's New (v2.0.0)

- **Complete Architecture Revamp**: Modular design with separate UI, core, data, and utility modules
- **Smart Removal Wizard**: Guided flow that tailors cleaning intensity to the user’s comfort level
- **Advanced Features**: Backup/restore, dry-run mode, batch operations, custom scripts, smart audits
- **Enhanced UI/UX**: Colored output, progress bars, comprehensive menus, planner, and device health insights
- **Robustness**: Better error handling, validation, logging, safety checks, and rescue list generation
- **Command Line Interface**: Support for CLI arguments and automation (including audits, health checks, and planner tools)
- **Device Management**: Device detection, info display, health snapshot, and connection validation
- **Statistics & Logging**: Detailed operation logs, session reports, and removal statistics
- **NDJSON Session Reports**: Optional structured logs for post‑processing and analytics
- **Richer Health Snapshot**: Battery voltage/health/technology, SoC temperature, memory, CPU, storage, and network states

## Features

- 📱 **Bloatware Removal**: Effortlessly remove unwanted bloatware from your Android device.
- 💪 **Safe & Recommended Cleaner**: Choose the safe and recommended cleaner for a standard bloatware removal.
- ⚡ **Pro Bloatware Cleaner**: Dive deeper with extra cleaning options for a thorough bloatware removal.
- 🌟 **Ultra Bloatware Cleaner**: For extreme cleaning, select the ultra cleaner option to rid your device of every trace of bloatware.
- 💼 **Manual Bloatware Cleaner**: Enter the APK package name to remove specific bloatware manually.
- 📋 **Batch Removal**: Remove multiple packages from a file or custom scripts.
- 🔄 **Backup & Restore**: Create backups before removal and restore packages when needed.
- 🧪 **Dry Run Mode**: Preview what would be removed without making changes.
- 🧠 **Smart Removal Wizard**: Guided, conversational flow to recommend safe, pro, or ultra cleaning levels.
- 🔍 **Pre-removal Audits**: Inspect remote script packages before removal, with system-app warnings.
- ❤️ **Device Health Snapshot**: Monitor battery, voltage, health, technology, SoC temperature, memory, CPU, storage, uptime, Wi‑Fi, airplane mode, and mobile data state.
- 🧭 **Cleaning Planner**: Capture goals, reminders, and notes for future cleanups.
- 🛟 **Rescue Lists & Session Reports**: Automatically log removal operations and export rescue lists of removed packages.
- 📊 **Statistics & Logging**: Track operations, view logs, reports, and monitor removal statistics.
- ⚙️ **Settings & Configuration**: Customize behavior with persistent settings.
- 🧾 **NDJSON Reports**: Enable structured NDJSON logging via Settings and view with `--report-ndjson`.
- 📴 **Offline Mode**: Use local OEM scripts when connectivity is limited.
- 🧹 **Removal Mode**: Choose default behavior: `uninstall`, `disable`, or `clear` (toggle in Settings).
- 🔍 **Device Information**: View detailed connected device information.
- 🔄 **Auto Updates**: Automatically checks for updates and updates itself to ensure you have the latest version of BloatwareHatao.
- 🛠️ **Easy-to-Use Interface**: Interactive menus with colored output and progress indicators.
- 📁 **Manufacturer & OS Version Selection**: Choose your device manufacturer and OS version for precise bloatware removal.
- 🛡️ **Safety Features**: Comprehensive validation, confirmations, and error handling.

## Command Line Options

BloatwareHatao now supports command line arguments for automation:

```bash
./BloatwareHatao.sh [options]

Options:
  --help, -h          Show help message
  --version, -v       Show version information
  --dry-run           Enable dry run mode
  --device-info       Show connected device information
  --smart-wizard      Launch guided smart removal wizard
  --audit <type> <manufacturer> <os-slug>
                      Run pre-removal audit (e.g., --audit Safe samsung oneui-6)
  --health            Show device health snapshot
  --planner           Open cleaning planner
  --report            Display current session report
  --report-ndjson     Display NDJSON session report (if enabled)
  --backup            Create backup of installed packages
  --restore           Interactive restore menu
  --log               Show operation logs
  --stats             Show removal statistics
  --mode <uninstall|disable|clear>
                      Set removal behavior (default: uninstall)
  --offline           Use local OEM scripts (offline mode)
```

## Device OEM / OS Support

- Samsung : OneUI 6, OneUI 5, OneUI 4
- Huawei : EMUI 14, EMUI 13, EMUI 12
- Honor: MagicUI 8, MagicUI 7, MagicUI 6
- Xiaomi : HyperOS 1, MIUI 14, MIUI 13
- OnePlus : OxygenOS 14, OxygenOS 13, OxygenOS 12
- Realme : RealmeUI 5, RealmeUI 4, RealmeUI 3
- Vivo : FuntouchOS 14, FuntouchOS 13, FuntouchOS 12
- Oppo : ColorOS 14, ColorOS 13, ColorOS 12
- Nothing : NothingOS 3, NothingOS 2, NothingOS 1
- Motorola : HelloUI 1, MyUX 13, MyUX 12
- Meizu : FlymeAIOS 11, FlymeOS 10, FlymeOS 9
- Infinix : XOS 14, XOS 13, XOS 12

## Requirements

- macOS, Linux, Windows
- Bash-compatible environment.
- Internet connectivity for fetching manufacturer-specific bloatware removal scripts.
- Android Device with [ADB (Android Debug Bridge)](https://developer.android.com/tools/adb) Enabled
- [Android SDK Platform-Tools](https://developer.android.com/tools/releases/platform-tools) installed properly and added to your system's PATH.

## Usage

### Interactive Mode

1. Clone the repository:

   ```bash
   git clone https://github.com/ImKKingshuk/BloatwareHatao.git
   cd BloatwareHatao
   ```

2. Run BloatwareHatao:

   ```bash
   bash BloatwareHatao.sh
   ```

3. Follow the interactive menus to select your options.

### Command Line Mode

For automation or specific tasks:

```bash
# Show device info
./BloatwareHatao.sh --device-info

# Create backup
./BloatwareHatao.sh --backup

# Dry run mode
./BloatwareHatao.sh --dry-run

# Show statistics
./BloatwareHatao.sh --stats

# View structured NDJSON report (enable in Settings)
./BloatwareHatao.sh --report-ndjson

# Switch removal mode (uninstall|disable|clear)
./BloatwareHatao.sh --mode disable

# Work offline using local OEM scripts
./BloatwareHatao.sh --offline
```

## Disclaimer

⚠️⚠️⚠️ "The Developer of this tool is not responsible for any type of damage caused by the misuse of this tool. Use at your own risk." ⚠️⚠️⚠️
BloatwareHatao is designed for removing bloatware from Android devices. Ensure that you have proper authorization before using it. Removing system apps may lead to unintended consequences, so use it wisely and responsibly.
BloatwareHatao is created to simplify the process of removing bloatware from Android devices. It is meant for personal use and enhancing the user experience. The developer of this tool is not responsible for any misuse.

## Contributing

Contributions are welcome!
Feel free to report issues or submit pull requests to enhance BloatwareHatao.

Feel free to contribute to the project by reporting issues or submitting pull requests!

### 💪 Reclaim Control of Your Android Device with BloatwareHatao! 💪
