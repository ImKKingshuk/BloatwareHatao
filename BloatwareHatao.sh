#!/bin/bash

# BloatwareHatao v2.0.0 - Ultimate Android Bloatware Removal Tool
# Main script that orchestrates all modules

# Set script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Source modules
source "$SCRIPT_DIR/lib/ui.sh"
source "$SCRIPT_DIR/lib/utils.sh"
source "$SCRIPT_DIR/lib/core.sh"
source "$SCRIPT_DIR/lib/data.sh"

# Main function
main() {
    # Initialize
    init_logging
    load_config
    set_verbose_mode "$DEFAULT_VERBOSE"
    # Initialize removal mode from config
    set_remove_mode "${DEFAULT_REMOVE_MODE:-uninstall}"
    # Initialize offline mode from config
    set_offline_mode "${DEFAULT_OFFLINE_MODE:-false}"
    setup_error_handling

    # Parse command line arguments
    parse_args "$@"

    # Show banner
    print_banner

    # Validate dependencies
    validate_dependencies

    # Check for updates if enabled
    if [ "$DEFAULT_AUTO_UPDATE" = true ]; then
        check_for_updates
    fi

    # Check device connection
    if ! check_device_connected; then
        show_error "No device connected. Please connect your Android device and try again."
        exit 1
    fi

    # Main menu
    show_main_menu
}

show_pre_removal_audit_menu() {
    show_info "Pre-removal Audit"
    print_divider
    show_info "Review package list before removing anything."

    local cleaner_options=("Safe" "Pro" "Ultra")
    show_menu "Select Cleaner Type" "${cleaner_options[@]}"
    local cleaner_choice=$(get_choice ${#cleaner_options[@]})
    local cleaner_type="${cleaner_options[$((cleaner_choice-1))]}"

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

    if preview_bloatware "$manufacturer" "$os_version" "$cleaner_type"; then
        show_info "Audit complete for $manufacturer $os_display ($cleaner_type)."
    fi
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                show_help
                exit 0
                ;;
            --version|-v)
                echo "BloatwareHatao v2.0.0"
                exit 0
                ;;
            --dry-run)
                set_dry_run
                shift
                ;;
            --offline)
                set_offline_mode true
                shift
                ;;
            --mode)
                # Set removal mode: uninstall|disable|clear
                shift
                local mode_arg="$1"
                if [ -z "$mode_arg" ]; then
                    show_error "--mode requires a value: uninstall|disable|clear"
                    show_help
                    exit 1
                fi
                set_remove_mode "$mode_arg"
                shift
                ;;
            --device-info)
                show_device_info
                exit 0
                ;;
            --smart-wizard)
                smart_removal_wizard
                finalize_and_exit 0
                ;;
            --audit)
                shift
                run_audit_cli "$@"
                finalize_and_exit 0
                ;;
            --health)
                show_device_health_snapshot
                finalize_and_exit 0
                ;;
            --planner)
                show_cleaning_planner
                finalize_and_exit 0
                ;;
            --report)
                display_session_report
                finalize_and_exit 0
                ;;
            --report-ndjson)
                display_session_report_ndjson
                finalize_and_exit 0
                ;;
            --backup)
                create_backup
                exit 0
                ;;
            --restore)
                show_restore_menu
                exit 0
                ;;
            --log)
                show_logs
                exit 0
                ;;
            --stats)
                show_stats
                exit 0
                ;;
            *)
                show_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Show device information
show_device_info() {
    show_info "Connected Device Information:"
    echo "=============================="
    get_device_info
}

# Show logs
show_logs() {
    if [ -f "$LOG_FILE" ]; then
        show_info "Recent Logs:"
        echo "=============="
        tail -20 "$LOG_FILE"
    else
        show_info "No logs found"
    fi
}

# Main menu
show_main_menu() {
    local options=(
        "Standard Bloatware Removal (Interactive)"
        "Smart Removal Wizard"
        "Manual Package Removal"
        "Pre-removal Audit"
        "Batch Removal from File"
        "Create Custom Removal Script"
        "Backup & Restore"
        "Device Information"
        "Device Health Snapshot"
        "Cleaning Planner"
        "View Logs & Statistics"
        "Settings & Configuration"
        "Exit"
    )

    while true; do
        show_menu "BloatwareHatao Main Menu" "${options[@]}"
        local choice=$(get_choice ${#options[@]})

        case $choice in
            1) show_standard_removal_menu ;;
            2) smart_removal_wizard ;;
            3) show_manual_removal_menu ;;
            4) show_pre_removal_audit_menu ;;
            5) show_batch_removal_menu ;;
            6) show_custom_script_menu ;;
            7) show_backup_restore_menu ;;
            8) show_device_info ;;
            9) show_device_health_snapshot ;;
            10) show_cleaning_planner ;;
            11) show_logs_stats_menu ;;
            12) show_settings_menu ;;
            13) show_info "Goodbye!"; finalize_and_exit 0 ;;
        esac

        echo
        read -p "Press Enter to continue..."
    done
}

# Standard removal menu (manufacturer selection)
show_standard_removal_menu() {
    show_info "Standard Bloatware Removal"

    # Select cleaner type
    local cleaner_options=("Safe & Recommended" "Pro (Extra Cleaning)" "Ultra (Extreme Cleaning)")
    show_menu "Select Cleaner Type" "${cleaner_options[@]}"
    local cleaner_choice=$(get_choice ${#cleaner_options[@]})
    local cleaner_type=$(cleaner_display_to_dir "${cleaner_options[$((cleaner_choice-1))]}")

    # Select manufacturer
    local manufacturer_options=()
    local manufacturer_count=$(get_manufacturer_count)
    for ((i=1; i<=manufacturer_count; i++)); do
        manufacturer_options+=("$(get_manufacturer_name $i)")
    done
    show_menu "Select Manufacturer" "${manufacturer_options[@]}"
    local manufacturer_choice=$(get_choice ${#manufacturer_options[@]})
    local manufacturer=$(get_manufacturer_name $manufacturer_choice)

    # Select OS version
    local os_raw_options=$(get_os_versions "$manufacturer")
    local os_display_options=()
    local os_slug_options=()
    local idx=0

    while IFS='|' read -r display slug; do
        [ -z "$display" ] && continue
        os_display_options+=("$display")
        os_slug_options+=("$slug")
        idx=$((idx + 1))
    done <<EOF
$os_raw_options
EOF

    if [ ${#os_display_options[@]} -eq 0 ]; then
        show_error "No OS options available for $manufacturer"
        return
    fi

    show_menu "Select OS Version for $manufacturer" "${os_display_options[@]}"
    local os_choice=$(get_choice ${#os_display_options[@]})
    local os_display="${os_display_options[$((os_choice-1))]}"
    local os_version="${os_slug_options[$((os_choice-1))]}"

    show_info "Selected: $manufacturer $os_display ($cleaner_type)"

    if confirm_action "Proceed with removal?"; then
        if [ "$DRY_RUN" = true ]; then
            show_dry_run_notice "No packages will be uninstalled"
        fi

        # Load packages and optionally let user pick
        local packages=()
        read -r -a packages <<< "$(get_bloatware_packages "$manufacturer" "$os_version" "$cleaner_type")"

        if [ ${#packages[@]} -eq 0 ]; then
            show_error "No packages found for $manufacturer $os_display ($cleaner_type)"
            return
        fi

        local selected=()
        if confirm_action "Review and select packages before removal?"; then
            read -r -a selected <<< "$(interactive_select_packages "${packages[@]}")"
        else
            selected=("${packages[@]}")
        fi

        if [ ${#selected[@]} -eq 0 ]; then
            show_warning "No packages selected. Aborting."
            return
        fi

        batch_remove "${selected[@]}"
    fi
}

# Manual removal menu
show_manual_removal_menu() {
    show_info "Manual Package Removal"
    echo "Enter package name (e.g., com.google.android.youtube)"
    echo "Leave empty to cancel"
    echo

    while true; do
        read -p "Package name: " package
        if [ -z "$package" ]; then
            break
        fi
        manual_remove "$package"
        echo
    done
}

# Batch removal menu
show_batch_removal_menu() {
    show_info "Batch Removal from File"
    echo "Enter path to file containing package names (one per line)"
    echo "Comments starting with # are ignored"
    echo

    read -p "File path: " file_path
    if [ -f "$file_path" ]; then
        local packages=($(load_package_list "$file_path"))
        if [ ${#packages[@]} -gt 0 ]; then
            show_info "Found ${#packages[@]} packages in $file_path"
            local selected=()
            if confirm_action "Review and select packages before removal?"; then
                read -r -a selected <<< "$(interactive_select_packages "${packages[@]}")"
            else
                selected=("${packages[@]}")
            fi

            if [ ${#selected[@]} -eq 0 ]; then
                show_warning "No packages selected. Aborting."
                return
            fi

            if confirm_action "Remove selected packages?"; then
                batch_remove "${selected[@]}"
            fi
        else
            show_error "No valid packages found in file"
        fi
    else
        show_error "File not found: $file_path"
    fi
}

# Custom script menu
show_custom_script_menu() {
    show_info "Create Custom Removal Script"
    read -p "Script name: " script_name
    if [ -z "$script_name" ]; then
        return
    fi

    echo "Enter package names (one per line, empty line to finish):"
    local packages=()
    while true; do
        read -p "Package: " package
        if [ -z "$package" ]; then
            break
        fi
        if validate_package_name "$package"; then
            packages+=("$package")
        else
            show_warning "Invalid package name: $package"
        fi
    done

    if [ ${#packages[@]} -gt 0 ]; then
        create_custom_script "$script_name" "${packages[@]}"
    else
        show_warning "No packages specified"
    fi
}

# Backup and restore menu
show_backup_restore_menu() {
    local options=("Create Backup" "Restore from Backup" "List Backups" "Back to Main Menu")
    show_menu "Backup & Restore" "${options[@]}"
    local choice=$(get_choice ${#options[@]})

    case $choice in
        1) create_backup ;;
        2) show_restore_menu ;;
        3) list_backups ;;
        4) return ;;
    esac
}

# Show restore menu
show_restore_menu() {
    show_info "Available Backups:"
    if [ -d "$BACKUP_DIR" ]; then
        local backups=($(ls -t "$BACKUP_DIR"/*.txt 2>/dev/null))
        if [ ${#backups[@]} -gt 0 ]; then
            for i in "${!backups[@]}"; do
                echo "$((i+1)). ${backups[$i]##*/}"
            done
            echo
            read -p "Select backup number (or 0 to cancel): " choice
            if [ "$choice" -gt 0 ] && [ "$choice" -le ${#backups[@]} ]; then
                restore_from_backup "${backups[$((choice-1))]}"
            fi
        else
            show_info "No backups found"
        fi
    else
        show_info "No backups directory found"
    fi
}

# List backups
list_backups() {
    if [ -d "$BACKUP_DIR" ]; then
        show_info "Available Backups:"
        ls -la "$BACKUP_DIR"
    else
        show_info "No backups found"
    fi
}

# Logs and stats menu
show_logs_stats_menu() {
    local options=(
        "View Recent Logs"
        "Show Statistics"
        "View Session Report"
        "List Rescue Lists"
        "Restore from Rescue List"
        "Clear Logs"
        "Back to Main Menu"
    )
    show_menu "Logs & Statistics" "${options[@]}"
    local choice=$(get_choice ${#options[@]})

    case $choice in
        1) show_logs ;;
        2) show_stats ;;
        3) display_session_report ;;
        4) list_rescue_lists ;;
        5) restore_from_rescue_menu ;;
        6) clear_logs ;;
        7) return ;;
    esac
}

# Clear logs
clear_logs() {
    if confirm_action "Clear all logs?"; then
        > "$LOG_FILE"
        show_success "Logs cleared"
    fi
}

# Settings menu
show_settings_menu() {
    show_info "Settings & Configuration"

    echo "Current settings:"
    echo "  Dry run mode: $DEFAULT_DRY_RUN"
    echo "  Auto backup: $DEFAULT_BACKUP_BEFORE_REMOVE"
    echo "  Verbose output: $DEFAULT_VERBOSE"
    echo "  Auto update: $DEFAULT_AUTO_UPDATE"
    echo "  Offline mode: $DEFAULT_OFFLINE_MODE"
    echo "  Remove mode: $DEFAULT_REMOVE_MODE"
    echo "  NDJSON reports: $DEFAULT_REPORT_NDJSON"
    echo

    local options=(
        "Toggle Dry Run"
        "Toggle Auto Backup"
        "Toggle Verbose"
        "Toggle Auto Update"
        "Toggle Offline Mode"
        "Cycle Default Remove Mode"
        "Toggle NDJSON Reports"
        "Save Settings"
        "Back to Main Menu"
    )
    show_menu "Settings Menu" "${options[@]}"
    local choice=$(get_choice ${#options[@]})

    case $choice in
        1) DEFAULT_DRY_RUN=$([ "$DEFAULT_DRY_RUN" = true ] && echo false || echo true) ;;
        2) DEFAULT_BACKUP_BEFORE_REMOVE=$([ "$DEFAULT_BACKUP_BEFORE_REMOVE" = true ] && echo false || echo true) ;;
        3) DEFAULT_VERBOSE=$([ "$DEFAULT_VERBOSE" = true ] && echo false || echo true); set_verbose_mode "$DEFAULT_VERBOSE" ;;
        4) DEFAULT_AUTO_UPDATE=$([ "$DEFAULT_AUTO_UPDATE" = true ] && echo false || echo true) ;;
        5) DEFAULT_OFFLINE_MODE=$([ "$DEFAULT_OFFLINE_MODE" = true ] && echo false || echo true); set_offline_mode "$DEFAULT_OFFLINE_MODE" ;;
        6) 
            case "$DEFAULT_REMOVE_MODE" in
                uninstall) DEFAULT_REMOVE_MODE=disable ;;
                disable) DEFAULT_REMOVE_MODE=clear ;;
                *) DEFAULT_REMOVE_MODE=uninstall ;;
            esac
            set_remove_mode "$DEFAULT_REMOVE_MODE"
            ;;
        7) DEFAULT_REPORT_NDJSON=$([ "$DEFAULT_REPORT_NDJSON" = true ] && echo false || echo true) ;;
        8) save_config; show_success "Settings saved" ;;
        9) return ;;
    esac

    # Show updated settings
    show_settings_menu
}

display_session_report() {
    local report_path=$(get_report_path)
    if [ -n "$report_path" ] && [ -f "$report_path" ]; then
        show_info "Session Report: $report_path"
        cat "$report_path"
    else
        show_info "No session report available yet."
    fi
}

display_session_report_ndjson() {
    local ndjson_path=$(get_report_ndjson_path)
    if [ -n "$ndjson_path" ] && [ -f "$ndjson_path" ]; then
        show_info "NDJSON Session Report: $ndjson_path"
        cat "$ndjson_path"
    else
        show_info "No NDJSON session report available. Enable it in Settings."
    fi
}

list_rescue_lists() {
    local rescue_dir="$DATA_DIR/rescue"
    if [ -d "$rescue_dir" ]; then
        show_info "Available rescue lists:"
        ls -1 "$rescue_dir"
    else
        show_info "No rescue lists found yet."
    fi
}

# Restore from a rescue list interactively
restore_from_rescue_menu() {
    local rescue_dir="$DATA_DIR/rescue"
    if [ ! -d "$rescue_dir" ]; then
        show_info "No rescue lists directory found."
        return
    fi

    local files=($(ls -t "$rescue_dir"/*.txt 2>/dev/null))
    if [ ${#files[@]} -eq 0 ]; then
        show_info "No rescue lists found."
        return
    fi

    show_info "Rescue Lists:"
    for i in "${!files[@]}"; do
        echo "$((i+1)). ${files[$i]##*/}"
    done
    echo
    read -p "Select rescue number (or 0 to cancel): " choice
    if [ "$choice" -gt 0 ] && [ "$choice" -le ${#files[@]} ]; then
        restore_rescue_list "${files[$((choice-1))]}"
    fi
}

run_audit_cli() {
    local cleaner_type=${1:-}
    local manufacturer=${2:-}
    local os_version=${3:-}

    if [ -z "$cleaner_type" ] || [ -z "$manufacturer" ] || [ -z "$os_version" ]; then
        show_error "Usage: --audit <CleanerType> <Manufacturer> <OS-Slug>"
        echo "Example: ./BloatwareHatao.sh --audit Safe samsung oneui-6"
        finalize_and_exit 1
    fi

    if preview_bloatware "$manufacturer" "$os_version" "$cleaner_type"; then
        show_success "Audit completed for $manufacturer $os_version ($cleaner_type)."
    else
        finalize_and_exit 1
    fi
}

# Run main function
main "$@"
