#!/bin/bash

 
declare -a bloatware=(
    "com.samsung.bloatware1"
    "com.samsung.bloatware2"
    "com.samsung.bloatware3"
    # Add more bloatware package names as needed
)

for package in "${bloatware[@]}"; do
    adb shell pm uninstall --user 0 "$package"
done
