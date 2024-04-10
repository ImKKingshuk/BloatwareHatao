#!/bin/bash

 
declare -a bloatware=(
    "com.miui.android.fashiongallery"
    "com.miui.bugreport"
    "com.miui.msa.global"
    "com.xiaomi.glgm"
    "com.xiaomi.joyose"
    "com.xiaomi.mipicks"
    "com.xiaomi.payment"
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

    # Add more bloatware package names as needed
)

for package in "${bloatware[@]}"; do
    adb shell pm uninstall --user 0 "$package"
done
