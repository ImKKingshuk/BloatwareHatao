#!/bin/bash


remove_bloatware() {
    manufacturer=$1
    os_version=$2
    cleaner_type=$3
    script_path="https://raw.githubusercontent.com/ImKKingshuk/BloatwareHatao/main/$cleaner_type/$manufacturer/$os_version.sh"

    echo "Fetching bloatware removal script from $manufacturer for $os_version ($cleaner_type)..."
    curl -sSL "$script_path" | bash

    echo "Bloatware removal completed."
}


remove_bloatware_manual() {
    package_name=$1

    echo "Attempting to remove bloatware with package name: $package_name"
    adb shell pm uninstall --user 0 "$package_name"

    echo "Bloatware removal completed."
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
        1)
            cleaner_type="Safe"
            ;;
        2)
            cleaner_type="Pro"
            ;;
        3)
            cleaner_type="Ultra"
            ;;
        4)
            cleaner_type="Manual"
            ;;
        5)
            echo "Exiting..."
            exit
            ;;
        *)
            echo "Invalid choice. Please try again."
            show_cleaner_type_menu
            return
            ;;
    esac

    if [ "$cleaner_type" = "cleaner4" ]; then
        manual_cleaner_menu
    else
        show_manufacturer_menu "$cleaner_type"
    fi
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
        [Yy][Ee][Ss])
            remove_bloatware_manual "$package_name"
            ;;
        [Nn][Oo])
            echo "Bloatware removal cancelled."
            ;;
        *)
            echo "Invalid choice. Bloatware removal cancelled."
            ;;
    esac
}


show_manufacturer_menu() {
    cleaner_type=$1

    echo "Select your device manufacturer:"
    echo "1. Samsung"
    echo "2. Xiaomi"
    echo "3. OnePlus"
    echo "4. Vivo"
    echo "5. OPPO"
    echo "6. Realme"
    echo "7. Nothing"
    echo "8. Honor"
    echo "9. Motorola"
    echo "10. Exit"
    echo "--------------------------------------"
    read -p "Enter your choice: " manufacturer_choice

    case $manufacturer_choice in
        1)
            manufacturer="samsung"
            ;;
        2)
            manufacturer="xiaomi"
            ;;
        3)
            manufacturer="oneplus"
            ;;
        4)
            manufacturer="vivo"
            ;;
        5)
            manufacturer="oppo"
            ;;
        6)
            manufacturer="realme"
            ;;
        7)
            manufacturer="nothing"
            ;;
        8)
            manufacturer="honor"
            ;;
        9)
            manufacturer="motorola"
            ;;
        10)
            echo "Exiting..."
            exit
            ;;
        *)
            echo "Invalid choice. Please try again."
            show_manufacturer_menu "$cleaner_type"
            return
            ;;
    esac

    show_os_version_menu "$manufacturer" "$cleaner_type"
}


show_os_version_menu() {
    manufacturer=$1
    cleaner_type=$2

    echo "Select your $manufacturer's OS version:"


    case $manufacturer in
        "samsung")
            echo "1. Samsung OneUI 6.0"
            echo "2. Samsung OneUI 5.0"
            echo "3. Samsung OneUI 4.0"
            ;;
        "xiaomi")
            echo "1. Xiaomi HyperOS 1"
            echo "2. Xiaomi MIUI 14"
            echo "3. Xiaomi MIUI 13"
            ;;
        "oneplus")
            echo "1. OnePlus OxygenOS 14"
            echo "2. OnePlus OxygenOS 13"
            echo "3. OnePlus OxygenOS 12"
            ;;
        "vivo")
            echo "1. Vivo FuntouchOS 14"
            echo "2. Vivo FuntouchOS 13"
            echo "3. Vivo FuntouchOS 12"
            ;;
        "oppo")
            echo "1. OPPO ColorOS 14"
            echo "2. OPPO ColorOS 13"
            echo "3. OPPO ColorOS 12"
            ;;
        "realme")
            echo "1. Realme RealmeUI 3"
            echo "2. Realme RealmeUI 4"
            echo "3. Realme RealmeUI 5"
            ;;
        "nothing")
            echo "1. NothingOS 1"
            echo "2. NothingOS 2"
            echo "3. NothingOS 2.5"
            ;;
        "honor")
            echo "1. Honor MagicUI 6"
            echo "2. Honor MagicUI 7"
            echo "3. Honor MagicUI 8"
            ;;
        "motorola")
            echo "1. Motorola MyUX 12"
            echo "2. Motorola MyUX 13"
            echo "3. Motorola HelloUI 1"
            ;;
        "meizu")
            echo "1. Meizu FlymeAIOS 11"
            echo "2. Meizu FlymeOS 10"
            echo "3. Meizu FlymeOS 9"
            ;;
    esac

    echo "4. Back"
    echo "5. Exit"
    echo "--------------------------------------"
    read -p "Enter your choice: " os_choice

    case $os_choice in
        1)
            os_version="${manufacturer}/oneui-1.sh"
            ;;
        2)
            os_version="${manufacturer}/oneui-2.sh"
            ;;
        3)
           os_version="${manufacturer}/oneui-3.sh"
            ;;
        4)
            show_manufacturer_menu "$cleaner_type"
            return
            ;;
        5)
            echo "Exiting..."
            exit
            ;;
        *)
            echo "Invalid choice. Please try again."
            show_os_version_menu "$manufacturer" "$cleaner_type"
            return
            ;;
    esac

    remove_bloatware "$manufacturer" "$os_version" "$cleaner_type"
}


    echo "******************************************"
echo -e  "* \e[48;5;52m\e[1m  BloatwareHatao \e[0m *"
    echo "*     Android Bloatware Removal Too      *"
    echo "*      ----------------------------      *"
    echo "*                        by @ImKKingshuk *"
    echo "* Github- https://github.com/ImKKingshuk *"
    echo "******************************************"




show_cleaner_type_menu
