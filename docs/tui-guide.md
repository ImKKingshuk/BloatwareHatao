# BloatwareHatao TUI Guide

This guide covers the **Terminal User Interface (TUI)** mode of BloatwareHatao. This mode is perfect for servers, headless environments, or users who prefer the command line efficiency.

## Launching the TUI

Run the binary from your terminal:

```bash
bloatwarehatao
```

Or via Cargo:
```bash
cargo run --bin bloatwarehatao
```

## Navigation Controls

The TUI is fully keyboard-driven.

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate lists and menus up/down |
| `Enter` | Select an item / Confirm action |
| `Esc` | Go back / Cancel / Exit |
| `Tab` | Switch focus between panels |
| `/` | Start searching/filtering |
| `Space` | Toggle selection (checkboxes) |

## Modes

### Interactive Mode
By default, running the command launches the interactive dashboard. You will see:
- Device status sidebar
- Package list main panel
- Log output window (bottom)

### CLI Arguments (Headless)

You can also use BloatwareHatao in "one-shot" CLI mode for scripts.

**List all packages:**
```bash
bloatwarehatao --list-packages
```

**Uninstall a specific package:**
```bash
bloatwarehatao --uninstall com.facebook.katana
```

**Disable a package:**
```bash
bloatwarehatao --disable com.facebook.katana
```

**Target a specific device (if multiple connected):**
```bash
bloatwarehatao -s <SERIAL_NUMBER> --list-packages
```

**Dry Run (Simulation):**
Add `--dry-run` to any command to simulate the action without changes.
```bash
bloatwarehatao --uninstall com.android.chrome --dry-run
```

## Troubleshooting

*   **"No device found"**: Ensure `adb devices` lists your phone. If strictly using TUI, you might need to kill other ADB server instances manually if frequent disconnects occur.
*   **Permission Denied**: On Linux/macOS, you might need `sudo` or configured dev rules for ADB access.
