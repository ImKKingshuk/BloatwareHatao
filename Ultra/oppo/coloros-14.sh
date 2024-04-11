#!/bin/bash

 
declare -a bloatware=(
    "com.heytap.browser"
    "com.heytap.habit.analysis"
    "com.heytap.cloud"
    "com.heytap.market"
    "com.oppo.market"
    "com.oppo.music"
    "com.oppo.partnerbrowsercustomizations"
    "com.oppo.sos"
    "com.oppo.usageDump"
    "com.nearme.browser"
    "com.nearme.gamecenter"
    "com.nearme.statistics.rom"
    "com.opera.preinstall"
    "com.google.android.apps.youtube.music"
    "com.google.android.calendar"





    # Add more bloatware package names as needed
)

for package in "${bloatware[@]}"; do
    adb shell pm uninstall --user 0 "$package"
done
