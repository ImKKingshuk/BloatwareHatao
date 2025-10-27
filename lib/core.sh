#!/bin/bash

# Core Module for BloatwareHatao
# Contains core functionality: removal, backup, restore operations

# Global variables for dry run
DRY_RUN=false
BACKUP_FILE=""
VERBOSE_MODE=false

set_verbose_mode() {
    if [ "$1" = true ]; then
        VERBOSE_MODE=true
    else
        VERBOSE_MODE=false
    fi
}

# Set dry run mode
set_dry_run() {
    DRY_RUN=true
    show_info "DRY RUN MODE: No actual changes will be made"
}

set_verbose_mode() {
    if [ "$1" = true ]; then
        VERBOSE_MODE=true
    else
        VERBOSE_MODE=false
    fi
}

# Perform package removal
remove_package() {
    local package=$1
    local force=$2

    if ! validate_package_name "$package"; then
        show_error "Invalid package format: $package"
        return 1
    fi

    if [ "$DRY_RUN" = true ]; then
        show_info "Would remove: $package"
        record_operation REMOVE "dry-run|$package"
        return 0
    fi

    log_info "Attempting to remove package: $package"

    # Check if package exists
    if ! is_package_installed "$package"; then
        show_warning "Package $package is not installed"
        return 1
    fi

    # Check if system app and not forced
    if is_system_app "$package" && [ "$force" != "force" ]; then
        show_warning "Package $package appears to be a system app. Use force removal if needed."
        if ! confirm_action "Remove system app $package?"; then
            return 1
        fi
    fi

    # Create backup before removal if enabled
    if [ "$DEFAULT_BACKUP_BEFORE_REMOVE" = true ] && [ -z "$BACKUP_FILE" ]; then
        create_backup > /dev/null 2>&1 || show_warning "Backup step skipped due to earlier failure"
    fi

    # Perform removal
    local result
    result=$(adb shell pm uninstall --user 0 "$package" 2>&1)

    if [ $? -eq 0 ]; then
        show_success "Successfully removed: $package"
        log_info "Successfully removed: $package"
        record_operation REMOVE "success|$package"
        return 0
    else
        show_error "Failed to remove: $package"
        log_error "Failed to remove $package: $result"
        record_operation REMOVE "failed|$package|$result"
        return 1
    fi
}

# Batch remove packages
batch_remove() {
    local packages=("$@")
    local total=${#packages[@]}
    local success_count=0
    local fail_count=0

    if [ "$DRY_RUN" = true ]; then
        show_info "DRY RUN: Evaluating $total packages for removal"
    else
        show_info "Starting batch removal of $total packages..."
    fi

    for i in "${!packages[@]}"; do
        local package="${packages[$i]}"
        show_progress $((i + 1)) $total
        if [ "$VERBOSE_MODE" = true ]; then
            printf " - %s" "$package"
        fi

        if remove_package "$package"; then
            ((success_count++))
        else
            ((fail_count++))
        fi
    done

    echo

    if [ "$DRY_RUN" = true ]; then
        show_info "DRY RUN SUMMARY: ${success_count} packages would be removed, ${fail_count} skipped"
        log_info "Dry run batch removal summary: $success_count target, $fail_count skipped"
    else
        show_success "Batch removal completed: $success_count successful, $fail_count failed"
        log_info "Batch removal: $success_count success, $fail_count fail"
    fi
}

# Remove bloatware from remote script
remove_bloatware() {
    local manufacturer=$1
    local os_version=$2
    local cleaner_type=$3
    local script_path="https://raw.githubusercontent.com/ImKKingshuk/BloatwareHatao/main/$cleaner_type/$manufacturer/$os_version.sh"

    show_info "Fetching bloatware removal script for $manufacturer $os_version ($cleaner_type)..."

    # Download and execute script
    local script_content
    script_content=$(curl -sSL "$script_path")

    if [ $? -ne 0 ] || [ -z "$script_content" ]; then
        show_error "Failed to download script from $script_path"
        return 1
    fi

    # Extract package list from script
    local packages=()
    while IFS= read -r line; do
        # Parse lines like: "com.example.app"
        if [[ $line =~ ^[[:space:]]*\"([^\"]+)\" ]]; then
            packages+=("${BASH_REMATCH[1]}")
        fi
    done <<< "$script_content"

    if [ ${#packages[@]} -eq 0 ]; then
        show_error "No packages found in script"
        return 1
    fi

    show_info "Found ${#packages[@]} packages to remove"
    log_info "Starting removal of ${#packages[@]} packages for $manufacturer $os_version"
    record_operation RUN "removal|$manufacturer|$os_version|$cleaner_type|count=${#packages[@]}"

    batch_remove "${packages[@]}"
}

# Manual package removal
manual_remove() {
    local package=$1

    if ! validate_package_name "$package"; then
        show_error "Invalid package name format: $package"
        show_info "Package names should be in reverse domain notation (e.g., com.example.app)"
        return 1
    fi

    show_info "Package: $package"
    if is_package_installed "$package"; then
        show_info "Status: Installed"
        if is_system_app "$package"; then
            show_warning "This appears to be a system app"
        fi
    else
        show_warning "Package not found on device"
        return 1
    fi

    if confirm_action "Remove package $package?"; then
        remove_package "$package"
    fi
}

create_rescue_package_list() {
    ensure_directory "$DATA_DIR/rescue"
    local rescue_file="$DATA_DIR/rescue/rescue_$(date +%Y%m%d_%H%M%S).txt"
    log_info "Creating rescue list at $rescue_file"
    echo "# Packages removed during session $SESSION_ID" > "$rescue_file"
    grep "REMOVE|success" "$(get_report_path)" | awk -F'|' '{print $3}' >> "$rescue_file"
    record_operation RESCUE "created|$rescue_file"
    show_success "Rescue list saved to $rescue_file"
}

smart_removal_wizard() {
    show_info "Smart Removal Wizard"
    print_divider
    show_info "We'll guide you to safe removal tailored for your device."

    local default_cleaner="Safe"
    if confirm_action "Are you experienced with Android debloating?"; then
        default_cleaner="Pro"
        if confirm_action "Do you want the most aggressive cleanup?"; then
            default_cleaner="Ultra"
        fi
    fi

    show_info "Default cleaner suggestion: $default_cleaner"

    local manufacturer_options=()
    local manufacturer_count=$(get_manufacturer_count)
    for ((i=1; i<=manufacturer_count; i++)); do
        manufacturer_options+=("$(get_manufacturer_name $i)")
    done

    show_menu "Select Manufacturer" "${manufacturer_options[@]}"
    local manufacturer_choice=$(get_choice ${#manufacturer_options[@]})
    local manufacturer=$(get_manufacturer_name $manufacturer_choice)

    local os_raw_options=$(get_os_versions "$manufacturer")
    local os_display_options=()
    local os_slug_options=()
    while IFS='|' read -r display slug; do
        [ -z "$display" ] && continue
        os_display_options+=("$display")
        os_slug_options+=("$slug")
    done <<EOF
$os_raw_options
EOF

    show_menu "Select OS Version" "${os_display_options[@]}"
    local os_choice=$(get_choice ${#os_display_options[@]})
    local os_display="${os_display_options[$((os_choice-1))]}"
    local os_version="${os_slug_options[$((os_choice-1))]}"

    show_info "Gathering package list..."
    if ! preview_bloatware "$manufacturer" "$os_version" "$default_cleaner"; then
        show_warning "Audit failed. You can still attempt removal manually."
        return
    fi

    if confirm_action "Proceed with $default_cleaner removal for $manufacturer $os_display?"; then
        remove_bloatware "$manufacturer" "$os_version" "$default_cleaner"
    fi

    if confirm_action "Would you like a rescue list of removed packages?"; then
        create_rescue_package_list
    fi
}

preview_bloatware() {
    local manufacturer=$1
    local os_version=$2
    local cleaner_type=$3
    local script_path="https://raw.githubusercontent.com/ImKKingshuk/BloatwareHatao/main/$cleaner_type/$manufacturer/$os_version.sh"

    show_info "Preparing smart audit for $manufacturer $os_version ($cleaner_type)..."

    local script_content
    script_content=$(curl -sSL "$script_path")

    if [ $? -ne 0 ] || [ -z "$script_content" ]; then
        show_error "Failed to fetch script for audit: $script_path"
        record_operation AUDIT "failed|$manufacturer|$os_version|$cleaner_type"
        return 1
    fi

    local packages=()
    while IFS= read -r line; do
        if [[ $line =~ ^[[:space:]]*"([^"]+)" ]]; then
            packages+=("${BASH_REMATCH[1]}")
        fi
    done <<< "$script_content"

    if [ ${#packages[@]} -eq 0 ]; then
        show_error "No packages found in script for audit"
        record_operation AUDIT "empty|$manufacturer|$os_version|$cleaner_type"
        return 1
    fi

    local installed=()
    local missing=()
    local system_flags=()

    local package
    for package in "${packages[@]}"; do
        if is_package_installed "$package"; then
            installed+=("$package")
            if is_system_app "$package"; then
                system_flags+=("$package")
            fi
        else
            missing+=("$package")
        fi
    done

    local installed_count=${#installed[@]}
    local missing_count=${#missing[@]}
    local system_count=${#system_flags[@]}

    print_divider
    show_info "Audit Summary"
    echo "Packages in script: ${#packages[@]}"
    echo "Installed on device: $installed_count"
    echo "Potential system apps: $system_count"
    echo "Not currently installed: $missing_count"
    print_divider

    if [ "$VERBOSE_MODE" = true ] || [ $installed_count -le 20 ]; then
        show_info "Installed packages that would be removed:"
        for package in "${installed[@]}"; do
            echo "  - $package"
        done
    else
        show_info "Installed packages sample (enable verbose mode to list all):"
        local limit=0
        for package in "${installed[@]}"; do
            echo "  - $package"
            limit=$((limit + 1))
            if [ $limit -ge 10 ]; then
                echo "  ... (${installed_count} total installed packages)"
                break
            fi
        done
    fi

    if [ $system_count -gt 0 ]; then
        print_divider
        show_warning "Packages flagged as system components (review carefully):"
        local limit=0
        for package in "${system_flags[@]}"; do
            echo "  - $package"
            limit=$((limit + 1))
            if [ $limit -ge 10 ] && [ "$VERBOSE_MODE" != true ]; then
                echo "  ... (${system_count} total flagged)"
                break
            fi
        done
    fi

    print_divider
    show_info "Tip: Use Dry Run mode if you want to test the removal safely."
    record_operation AUDIT "success|$manufacturer|$os_version|$cleaner_type|installed=$installed_count|system=$system_count|missing=$missing_count"
    return 0
}

# Restore from backup
restore_from_backup() {
    local backup_file=$1

    if [ ! -f "$backup_file" ]; then
        show_error "Backup file not found: $backup_file"
        return 1
    fi

    show_info "Restoring from backup: $backup_file"

    local packages=()
    while IFS= read -r line; do
        if [[ $line =~ ^package:(.+)$ ]]; then
            packages+=("${BASH_REMATCH[1]}")
        elif [[ $line =~ ^([[:alnum:]._-]+)$ ]]; then
            packages+=("${BASH_REMATCH[1]}")
        fi
    done < "$backup_file"

    show_info "Found ${#packages[@]} packages in backup"

    if confirm_action "Restore ${#packages[@]} packages?"; then
        show_info "Note: This will attempt to reinstall packages. Success depends on availability in Play Store or APK sources."
        log_info "Starting restore from $backup_file"

        local success_count=0
        local total=${#packages[@]}

        for i in "${!packages[@]}"; do
            package="${packages[$i]}"
            show_progress $((i+1)) $total
            printf " - %s" "$package"

            if [ "$DRY_RUN" = true ]; then
                show_info "DRY RUN: Would attempt to restore $package"
                ((success_count++))
                continue
            fi

            # Try to install via Play Store intent (best effort)
            if adb shell am start -a android.intent.action.VIEW -d "market://details?id=$package" > /dev/null 2>&1; then
                ((success_count++))
                show_success "Restore initiated: $package"
            else
                show_warning "Could not initiate restore: $package"
            fi
        done

        echo
        if [ "$DRY_RUN" = true ]; then
            show_info "DRY RUN: Restore simulation complete. No changes made."
            log_info "Dry run restore simulation for $backup_file"
        else
            show_info "Restore process completed. Check device for installation prompts."
            log_info "Restore completed: $success_count packages initiated"
        fi
    fi
}

# Create custom removal script
create_custom_script() {
    local script_name=$1
    local packages=("$@")
    unset packages[0] # Remove script name

    mkdir -p data/custom
    local script_path="data/custom/$script_name.sh"

    cat > "$script_path" << EOF
#!/bin/bash

# Custom bloatware removal script: $script_name
# Generated by BloatwareHatao v2.0.0

declare -a bloatware=(
$(printf '    "%s"\n' "${packages[@]}")
)

for package in "\${bloatware[@]}"; do
    echo "Removing: \$package"
    adb shell pm uninstall --user 0 "\$package"
done

echo "Custom removal script completed"
EOF

    chmod +x "$script_path"
    show_success "Custom script created: $script_path"
    log_info "Custom script created: $script_path"
}

# Load custom package list from file
load_package_list() {
    local file_path=$1
    local packages=()

    if [ ! -f "$file_path" ]; then
        show_error "Package list file not found: $file_path"
        return 1
    fi

    while IFS= read -r line; do
        # Skip comments and empty lines
        [[ $line =~ ^[[:space:]]*# ]] && continue
        [[ -z "$line" ]] && continue
        packages+=("$line")
    done < "$file_path"

    echo "${packages[@]}"
}

# Show removal statistics
show_stats() {
    show_info "BloatwareHatao Statistics"

    # Count total operations from logs
    local total_removed=$(grep "Successfully removed:" "$LOG_FILE" 2>/dev/null | wc -l)
    local total_failed=$(grep "Failed to remove" "$LOG_FILE" 2>/dev/null | wc -l)
    local total_backups=$(ls -1 "$BACKUP_DIR" 2>/dev/null | wc -l)

    echo "Total packages removed: $total_removed"
    echo "Total removal failures: $total_failed"
    echo "Total backups created: $total_backups"

    if [ -d "$BACKUP_DIR" ]; then
        echo "Available backups:"
        ls -la "$BACKUP_DIR"
    fi
}
