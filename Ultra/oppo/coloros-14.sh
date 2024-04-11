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
    "com.coloros.wallet"
    "com.coloros.video"
    "com.coloros.securepay"
    "com.coloros.screenrecorder"
    "com.coloros.sauhelper"
    "com.coloros.safesdkproxy"
    "com.coloros.ocrscanner"
    "com.coloros.oshare"
    "com.coloros.healthcheck"
    "com.coloros.compass2"
    "com.coloros.soundrecorder"
    "com.coloros.smartdrive"
    "com.dropboxchmod"
    "com.nearme.browser"
    "com.nearme.gamecenter"
    "com.nearme.statistics.rom"
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
