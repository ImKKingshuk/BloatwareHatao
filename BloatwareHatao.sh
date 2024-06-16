#!/bin/bash

print_banner() {
    local banner=(
        "******************************************"
        "*              BloatwareHatao            *"
        "*     Android Bloatware Removal Tool     *"
        "*                  v1.3.1                *"
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
}

remove_bloatware() {
    local manufacturer=$1
    local os_version=$2
    local cleaner_type=$3
    local script_path="https://raw.githubusercontent.com/ImKKingshuk/BloatwareHatao/main/$cleaner_type/$manufacturer/$os_version.sh"

    echo "Fetching bloatware removal script for $manufacturer $os_version ($cleaner_type)..."
    curl -sSL "$script_path" | bash

    if [ $? -eq 0 ]; then
        echo "Bloatware removal completed successfully."
    else
        echo "Error occurred during bloatware removal."
    fi
}

remove_bloatware_manual() {
    local package_name=$1

    echo "Attempting to remove bloatware with package name: $package_name"
    adb shell pm uninstall --user 0 "$package_name"

    if [ $? -eq 0 ]; then
        echo "Bloatware removal completed successfully."
    else
        echo "Error occurred during bloatware removal."
    fi
}

show_cleaner_type_menu() {
    echo "Select the cleaner type:"
    echo "1. Bloatware Cleaner (Safe & Recommended)"
    echo "2. Pro Bloatware Cleaner (Extra Cleaning)"
    echo "3. Ultra Bloatware Cleaner (Extreme Cleaning)"
    echo "4. Manual Bloatware Cleaner (Enter APK pkg & Clean)"
    echo "5. Exit"
    echo "--------------------------------------"
    read -p "Enter your choice: " cleaner_type_choice

    case $cleaner_type_choice in
        1) cleaner_type="Safe" ;;
        2) cleaner_type="Pro" ;;
        3) cleaner_type="Ultra" ;;
        4) manual_cleaner_menu ;;
        5) echo "Exiting..."; exit ;;
        *) echo "Invalid choice. Please try again."; show_cleaner_type_menu ;;
    esac

    show_manufacturer_menu "$cleaner_type"
}

manual_cleaner_menu() {
    echo "Manual Bloatware Cleaner"
    echo "Enter the package name of the bloatware APK:"
    echo "Example: com.example.bloatware"
    echo "--------------------------------------"
    read -p "Package Name: " package_name

    echo "You have entered the package name: $package_name"
    read -p "Are you sure you want to remove this bloatware? (Yes/No): " confirmation

    case $confirmation in
        [Yy][Ee][Ss]) remove_bloatware_manual "$package_name" ;;
        [Nn][Oo]) echo "Bloatware removal cancelled." ;;
        *) echo "Invalid choice. Bloatware removal cancelled." ;;
    esac
}

show_manufacturer_menu() {
    local cleaner_type=$1

    echo "Select your device manufacturer:"
    echo "1. Samsung"
    echo "2. Xiaomi"
    echo "3. Huawei"
    echo "4. OnePlus"
    echo "5. Vivo"
    echo "6. OPPO"
    echo "7. Realme"
    echo "8. Nothing"
    echo "9. Honor"
    echo "10. Motorola"
    echo "11. Meizu"
    echo "12. Exit"
    echo "--------------------------------------"
    read -p "Enter your choice: " manufacturer_choice

    case $manufacturer_choice in
        1) manufacturer="samsung" ;;
        2) manufacturer="xiaomi" ;;
        3) manufacturer="huawei" ;;
        4) manufacturer="oneplus" ;;
        5) manufacturer="vivo" ;;
        6) manufacturer="oppo" ;;
        7) manufacturer="realme" ;;
        8) manufacturer="nothing" ;;
        9) manufacturer="honor" ;;
        10) manufacturer="motorola" ;;
        11) manufacturer="meizu" ;;
        12) echo "Exiting..."; exit ;;
        *) echo "Invalid choice. Please try again."; show_manufacturer_menu "$cleaner_type" ;;
    esac

    show_os_version_menu "$manufacturer" "$cleaner_type"
}

show_os_version_menu() {
    local manufacturer=$1
    local cleaner_type=$2

    echo "Select your $manufacturer's OS version:"

    case $manufacturer in
        "samsung")
            echo "1. OneUI 6"
            echo "2. OneUI 5"
            echo "3. OneUI 4"
            ;;
        "xiaomi")
            echo "1. HyperOS 1"
            echo "2. MIUI 14"
            echo "3. MIUI 13"
            ;;
        "huawei")
            echo "1. EMUI 14"
            echo "2. EMUI 13"
            echo "3. EMUI 12"
            ;;
        "oneplus")
            echo "1. OxygenOS 14"
            echo "2. OxygenOS 13"
            echo "3. OxygenOS 12"
            ;;
        "vivo")
            echo "1. FuntouchOS 14"
            echo "2. FuntouchOS 13"
            echo "3. FuntouchOS 12"
            ;;
        "oppo")
            echo "1. ColorOS 14"
            echo "2. ColorOS 13"
            echo "3. ColorOS 12"
            ;;
        "realme")
            echo "1. RealmeUI 5"
            echo "2. RealmeUI 4"
            echo "3. RealmeUI 3"
            ;;
        "nothing")
            echo "1. NothingOS 3"
            echo "2. NothingOS 2"
            echo "3. NothingOS 1"
            ;;
        "honor")
            echo "1. MagicUI 8"
            echo "2. MagicUI 7"
            echo "3. MagicUI 6"
            ;;
        "motorola")
            echo "1. HelloUI 1"
            echo "2. MyUX 13"
            echo "3. MyUX 12"
            ;;
        "meizu")
            echo "1. FlymeAIOS 11"
            echo "2. FlymeOS 10"
            echo "3. FlymeOS 9"
            ;;
    esac

    echo "4. Back"
    echo "5. Exit"
    echo "--------------------------------------"
    read -p "Enter your choice: " os_choice

    local os_version=""
    case $manufacturer in
        "samsung")
            case $os_choice in
                1) os_version="oneui-6" ;;
                2) os_version="oneui-5" ;;
                3) os_version="oneui-4" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "xiaomi")
            case $os_choice in
                1) os_version="hyperos-1" ;;
                2) os_version="miui-14" ;;
                3) os_version="miui-13" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "huawei")
            case $os_choice in
                1) os_version="emui-14" ;;
                2) os_version="emui-13" ;;
                3) os_version="emui-12" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "oneplus")
            case $os_choice in
                1) os_version="oxygenos-14" ;;
                2) os_version="oxygenos-13" ;;
                3) os_version="oxygenos-12" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "vivo")
            case $os_choice in
                1) os_version="funtouchos-14" ;;
                2) os_version="funtouchos-13" ;;
                3) os_version="funtouchos-12" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "oppo")
            case $os_choice in
                1) os_version="coloros-14" ;;
                2) os_version="coloros-13" ;;
                3) os_version="coloros-12" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "realme")
            case $os_choice in
                1) os_version="realmeui-5" ;;
                2) os_version="realmeui-4" ;;
                3) os_version="realmeui-3" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "nothing")
            case $os_choice in
                1) os_version="nothingos-3" ;;
                2) os_version="nothingos-2" ;;
                3) os_version="nothingos-1" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "honor")
            case $os_choice in
                1) os_version="magicos-8" ;;
                2) os_version="magicos-7" ;;
                3) os_version="magicos-6" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "motorola")
            case $os_choice in
                1) os_version="helloui-1" ;;
                2) os_version="myux-13" ;;
                3) os_version="myux-12" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
        "meizu")
            case $os_choice in
                1) os_version="flymeos-9" ;;
                2) os_version="flymeos-10" ;;
                3) os_version="flymeos-11" ;;
                4) show_manufacturer_menu "$cleaner_type"; return ;;
                5) echo "Exiting..."; exit ;;
                *) echo "Invalid choice. Please try again."; show_os_version_menu "$manufacturer" "$cleaner_type"; return ;;
            esac
            ;;
    esac

    remove_bloatware "$manufacturer" "$os_version" "$cleaner_type"
}

print_banner
show_cleaner_type_menu
