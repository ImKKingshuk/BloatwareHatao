#!/bin/bash

 
declare -a bloatware=(

    "com.bbk.calendar"
    "com.android.bbkmusic"
    "com.android.bbkcalculator"
    "com.android.bbksoundrecorder"
    "com.bbk.theme"
    "com.bbk.theme.resources"
    "com.baidu.input_vivo"
    "com.android.notes"
    "com.android.filemanager"
    "com.android.chrome"
    "com.android.BBKClock"
    "com.android.bbklog"
    "com.ibimuyu.lockscreen"
    "com.iqoo.engineermode"
    "com.vivo.appstore"
    "com.vivo.assistant"
    "com.vivo.motormode"
    "com.vivo.browser"
    "com.vivo.widget.calendar"
    "com.vivo.email"
    "com.vivo.weather.provider"
    "com.vivo.website"
    "com.vivo.FMRadio"
    "com.vivo.vivokaraoke"
    "com.vivo.hiboard"
    "com.vivo.gallery"
    "com.vivo.compass"
    "com.vivo.collage"
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
