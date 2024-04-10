#!/bin/bash

 
declare -a bloatware=(
    "com.miui.android.fashiongallery"
    "com.miui.bugreport"
  
    "com.miui.videoplayer"
    "com.mi.globalbrowser"
    "com.miui.player"
    "com.xiaomi.glgm"
 
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
   
   
    # Add more bloatware package names as needed
)

for package in "${bloatware[@]}"; do
    adb shell pm uninstall --user 0 "$package"
done
