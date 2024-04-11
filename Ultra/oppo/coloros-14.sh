#!/bin/bash

 
declare -a bloatware=(
    "com.heytap.browser"
    "com.heytap.habit.analysis"
    "com.heytap.cloud"





    # Add more bloatware package names as needed
)

for package in "${bloatware[@]}"; do
    adb shell pm uninstall --user 0 "$package"
done
