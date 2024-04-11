#!/bin/bash

 
declare -a bloatware=(

    "com.opera.preinstall"
    "com.google.android.apps.youtube.music"
    "com.google.android.calendar"
    "com.google.android.keep"
    "com.google.android.youtube"
    "com.google.android.apps.photos"
    "com.google.android.apps.maps"
    "com.google.android.apps.magazines"
    "com.google.android.apps.docs"
    "com.facebook.system"
    "com.facebook.services"
    "com.facebook.appmanager"

    # Add more bloatware package names as needed
)

for package in "${bloatware[@]}"; do
    adb shell pm uninstall --user 0 "$package"
done
