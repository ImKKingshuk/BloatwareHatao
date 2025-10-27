#!/bin/bash

# Report helper
log_report() {
    local entry="$1"
    ensure_directory "$REPORT_DIR"
    if [ -z "$REPORT_FILE" ]; then
        REPORT_FILE="$REPORT_DIR/report_${SESSION_ID}.txt"
        touch "$REPORT_FILE"
    fi
    echo "$(date '+%Y-%m-%d %H:%M:%S')|$entry" >> "$REPORT_FILE"
}

get_report_path() {
    if [ -n "$REPORT_FILE" ]; then
        echo "$REPORT_FILE"
    fi
}

# Utils Module for BloatwareHatao
# Contains utility functions: logging, validation, device detection, etc.

# Global variables
LOG_FILE="logs/bloatwarehatao.log"
BACKUP_DIR="backups"
CONFIG_FILE="config/bloatwarehatao.conf"
DATA_DIR="data"
REPORT_DIR="$DATA_DIR/reports"
PLANS_DIR="$DATA_DIR/plans"
LAST_BACKUP_FILE=""
ADB_TARGET=""
SESSION_ID="$(date +%Y%m%d_%H%M%S)"
REPORT_FILE=""
SESSION_FINALIZED=false

# Logging functions
log_info() {
    local message="$1"
    echo "$(date '+%Y-%m-%d %H:%M:%S') [INFO] $message" >> "$LOG_FILE"
}

log_error() {
    local message="$1"
    echo "$(date '+%Y-%m-%d %H:%M:%S') [ERROR] $message" >> "$LOG_FILE"
}

log_warning() {
    local message="$1"
    echo "$(date '+%Y-%m-%d %H:%M:%S') [WARNING] $message" >> "$LOG_FILE"
}

# Initialize logging
init_logging() {
    ensure_directory logs
    ensure_directory "$DATA_DIR"
    ensure_directory "$REPORT_DIR"
    touch "$LOG_FILE"
    REPORT_FILE="$REPORT_DIR/report_${SESSION_ID}.txt"
    touch "$REPORT_FILE"
    log_info "BloatwareHatao session started"
    log_report "SESSION|START|$SESSION_ID"
}

# Directory helper
ensure_directory() {
    local dir="$1"
    if [ ! -d "$dir" ]; then
        mkdir -p "$dir"
    fi
}

# Report helper wrappers
record_operation() {
    local type="$1"
    shift
    log_report "${type}|$*"
}

finalize_session() {
    if [ "$SESSION_FINALIZED" = true ]; then
        return
    fi
    record_operation SESSION-END "$SESSION_ID"
    log_info "BloatwareHatao session ended"
    SESSION_FINALIZED=true
}

finalize_and_exit() {
    local code=${1:-0}
    finalize_session
    exit "$code"
}

# Validate dependencies
validate_dependencies() {
    local missing_deps=()

    if ! command -v adb &> /dev/null; then
        missing_deps+=("adb (Android Debug Bridge)")
    fi

    if ! command -v curl &> /dev/null; then
        missing_deps+=("curl")
    fi

    if [ ${#missing_deps[@]} -ne 0 ]; then
        show_error "Missing dependencies:"
        for dep in "${missing_deps[@]}"; do
            echo "  - $dep"
        done
        echo
        show_info "Please install the missing dependencies and try again."
        show_info "ADB download: https://developer.android.com/tools/releases/platform-tools"
        exit 1
    fi
}

# Check if device is connected
check_device_connected() {
    local devices=()
    local problematic=()

    while read -r serial status; do
        [ -z "$serial" ] && continue
        [ "$serial" = "List" ] && continue
        case $status in
            device)
                devices+=("$serial")
                ;;
            unauthorized|offline)
                problematic+=("$serial:$status")
                ;;
        esac
    done <<EOF
$(adb devices)
EOF

    if [ ${#devices[@]} -eq 0 ]; then
        if [ ${#problematic[@]} -gt 0 ]; then
            show_warning "Detected devices needing attention:"
            local entry
            for entry in "${problematic[@]}"; do
                echo "  - ${entry%%:*} (${entry##*:})"
            done
            echo
            show_info "Authorize the device on your phone screen and ensure USB debugging is enabled."
        fi
        show_error "No ready Android device detected. Please ensure:"
        echo "  1. Android device is connected via USB"
        echo "  2. USB debugging is enabled in Developer Options"
        echo "  3. The device is authorized for this computer"
        return 1
    fi

    if [ ${#devices[@]} -eq 1 ]; then
        ADB_TARGET="${devices[0]}"
    else
        show_info "Multiple devices detected. Please select the target device."
        local index=1
        for serial in "${devices[@]}"; do
            echo "  $index) $serial"
            index=$((index + 1))
        done
        local selection
        while true; do
            read -p "Choose device (1-${#devices[@]}): " selection
            if [[ $selection =~ ^[0-9]+$ ]] && [ "$selection" -ge 1 ] && [ "$selection" -le ${#devices[@]} ]; then
                break
            fi
            echo "Please enter a number between 1 and ${#devices[@]}."
        done
        ADB_TARGET="${devices[$((selection-1))]}"
    fi

    export ANDROID_SERIAL="$ADB_TARGET"
    log_info "Using Android device: $ADB_TARGET"
    record_operation DEVICE "$ADB_TARGET"
    show_info "Connected device: $ADB_TARGET"
    return 0
}

# Get device information
get_device_info() {
    local info=""
    info+="Device Model: $(adb shell getprop ro.product.model)\n"
    info+="Android Version: $(adb shell getprop ro.build.version.release)\n"
    info+="Build Number: $(adb shell getprop ro.build.display.id)\n"
    info+="Manufacturer: $(adb shell getprop ro.product.manufacturer)\n"
    info+="Brand: $(adb shell getprop ro.product.brand)\n"
    info+="Serial: $(adb devices | grep device | head -1 | awk '{print $1}')\n"
    echo -e "$info"
}

# Validate package name
validate_package_name() {
    local package=$1
    # Basic package name validation (reverse domain notation)
    if [[ $package =~ ^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$ ]]; then
        return 0
    else
        return 1
    fi
}

# Check if package is installed
is_package_installed() {
    local package=$1
    if adb shell pm list packages | grep -q "^package:$package$"; then
        return 0
    else
        return 1
    fi
}

# Check if package is system app
is_system_app() {
    local package=$1
    local path=$(adb shell pm path "$package" 2>/dev/null | head -1 | cut -d: -f2)
    if [[ $path == /system/* ]] || [[ $path == /product/* ]] || [[ $path == /vendor/* ]]; then
        return 0
    else
        return 1
    fi
}

# Create backup of package list
create_backup() {
    local timestamp=$(date +%Y%m%d_%H%M%S)
    local backup_name="backup_${timestamp}.txt"
    LAST_BACKUP_FILE="$BACKUP_DIR/$backup_name"

    if [ "${DRY_RUN:-false}" = true ]; then
        show_info "DRY RUN: Would create backup: $backup_name"
        log_info "Dry run backup simulated at $LAST_BACKUP_FILE"
        return 0
    fi

    mkdir -p "$BACKUP_DIR"

    if adb shell pm list packages -3 > "$LAST_BACKUP_FILE"; then
        log_info "Backup created: $LAST_BACKUP_FILE"
        record_operation BACKUP "success|$LAST_BACKUP_FILE"
        show_success "Backup created: $backup_name"
        echo "$LAST_BACKUP_FILE"
        return 0
    else
        show_error "Failed to create backup."
        log_error "Failed to create backup at $LAST_BACKUP_FILE"
        record_operation BACKUP "failed|$LAST_BACKUP_FILE"
        return 1
    fi
}

# Load configuration
load_config() {
    if [ -f "$CONFIG_FILE" ]; then
        source "$CONFIG_FILE"
    else
        # Default configuration
        DEFAULT_DRY_RUN=false
        DEFAULT_BACKUP_BEFORE_REMOVE=true
        DEFAULT_VERBOSE=true
        DEFAULT_AUTO_UPDATE=true
    fi
}

# Save configuration
save_config() {
    mkdir -p config
    cat > "$CONFIG_FILE" << EOF
# BloatwareHatao Configuration
DEFAULT_DRY_RUN=$DEFAULT_DRY_RUN
DEFAULT_BACKUP_BEFORE_REMOVE=$DEFAULT_BACKUP_BEFORE_REMOVE
DEFAULT_VERBOSE=$DEFAULT_VERBOSE
DEFAULT_AUTO_UPDATE=$DEFAULT_AUTO_UPDATE
EOF
}

# Device health insight
get_device_health_report() {
    local battery_level=$(adb shell dumpsys battery 2>/dev/null | grep -m1 "level" | awk '{print $2}')
    local battery_status=$(adb shell dumpsys battery 2>/dev/null | grep -m1 "status" | awk '{print $2}')
    local temperature=$(adb shell dumpsys battery 2>/dev/null | grep -m1 "temperature" | awk '{print $2}')
    local power_source=$(adb shell dumpsys battery 2>/dev/null | grep -m1 "AC powered" | awk -F': ' '{print $2}')

    local storage_line=$(adb shell df /data 2>/dev/null | tail -1)
    local storage_total=$(echo "$storage_line" | awk '{print $2}')
    local storage_used=$(echo "$storage_line" | awk '{print $3}')
    local storage_free=$(echo "$storage_line" | awk '{print $4}')
    local storage_percent=$(echo "$storage_line" | awk '{print $5}')

    local uptime=$(adb shell uptime 2>/dev/null)
    local wifi_state=$(adb shell dumpsys wifi 2>/dev/null | grep -m1 "Wi-Fi is" | sed 's/^\s*//')

    echo "Battery Level: ${battery_level}%"
    echo "Battery Status: ${battery_status}"
    if [ -n "$temperature" ]; then
        echo "Battery Temp: ${temperature} (tenths °C)"
    fi
    if [ -n "$power_source" ]; then
        echo "Charging (AC powered): ${power_source}"
    fi
    echo "Data Storage Used: $storage_used / $storage_total ($storage_percent)"
    echo "Data Storage Free: $storage_free"
    echo "Device Uptime: $uptime"
    if [ -n "$wifi_state" ]; then
        echo "$wifi_state"
    fi
}

# Check for updates
check_for_updates() {
    local current_version="2.0.0"
    local latest_version=$(curl -sSL "https://raw.githubusercontent.com/ImKKingshuk/BloatwareHatao/main/version.txt" 2>/dev/null || echo "$current_version")

    if [ "$latest_version" != "$current_version" ]; then
        show_warning "A new version ($latest_version) is available. Current: $current_version"
        if confirm_action "Would you like to update?"; then
            update_tool
        fi
    else
        log_info "Running latest version: $current_version"
    fi
}

# Update tool
update_tool() {
    show_info "Updating tool..."
    local repo_url="https://raw.githubusercontent.com/ImKKingshuk/BloatwareHatao/main"

    # Update main files
    curl -sSL "$repo_url/BloatwareHatao.sh" -o BloatwareHatao_new.sh
    curl -sSL "$repo_url/version.txt" -o version_new.txt

    if [ $? -eq 0 ]; then
        mv BloatwareHatao_new.sh BloatwareHatao.sh
        mv version_new.txt version.txt
        chmod +x BloatwareHatao.sh
        show_success "Tool updated successfully. Please restart."
        log_info "Tool updated to latest version"
        exit 0
    else
        show_error "Update failed."
    fi
}

# Clean up temp files
cleanup() {
    # Remove any temp files if needed
    log_info "Cleanup completed"
}

# Error handler
error_handler() {
    local error_code=$?
    log_error "Script exited with error code: $error_code"
    show_error "An error occurred. Check logs for details."
    cleanup
    exit $error_code
}

# Set up error handling
setup_error_handling() {
    trap error_handler ERR
    set -e
}
