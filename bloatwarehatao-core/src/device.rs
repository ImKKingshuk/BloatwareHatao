//! Device management module
//!
//! Provides device detection, information, and health monitoring.

use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::adb::{Adb, DeviceInfo as AdbDeviceInfo};
use crate::{Error, Result};

/// Detailed device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    /// Device serial number
    pub serial: String,
    /// Device model name
    pub model: String,
    /// Device manufacturer
    pub manufacturer: String,
    /// Device brand
    pub brand: String,
    /// Android version (e.g., "14")
    pub android_version: String,
    /// SDK/API level (e.g., 34)
    pub sdk_version: u32,
    /// Build ID
    pub build_id: String,
    /// Product name
    pub product: String,
    /// Device codename
    pub device: String,
    /// Security patch level
    pub security_patch: Option<String>,
}

impl Device {
    /// Fetch device information from a connected device
    #[instrument(skip(adb))]
    pub async fn from_adb(adb: &Adb) -> Result<Self> {
        let model = adb
            .shell("getprop ro.product.model")
            .await
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let manufacturer = adb
            .shell("getprop ro.product.manufacturer")
            .await
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let brand = adb
            .shell("getprop ro.product.brand")
            .await
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let android_version = adb
            .shell("getprop ro.build.version.release")
            .await
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let sdk_str = adb
            .shell("getprop ro.build.version.sdk")
            .await
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        let sdk_version = sdk_str.parse().unwrap_or(0);

        let build_id = adb
            .shell("getprop ro.build.display.id")
            .await
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let product = adb
            .shell("getprop ro.product.name")
            .await
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let device = adb
            .shell("getprop ro.product.device")
            .await
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let security_patch = adb
            .shell("getprop ro.build.version.security_patch")
            .await
            .map(|s| s.trim().to_string())
            .ok();

        // Get serial from ADB
        let devices = adb.devices().await?;
        let serial = devices
            .first()
            .map(|d| d.serial.clone())
            .unwrap_or_default();

        Ok(Self {
            serial,
            model,
            manufacturer,
            brand,
            android_version,
            sdk_version,
            build_id,
            product,
            device,
            security_patch,
        })
    }

    /// Get a display-friendly name for the device
    pub fn display_name(&self) -> String {
        if !self.model.is_empty() {
            format!("{} {}", self.brand, self.model)
        } else {
            self.serial.clone()
        }
    }

    /// Detect the OEM for package database matching
    pub fn detect_oem(&self) -> Oem {
        let manufacturer_lower = self.manufacturer.to_lowercase();
        let brand_lower = self.brand.to_lowercase();

        match manufacturer_lower.as_str() {
            "samsung" => Oem::Samsung,
            "xiaomi" => Oem::Xiaomi,
            "huawei" => Oem::Huawei,
            "honor" => Oem::Honor,
            "oneplus" => Oem::OnePlus,
            "oppo" => Oem::Oppo,
            "vivo" => Oem::Vivo,
            "realme" => Oem::Realme,
            "nothing" => Oem::Nothing,
            "motorola" => Oem::Motorola,
            "lenovo" if brand_lower == "motorola" => Oem::Motorola,
            "meizu" => Oem::Meizu,
            "infinix" => Oem::Infinix,
            "tecno" => Oem::Tecno,
            "itel" => Oem::Itel,
            "asus" => Oem::Asus,
            "sony" => Oem::Sony,
            "lg" | "lge" => Oem::Lg,
            "nokia" | "hmd global" => Oem::Nokia,
            "google" => Oem::Google,
            _ => Oem::Generic,
        }
    }
}

/// Supported OEM manufacturers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Oem {
    Samsung,
    Xiaomi,
    Huawei,
    Honor,
    OnePlus,
    Oppo,
    Vivo,
    Realme,
    Nothing,
    Motorola,
    Meizu,
    Infinix,
    Tecno,
    Itel,
    Asus,
    Sony,
    Lg,
    Nokia,
    Google,
    Generic,
    #[serde(rename = "aosp")]
    Aosp,
    #[serde(rename = "amazon")]
    Amazon,
    #[serde(rename = "meta")]
    Meta,
    #[serde(rename = "microsoft")]
    Microsoft,
    #[serde(rename = "mediatek")]
    MediaTek,
    #[serde(rename = "qualcomm")]
    Qualcomm,
}

impl Oem {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Samsung => "Samsung",
            Self::Xiaomi => "Xiaomi",
            Self::Huawei => "Huawei",
            Self::Honor => "Honor",
            Self::OnePlus => "OnePlus",
            Self::Oppo => "OPPO",
            Self::Vivo => "Vivo",
            Self::Realme => "Realme",
            Self::Nothing => "Nothing",
            Self::Motorola => "Motorola",
            Self::Meizu => "Meizu",
            Self::Infinix => "Infinix",
            Self::Tecno => "Tecno",
            Self::Itel => "Itel",
            Self::Asus => "ASUS",
            Self::Sony => "Sony",
            Self::Lg => "LG",
            Self::Nokia => "Nokia",
            Self::Google => "Google",
            Self::Generic => "Generic Android",
            Self::Aosp => "AOSP",
            Self::Amazon => "Amazon",
            Self::Meta => "Meta",
            Self::Microsoft => "Microsoft",
            Self::MediaTek => "MediaTek",
            Self::Qualcomm => "Qualcomm",
        }
    }
}

/// Device health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceHealth {
    /// Battery level (0-100)
    pub battery_level: Option<u8>,
    /// Battery status (charging, discharging, etc.)
    pub battery_status: Option<String>,
    /// Battery temperature in tenths of Celsius
    pub battery_temp: Option<i32>,
    /// Battery health
    pub battery_health: Option<String>,
    /// Total RAM in KB
    pub ram_total_kb: Option<u64>,
    /// Available RAM in KB
    pub ram_available_kb: Option<u64>,
    /// Total internal storage in KB
    pub storage_total_kb: Option<u64>,
    /// Free internal storage in KB
    pub storage_free_kb: Option<u64>,
    /// Device uptime
    pub uptime: Option<String>,
}

impl DeviceHealth {
    /// Fetch device health from a connected device
    #[instrument(skip(adb))]
    pub async fn from_adb(adb: &Adb) -> Result<Self> {
        let battery_output = adb.shell("dumpsys battery").await.unwrap_or_default();

        let mut battery_level = None;
        let mut battery_status = None;
        let mut battery_temp = None;
        let mut battery_health = None;

        for line in battery_output.lines() {
            let line = line.trim();
            if let Some(value) = line.strip_prefix("level:") {
                battery_level = value.trim().parse().ok();
            } else if let Some(value) = line.strip_prefix("status:") {
                battery_status = Some(value.trim().to_string());
            } else if let Some(value) = line.strip_prefix("temperature:") {
                battery_temp = value.trim().parse().ok();
            } else if let Some(value) = line.strip_prefix("health:") {
                battery_health = Some(value.trim().to_string());
            }
        }

        // Get memory info
        let meminfo = adb.shell("cat /proc/meminfo").await.unwrap_or_default();
        let mut ram_total_kb = None;
        let mut ram_available_kb = None;

        for line in meminfo.lines() {
            if let Some(value) = line.strip_prefix("MemTotal:") {
                ram_total_kb = value.split_whitespace().next().and_then(|s| s.parse().ok());
            } else if let Some(value) = line.strip_prefix("MemAvailable:") {
                ram_available_kb = value.split_whitespace().next().and_then(|s| s.parse().ok());
            }
        }

        // Get storage info
        let df_output = adb.shell("df /data").await.unwrap_or_default();
        let mut storage_total_kb = None;
        let mut storage_free_kb = None;

        for line in df_output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                storage_total_kb = parts.get(1).and_then(|s| s.parse().ok());
                storage_free_kb = parts.get(3).and_then(|s| s.parse().ok());
                break;
            }
        }

        // Get uptime
        let uptime = adb.shell("uptime").await.ok();

        Ok(Self {
            battery_level,
            battery_status,
            battery_temp,
            battery_health,
            ram_total_kb,
            ram_available_kb,
            storage_total_kb,
            storage_free_kb,
            uptime,
        })
    }

    /// Get battery temperature in Celsius
    pub fn battery_temp_celsius(&self) -> Option<f32> {
        self.battery_temp.map(|t| t as f32 / 10.0)
    }

    /// Get RAM usage percentage
    pub fn ram_usage_percent(&self) -> Option<f32> {
        match (self.ram_total_kb, self.ram_available_kb) {
            (Some(total), Some(available)) if total > 0 => {
                Some(((total - available) as f32 / total as f32) * 100.0)
            }
            _ => None,
        }
    }

    /// Get storage usage percentage
    pub fn storage_usage_percent(&self) -> Option<f32> {
        match (self.storage_total_kb, self.storage_free_kb) {
            (Some(total), Some(free)) if total > 0 => {
                Some(((total - free) as f32 / total as f32) * 100.0)
            }
            _ => None,
        }
    }
}

/// Device manager for handling multiple devices
#[derive(Debug)]
pub struct DeviceManager {
    adb: Adb,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new() -> Self {
        Self { adb: Adb::new() }
    }

    /// Create with custom ADB path
    pub fn with_adb(adb: Adb) -> Self {
        Self { adb }
    }

    /// Get list of connected devices
    pub async fn list_devices(&self) -> Result<Vec<AdbDeviceInfo>> {
        self.adb.devices().await
    }

    /// Get list of ready devices (authorized and connected)
    pub async fn list_ready_devices(&self) -> Result<Vec<AdbDeviceInfo>> {
        let devices = self.adb.devices().await?;
        Ok(devices
            .into_iter()
            .filter(|d| d.status.is_ready())
            .collect())
    }

    /// Get single connected device (error if multiple)
    pub async fn get_device(&self) -> Result<Adb> {
        let devices = self.list_ready_devices().await?;

        match devices.len() {
            0 => Err(Error::NoDevice),
            1 => Ok(self.adb.clone().with_device(&devices[0].serial)),
            _ => Err(Error::MultipleDevices(
                devices.iter().map(|d| d.serial.clone()).collect(),
            )),
        }
    }

    /// Get device by serial
    pub fn get_device_by_serial(&self, serial: &str) -> Adb {
        self.adb.clone().with_device(serial)
    }

    /// Check if ADB is available
    pub async fn is_adb_available(&self) -> bool {
        self.adb.is_available().await
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}
