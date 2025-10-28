#!/bin/bash

# UI Module for BloatwareHatao
# Handles all user interface elements, menus, and output formatting

# Color codes for enhanced UI
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
NC='\033[0m' # No Color

# Print colored text
print_color() {
    local color=$1
    local text=$2
    echo -e "${color}${text}${NC}"
}

# Print banner
print_banner() {
    clear
    local banner=(
        "******************************************"
        "*              BloatwareHatao            *"
        "*     Ultimate Android Bloatware Tool    *"
        "*                  v2.0.0                *"
        "*      ----------------------------      *"
        "*                        by @ImKKingshuk *"
        "* Github- https://github.com/ImKKingshuk *"
        "******************************************"
    )
    local width=$(tput cols)
    for line in "${banner[@]}"; do
        printf "%*s\n" $(((${#line} + width) / 2)) "$line"
    done
    echo
    print_color $GREEN "Welcome! Let's rejuvenate your Android device."
    print_color $WHITE "We'll guide you through each step—whether you're a first-time cleaner or a pro."
}

# Show progress bar
show_progress() {
    local current=$1
    local total=$2
    local width=50
    local percentage=$((current * 100 / total))
    local completed=$((current * width / total))

    printf "\rProgress: ["
    for ((i=1; i<=completed; i++)); do printf "="; done
    for ((i=completed+1; i<=width; i++)); do printf " "; done
    printf "] %d%%" $percentage
}

# Show spinner
show_spinner() {
    local pid=$1
    local delay=0.1
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b"
}

# Display menu with options
show_menu() {
    local title=$1
    shift
    local options=("$@")

    echo
    print_color $BLUE "$title"
    echo "--------------------------------------"
    for i in "${!options[@]}"; do
        echo "$((i+1)). ${options[$i]}"
    done
    echo "--------------------------------------"
}

# Display standard removal menu
show_standard_removal_menu() {
    show_info "Standard Bloatware Removal"
    print_divider
    show_info "We’ll help you pick the right cleaning level and device options."
    show_info "Need help deciding? Choose Safe for most users, Pro if you want more cleanup, Ultra for experts."

    # Select cleaner type
    local cleaner_options=("Safe & Recommended" "Pro (Extra Cleaning)" "Ultra (Extreme Cleaning)")
    show_menu "Select Cleaner Type" "${cleaner_options[@]}"
    local cleaner_choice=$(get_choice ${#cleaner_options[@]})
}

# Get user choice with validation
get_choice() {
    local max_choice=$1
    local choice
    while true; do
        read -p "Enter your choice (1-$max_choice): " choice
        if [[ $choice =~ ^[0-9]+$ ]] && [ "$choice" -ge 1 ] && [ "$choice" -le "$max_choice" ]; then
            echo $choice
            return
        else
            print_color $RED "Invalid choice. Please enter a number between 1 and $max_choice."
        fi
    done
}

# Confirm action
confirm_action() {
    local message=$1
    local response
    read -p "$message (y/N): " response
    case $response in
        [Yy]|[Yy][Ee][Ss]) return 0 ;;
        *) return 1 ;;
    esac
}

# Display success message
show_success() {
    print_color $GREEN "✓ $1"
}

# Display error message
show_error() {
    print_color $RED "✗ $1"
}

# Display warning
show_warning() {
    print_color $YELLOW "⚠ $1"
}

# Display info
show_info() {
    print_color $BLUE "ℹ $1"
}

# Display dry run message
show_dry_run_notice() {
    print_color $PURPLE "🧪 DRY RUN ACTIVE: $1"
}

# Section divider
print_divider() {
    print_color $CYAN "--------------------------------------"
}

# Display help
show_help() {
    echo
    print_color $CYAN "BloatwareHatao Help"
    echo "======================"
    echo "Available commands:"
    echo "  --help, -h          Show this help message"
    echo "  --version, -v       Show version information"
    echo "  --dry-run           Show what would be removed without actually removing"
    echo "  --backup            Create backup before removal"
    echo "  --restore           Restore from backup"
    echo "  --device-info       Show connected device information"
    echo "  --smart-wizard      Launch guided smart removal wizard"
    echo "  --audit <type> <manufacturer> <os-slug>"
    echo "                      Run pre-removal audit (e.g., --audit Safe samsung oneui-6)"
    echo "  --health            Show device health snapshot"
    echo "  --planner           Open cleaning planner"
    echo "  --report            Display current session report"
    echo "  --report-ndjson     Display NDJSON session report (if enabled)"
    echo "  --log               Show operation logs"
    echo "  --mode <uninstall|disable|clear>"
    echo "                      Set removal behavior (default: uninstall)"
    echo "  --offline           Use local OEM scripts (offline mode)"
    echo
    echo "Interactive mode: Run without arguments for full menu"
    echo
}

# Display main menu
show_main_menu() {
    local options=(
        "Standard Bloatware Removal (Interactive)"
        "Manual Package Removal"
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

    show_menu "Main Menu" "${options[@]}"
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
}

# Display device health snapshot
show_device_health_snapshot() {
    show_info "Gathering device health information..."
    print_divider
    get_device_health_report
    print_divider
    show_info "Tip: Keep battery above 30% during heavy cleaning for best results."
}

# Display cleaning planner
show_cleaning_planner() {
    show_info "Cleaning Planner"
    print_divider
    echo "Plan future cleanups or share recommendations."
    ensure_directory "$PLANS_DIR"
    local plan_file="$PLANS_DIR/plan_$(date +%Y%m%d_%H%M%S).txt"

    echo "Tell us your goal (e.g., remove social apps, prep for resale):"
    read -p "Goal: " plan_goal
    echo "How often should we remind you to run BloatwareHatao?"
    show_menu "Reminder Frequency" "Weekly" "Monthly" "Quarterly" "Only once"
    local freq_choice=$(get_choice 4)
    local freq_label
    case $freq_choice in
        1) freq_label="Weekly" ;;
        2) freq_label="Monthly" ;;
        3) freq_label="Quarterly" ;;
        4) freq_label="Once" ;;
    esac

    echo "Any specific packages or categories you want to target?"
    read -p "Notes: " plan_notes

    cat > "$plan_file" << EOF
Goal: $plan_goal
Reminder: $freq_label
Notes: $plan_notes
Created: $(date)
EOF

    record_operation PLAN "created|$plan_file"
    show_success "Plan saved to $plan_file"
}
