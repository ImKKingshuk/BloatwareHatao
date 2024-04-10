#!/bin/bash


remove_bloatware_manual() {
    package_name=$1
    
    echo "Attempting to remove bloatware with package name: $package_name"
    adb shell pm uninstall --user 0 "$package_name"
    
    echo "Bloatware removal completed."
}


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
