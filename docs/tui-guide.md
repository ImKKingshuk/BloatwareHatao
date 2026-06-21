# BloatwareHatao TUI Guide

This guide explains how to use the interactive terminal UI in BloatwareHatao v1.0.0.

The TUI is the main product surface. Use it for package review, bloatware removal, presets, rescue points, device health checks, settings, and wireless ADB workflows. The CLI is mainly for quick checks such as `--device-info` and `--list-packages`.

## Before You Start

You need:

- Android platform-tools installed and `adb` available on `PATH`.
- An Android device with Developer Options enabled.
- USB debugging enabled on the device.
- The computer authorized from the device's USB debugging prompt.

Check the device outside the app first:

```bash
adb devices
```

You should see a device with `device` status. If it says `unauthorized`, unlock the phone and accept the debugging prompt. If it says `offline`, reconnect the device or restart the ADB server.

## Launching The TUI

Run:

```bash
bloatwarehatao
```

For a safer first pass, start in dry-run mode:

```bash
bloatwarehatao --dry-run
```

Dry-run mode lets you walk through package-changing actions without modifying the connected device.

If you built from source:

```bash
cargo run --bin bloatwarehatao
```

## Recommended First Workflow

1. Connect the phone with USB debugging enabled.
2. Start BloatwareHatao with `bloatwarehatao --dry-run`.
3. Open `Device Info` and confirm the expected phone is connected.
4. Open `Backup & Restore` and create a rescue point with `n`.
5. Open `Package Browser`.
6. Use search, safety filters, and category filters to narrow packages.
7. Select packages with `Space`.
8. Press `Enter` to open the action menu.
9. Review the action and confirm only after you understand the selected packages.
10. Disable packages first when unsure; uninstall only when you are confident.

## Global Navigation

| Key | Action |
|-----|--------|
| `Up`, `k` | Move up |
| `Down`, `j` | Move down |
| `Enter` | Select or confirm |
| `Esc` | Go back or close the active mode/dialog |
| `q` | Return to main menu or quit from home |
| `?` | Open the help overlay |
| `Ctrl+C` | Force quit |

Dialogs use these keys:

| Key | Action |
|-----|--------|
| `Left`, `h` | Move confirmation choice left |
| `Right`, `l` | Move confirmation choice right |
| `Enter` | Confirm selected dialog action |
| `y` | Confirm package operation |
| `n`, `Esc` | Cancel package operation |

## Main Menu

The home screen contains:

- `Package Browser`: browse, filter, select, and act on packages.
- `User Guide`: in-app quick guide.
- `Profiles`: built-in and custom preset workflows.
- `Device Info`: connected-device details.
- `Health Check`: battery, RAM, and storage metrics.
- `Backup & Restore`: rescue points and removal sessions.
- `Settings`: app behavior and UI preferences.
- `Wireless ADB`: enable and connect ADB over Wi-Fi.
- `Support/Sponsor`: project support link.
- `About`: app version, license, and project details.
- `Exit`: quit the app.

Use `Up`/`Down` or `j`/`k`, then press `Enter`.

## Package Browser

The Package Browser is where most work happens. It shows package rows on the left and package details on the right.

### Package Browser Keys

| Key | Action |
|-----|--------|
| `Up`, `k` | Move to previous package |
| `Down`, `j` | Move to next package |
| `PageUp` | Move up by 10 rows |
| `PageDown` | Move down by 10 rows |
| `Home`, `g` | Go to top |
| `End`, `G` | Go to bottom |
| `Space` | Select or unselect the highlighted package |
| `/` | Start search |
| `Enter` while searching | Apply search and leave search input |
| `Esc` while searching | Leave search input |
| `Backspace` while searching | Delete one search character |
| `c` | Clear search |
| `i` | Toggle installed-only filter |
| `Tab` | Next package-status tab |
| `Shift+Tab` | Previous package-status tab |
| `1` | Toggle Recommended safety filter |
| `2` | Toggle Advanced safety filter |
| `3` | Toggle Unsafe safety filter |
| `4` | Toggle Danger safety filter |
| `a` to `z` | Toggle the category shown for that letter in the filter panel |
| `0` | Clear safety and category filters |
| `r` | Reload packages |
| `Enter` | Open action menu for selected/current package |
| `u` | Quick uninstall selected packages |
| `d` | Quick disable selected packages |
| `e` | Quick enable selected packages |
| `Esc` | Clear current multi-selection, or return home if nothing is selected |
| `q` | Return home |

### Package Status Tabs

The browser has status tabs:

- `All`
- `Installed`
- `Disabled`
- `Uninstalled`

Use `Tab` and `Shift+Tab` to move between them. In v1.0.0, disabled tracking is still limited, so installed package review is the main workflow.

### Safety Filters

The safety filters help you avoid risky removals:

| Rating | Meaning |
|--------|---------|
| `User Installed` | App installed by the user. Usually safe to remove if you no longer need it. |
| `Recommended` | Usually safe to remove. Common bloatware with no known critical dependency. |
| `Advanced` | May affect some device features. Review before removing. |
| `Unsafe` | Can break functionality. Only use if you understand the package. |
| `Danger` | High-risk or critical component. Avoid removal unless you know exactly why. |

Use the details panel before acting on a package. Check the package name, label, description, category, safety rating, and any dependency/alternative metadata.

### Selecting Packages

Press `Space` to select the highlighted package. Selecting a package puts the browser into multi-select mode. Press `Esc` once to clear selection and return to single-selection mode.

When packages are selected:

- Press `Enter` to open the action menu.
- Press `u` for uninstall.
- Press `d` for disable.
- Press `e` for enable.

If no package is selected and the highlighted package is installed, pressing `Enter` selects that package and opens the action menu.

### Package Actions

Available package actions include:

- `Uninstall`: runs a current-user uninstall for the selected package.
- `Disable`: disables the selected package.
- `Enable`: re-enables a disabled package.
- `Reinstall`: attempts to restore an existing system package.
- `Clear Data`: clears app data for the package.

Every package operation asks for confirmation. In dry-run mode, the app simulates success without changing the device.

## Profiles

The `Profiles` screen manages package presets.

Built-in presets include:

- Privacy focused
- Performance boost
- Minimal setup
- No social media
- De-Google

### Profile Keys

| Key | Action |
|-----|--------|
| `Up`, `k` | Move to previous preset |
| `Down`, `j` | Move to next preset |
| `Enter` | Apply selected preset to installed packages |
| `n`, `+` | Start creating a custom preset |
| `e` | Export selected preset |
| `d` | Delete selected custom preset |
| `i` | Show import guidance |
| `r` | Reload presets |
| `Esc`, `q` | Return home |

Applying a preset selects matching installed packages in the Package Browser. You still need to review the selection and run an action from the browser.

Exported presets are written as JSON. Imported presets should be placed in the custom preset directory, then reloaded with `r`.

## Backup And Restore

The `Backup & Restore` screen manages rescue points and removal sessions.

Use it before making changes. A rescue point records the current installed package list so BloatwareHatao can later attempt to reinstall packages with Android's `cmd package install-existing --user 0` flow.

### Backup And Restore Keys

| Key | Action |
|-----|--------|
| `Tab` | Switch between rescue points and sessions |
| `Up`, `k` | Move up |
| `Down`, `j` | Move down |
| `n` | Create a new rescue point |
| `r` | Reload rescue history |
| `Enter` | Restore from selected rescue point/session |
| `Esc`, `q` | Return home |

Restores require a connected device unless you are in dry-run mode.

## Device Info

The `Device Info` screen shows the connected device identity and Android build data:

- Brand
- Model
- Manufacturer
- Android version
- SDK version
- Build ID
- Serial
- Security patch, when available
- Detected OEM

Keys:

| Key | Action |
|-----|--------|
| `r` | Refresh device information |
| `?` | Help overlay |
| `Esc`, `q` | Return home |

If no device is detected, confirm `adb devices` works and that the phone has authorized this computer.

## Health Check

The `Health Check` screen reads basic device metrics through ADB:

- Battery level
- Battery temperature
- RAM usage
- Storage usage

Keys:

| Key | Action |
|-----|--------|
| `r` | Refresh health metrics |
| `?` | Help overlay |
| `Esc`, `q` | Return home |

## Wireless ADB

The `Wireless ADB` screen helps move from USB ADB to Wi-Fi ADB.

Recommended flow:

1. Connect the device over USB.
2. Open `Wireless ADB`.
3. Press `p` to edit the port, or keep the default `5555`.
4. Press `e` to enable TCP/IP mode.
5. If the app detects the device IP, it fills the address field automatically.
6. Disconnect USB.
7. Press `i` to edit the address if needed, using `IP:PORT`.
8. Press `c` to connect over Wi-Fi.

### Wireless ADB Keys

| Key | Action |
|-----|--------|
| `p` | Edit TCP/IP port |
| `i` | Edit Wi-Fi address |
| `e` | Enable wireless ADB mode on connected USB device |
| `c` | Connect to the entered Wi-Fi address |
| `Enter` while editing | Save input |
| `Esc` while editing | Cancel input |
| `Backspace` while editing | Delete one character |
| `Esc`, `q` | Return home |

Wireless ADB requires the phone and computer to be on a reachable network.

## Settings

The `Settings` screen exposes app preferences. Navigate categories with `Up`/`Down` or `j`/`k`.

Available setting groups:

- `General`: dry-run mode, auto-update check, offline mode, verbose output.
- `Removal Options`: default removal behavior, safety warnings, confirmation, backup preference, safety rating preference.
- `Appearance`: theme, descriptions, progress indicators, animations.

Keys:

| Key | Action |
|-----|--------|
| `Up`, `k` | Move to previous settings category |
| `Down`, `j` | Move to next settings category |
| `Enter`, `Space` | Toggle the active setting for the selected category |
| `s` | Save settings |
| `?` | Help overlay |
| `Esc`, `q` | Return home |

Changed settings are not persisted until you press `s`.

## User Guide Screen

The in-app `User Guide` screen is a quick reference.

Keys:

| Key | Action |
|-----|--------|
| `Up`, `k` | Scroll up |
| `Down`, `j` | Scroll down |
| `PageUp` | Scroll up faster |
| `PageDown` | Scroll down faster |
| `Esc`, `q` | Return home |

## CLI Checks From Outside The TUI

For quick checks without launching the TUI:

```bash
bloatwarehatao --device-info
bloatwarehatao --list-packages
bloatwarehatao -s <SERIAL_NUMBER> --list-packages
bloatwarehatao --version
```

In v1.0.0, package-changing operations are not standalone CLI subcommands. Use the TUI for package removal, disable, restore, presets, and wireless ADB.

## Troubleshooting

### ADB Not Found

Install Android platform-tools and confirm:

```bash
adb version
```

Homebrew installs platform-tools automatically through the BloatwareHatao formula. Scoop users should install platform-tools separately.

### No Device Found

Run:

```bash
adb devices
```

If the list is empty:

- Reconnect the USB cable.
- Try a different USB port or cable.
- Make sure USB debugging is enabled.
- Restart ADB with `adb kill-server && adb start-server`.

### Device Unauthorized

Unlock the phone and accept the USB debugging prompt. If the prompt does not appear, revoke USB debugging authorizations from Developer Options, reconnect, and accept the new prompt.

### Permission Denied On Linux

You may need Android udev rules for your device vendor. After installing rules, reconnect the device and restart the ADB server.

### Package Database Does Not Load

Release archives include the `packages/` directory. If you manually move the binary, keep `packages/` near the binary or run from a directory where `packages/` is available.

### Operation Failed

Package operations can fail when:

- No device is connected.
- The package name is invalid.
- The package is protected by the OEM.
- The package is not installed for the current user.
- The device blocks the requested package manager command.

When unsure, use `Disable` before `Uninstall`, keep dry-run enabled for rehearsal, and create a rescue point first.

## Safe Usage Checklist

- Confirm the correct device in `Device Info`.
- Start with `--dry-run` for the first pass.
- Create a rescue point before bulk actions.
- Prefer `Recommended` packages for first cleanup.
- Read package details before selecting.
- Avoid `Danger` packages unless you know the device-specific impact.
- Remove packages in small batches.
- Reboot and test device features after each batch.
- Keep USB debugging disabled when you are done.
