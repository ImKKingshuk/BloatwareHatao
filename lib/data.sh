#!/bin/bash

# Data Module for BloatwareHatao
# Handles data loading, package lists, and configuration data

# Supported manufacturers
MANUFACTURERS_ARRAY=(
    samsung
    xiaomi
    huawei
    oneplus
    vivo
    oppo
    realme
    nothing
    honor
    motorola
    meizu
    infinix
)

# OS options (Display|slug) for each manufacturer
get_os_versions() {
    local manufacturer=$1
    case $manufacturer in
        "samsung")
            cat <<EOF
OneUI 6|oneui-6
OneUI 5|oneui-5
OneUI 4|oneui-4
EOF
            ;;
        "xiaomi")
            cat <<EOF
HyperOS 1|hyperos-1
MIUI 14|miui-14
MIUI 13|miui-13
EOF
            ;;
        "huawei")
            cat <<EOF
EMUI 14|emui-14
EMUI 13|emui-13
EMUI 12|emui-12
EOF
            ;;
        "oneplus")
            cat <<EOF
OxygenOS 14|oxygenos-14
OxygenOS 13|oxygenos-13
OxygenOS 12|oxygenos-12
EOF
            ;;
        "vivo")
            cat <<EOF
FuntouchOS 14|funtouchos-14
FuntouchOS 13|funtouchos-13
FuntouchOS 12|funtouchos-12
EOF
            ;;
        "oppo")
            cat <<EOF
ColorOS 14|coloros-14
ColorOS 13|coloros-13
ColorOS 12|coloros-12
EOF
            ;;
        "realme")
            cat <<EOF
RealmeUI 5|realmeui-5
RealmeUI 4|realmeui-4
RealmeUI 3|realmeui-3
EOF
            ;;
        "nothing")
            cat <<EOF
NothingOS 3|nothingos-3
NothingOS 2|nothingos-2
NothingOS 1|nothingos-1
EOF
            ;;
        "honor")
            cat <<EOF
MagicUI 8|magicos-8
MagicUI 7|magicos-7
MagicUI 6|magicos-6
EOF
            ;;
        "motorola")
            cat <<EOF
HelloUI 1|helloui-1
MyUX 13|myux-13
MyUX 12|myux-12
EOF
            ;;
        "meizu")
            cat <<EOF
FlymeAIOS 11|flymeaios-11
FlymeOS 10|flymeos-10
FlymeOS 9|flymeos-9
EOF
            ;;
        "infinix")
            cat <<EOF
XOS 14|xos-14
XOS 13|xos-13
XOS 12|xos-12
EOF
            ;;
        *)
            echo ""
            ;;
    esac
}

# Get manufacturer by index (1-based)
get_manufacturer_name() {
    local index=$1
    local array_index=$((index - 1))
    if [ $array_index -ge 0 ] && [ $array_index -lt ${#MANUFACTURERS_ARRAY[@]} ]; then
        echo "${MANUFACTURERS_ARRAY[$array_index]}"
    fi
}

# Get manufacturer count
get_manufacturer_count() {
    echo "${#MANUFACTURERS_ARRAY[@]}"
}

# Map cleaner display to directory name
cleaner_display_to_dir() {
    local display=$1
    case $display in
        "Safe & Recommended") echo "Safe" ;;
        "Pro (Extra Cleaning)") echo "Pro" ;;
        "Ultra (Extreme Cleaning)") echo "Ultra" ;;
        *) echo "Safe" ;;
    esac
}

# Validate manufacturer/OS combination
validate_manufacturer_os() {
    local manufacturer=$1
    local os_slug=$2

    while IFS='|' read -r display slug; do
        [ -z "$display" ] && continue
        if [ "$slug" = "$os_slug" ]; then
            return 0
        fi
    done <<EOF
$(get_os_versions "$manufacturer")
EOF

    return 1
}

# Load configuration data
load_config_data() {
    # Load default settings
    DEFAULT_DRY_RUN=${DEFAULT_DRY_RUN:-false}
    DEFAULT_BACKUP_BEFORE_REMOVE=${DEFAULT_BACKUP_BEFORE_REMOVE:-true}
    DEFAULT_VERBOSE=${DEFAULT_VERBOSE:-true}
    DEFAULT_AUTO_UPDATE=${DEFAULT_AUTO_UPDATE:-true}
}

# Check if custom script exists
custom_script_exists() {
    local name=$1
    [ -f "data/custom/$name.sh" ]
}

# List custom scripts
list_custom_scripts() {
    if [ -d "data/custom" ]; then
        ls -1 data/custom/*.sh 2>/dev/null | sed 's|data/custom/||' | sed 's|\.sh$||'
    fi
}

# Load custom script content
load_custom_script() {
    local name=$1
    if custom_script_exists "$name"; then
        cat "data/custom/$name.sh"
    fi
}
