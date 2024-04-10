#!/bin/bash

 
declare -a bloatware=(
    "com.miui.android.fashiongallery"
    "com.miui.bugreport"
    "com.miui.msa.global"
    "com.miui.videoplayer"
    "com.miui.yellowpage"
    "com.miui.wallpaper.overlay"
    "com.miui.newmidrive"
    "com.miui.videoplayer.overlay"
    "com.miui.video"
    "com.miui.miservice"
    "com.miui.fmservice"
    "com.miui.fm"
    "com.mi.globalbrowser"
    "com.miui.player"
    "com.xiaomi.glgm"
    "com.xiaomi.joyose"
    "com.xiaomi.mipicks"
    "com.xiaomi.payment"
    "com.xiaomi.migameservice"
    "cn.wps.xiaomi.abroad.lite"
    "in.amazon.mShop.android.shopping"
    "com.netflix.partner.activation"
    "com.netflix.mediaclient"
    "com.opera.app.news"
    "com.opera.branding"
    "com.opera.branding.news"
    "com.opera.mini.native"
    "com.opera.preinstall"
    "com.facebook.katana"
    "com.facebook.appmanager"
    "com.facebook.services"
    "com.facebook.system"
    "com.miui.analytics"
    "com.google.android.videos"
    "com.google.android.youtube"
    "com.google.android.apps.meetings"
    "com.google.android.apps.docs"
    "com.google.android.apps.maps"
    "com.google.android.apps.photos"
    # Add more bloatware package names as needed
)

for package in "${bloatware[@]}"; do
    adb shell pm uninstall --user 0 "$package"
done
