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
    

    # Add more bloatware package names as needed
)

for package in "${bloatware[@]}"; do
    adb shell pm uninstall --user 0 "$package"
done
