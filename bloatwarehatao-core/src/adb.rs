//! ADB (Android Debug Bridge) communication module
//!
//! Provides async operations for communicating with Android devices via ADB.

use async_process::Command;
use std::process::Stdio;
use tracing::{debug, instrument, warn};

use crate::{Error, Result};

/// ADB command executor
#[derive(Debug, Clone)]
pub struct Adb {
    /// Path to ADB binary (defaults to "adb" in PATH)
    adb_path: String,
    /// Target device serial (None for single device)
    device_serial: Option<String>,
}

impl Default for Adb {
    fn default() -> Self {
        Self::new()
    }
}

impl Adb {
    /// Create a new ADB instance
    pub fn new() -> Self {
        Self {
            adb_path: "adb".to_string(),
            device_serial: None,
        }
    }

    /// Create ADB instance with custom binary path
    pub fn with_path(path: impl Into<String>) -> Self {
        Self {
            adb_path: path.into(),
            device_serial: None,
        }
    }

    /// Set the target device serial
    pub fn with_device(mut self, serial: impl Into<String>) -> Self {
        self.device_serial = Some(serial.into());
        self
    }

    /// Execute an ADB command and return stdout
    #[instrument(skip(self), fields(device = ?self.device_serial))]
    pub async fn exec(&self, args: &[&str]) -> Result<String> {
        let mut cmd = Command::new(&self.adb_path);

        // Add device selector if specified
        if let Some(ref serial) = self.device_serial {
            cmd.arg("-s").arg(serial);
        }

        cmd.args(args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        debug!("Executing ADB command: {:?}", args);

        let output = cmd.output().await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::adb("ADB not found. Please install Android SDK Platform Tools.")
            } else {
                Error::adb(format!("Failed to execute ADB: {}", e))
            }
        })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            warn!("ADB command failed: {}", stderr);
            return Err(Error::adb(stderr));
        }

        Ok(stdout)
    }

    /// Execute a shell command on the device
    #[instrument(skip(self))]
    pub async fn shell(&self, command: &str) -> Result<String> {
        self.exec(&["shell", command]).await
    }

    /// Check if ADB is available
    pub async fn is_available(&self) -> bool {
        self.exec(&["version"]).await.is_ok()
    }

    /// Get list of connected devices with their status
    #[instrument(skip(self))]
    pub async fn devices(&self) -> Result<Vec<DeviceInfo>> {
        let output = self.exec(&["devices", "-l"]).await?;
        let mut devices = Vec::new();

        for line in output.lines().skip(1) {
            // Skip "List of devices attached" header
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some(device) = DeviceInfo::parse(line) {
                devices.push(device);
            }
        }

        Ok(devices)
    }

    /// Wait for a device to be connected
    #[instrument(skip(self))]
    pub async fn wait_for_device(&self) -> Result<()> {
        self.exec(&["wait-for-device"]).await?;
        Ok(())
    }

    /// Start ADB server
    pub async fn start_server(&self) -> Result<()> {
        self.exec(&["start-server"]).await?;
        Ok(())
    }

    /// Kill ADB server
    pub async fn kill_server(&self) -> Result<()> {
        self.exec(&["kill-server"]).await?;
        Ok(())
    }

    /// Connect to a device over TCP/IP (wireless ADB)
    #[instrument(skip(self))]
    pub async fn connect(&self, address: &str) -> Result<()> {
        let output = self.exec(&["connect", address]).await?;
        if output.contains("connected") || output.contains("already connected") {
            Ok(())
        } else {
            Err(Error::adb(format!(
                "Failed to connect to {}: {}",
                address, output
            )))
        }
    }

    /// Disconnect from a wireless device
    pub async fn disconnect(&self, address: &str) -> Result<()> {
        self.exec(&["disconnect", address]).await?;
        Ok(())
    }

    /// Enable TCP/IP mode for wireless ADB on specified port
    /// Device must be connected via USB first
    #[instrument(skip(self))]
    pub async fn tcpip(&self, port: u16) -> Result<()> {
        let port_str = port.to_string();
        let output = self.exec(&["tcpip", &port_str]).await?;
        if output.contains("restarting") || !output.contains("error") {
            Ok(())
        } else {
            Err(Error::adb(format!(
                "Failed to enable TCP/IP mode: {}",
                output
            )))
        }
    }

    /// Get the IP address of the connected device
    #[instrument(skip(self))]
    pub async fn get_device_ip(&self) -> Result<Option<String>> {
        // Try wlan0 first (most common)
        let output = self
            .shell("ip addr show wlan0 2>/dev/null | grep 'inet ' | awk '{print $2}' | cut -d/ -f1")
            .await?;
        let ip = output.trim();
        if !ip.is_empty() && ip.contains('.') {
            return Ok(Some(ip.to_string()));
        }

        // Fallback to any available interface
        let output = self
            .shell("ip route get 1 2>/dev/null | awk '{print $7}' | head -1")
            .await?;
        let ip = output.trim();
        if !ip.is_empty() && ip.contains('.') {
            return Ok(Some(ip.to_string()));
        }

        Ok(None)
    }
}

/// Device connection status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceStatus {
    /// Device is connected and authorized
    Device,
    /// Device needs USB debugging authorization
    Unauthorized,
    /// Device is offline
    Offline,
    /// Device is in recovery mode
    Recovery,
    /// Device is in sideload mode
    Sideload,
    /// Unknown status
    Unknown(String),
}

impl DeviceStatus {
    fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "device" => Self::Device,
            "unauthorized" => Self::Unauthorized,
            "offline" => Self::Offline,
            "recovery" => Self::Recovery,
            "sideload" => Self::Sideload,
            other => Self::Unknown(other.to_string()),
        }
    }

    /// Check if device is ready for commands
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Device)
    }
}

/// Information about a connected device
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Device serial number
    pub serial: String,
    /// Connection status
    pub status: DeviceStatus,
    /// Device model (if available)
    pub model: Option<String>,
    /// Device product name (if available)
    pub product: Option<String>,
    /// Transport ID
    pub transport_id: Option<String>,
}

impl DeviceInfo {
    /// Parse device info from ADB devices -l output line
    fn parse(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }

        let serial = parts[0].to_string();
        let status = DeviceStatus::parse(parts[1]);

        let mut model = None;
        let mut product = None;
        let mut transport_id = None;

        for part in parts.iter().skip(2) {
            if let Some(value) = part.strip_prefix("model:") {
                model = Some(value.to_string());
            } else if let Some(value) = part.strip_prefix("product:") {
                product = Some(value.to_string());
            } else if let Some(value) = part.strip_prefix("transport_id:") {
                transport_id = Some(value.to_string());
            }
        }

        Some(Self {
            serial,
            status,
            model,
            product,
            transport_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_info_parse() {
        let line = "RFXXXXXXXX device usb:1-1 product:starqltesq model:SM_G960U device:starqltesq transport_id:1";
        let info = DeviceInfo::parse(line).unwrap();

        assert_eq!(info.serial, "RFXXXXXXXX");
        assert!(info.status.is_ready());
        assert_eq!(info.model, Some("SM_G960U".to_string()));
        assert_eq!(info.product, Some("starqltesq".to_string()));
    }

    #[test]
    fn test_device_status_parse() {
        assert!(DeviceStatus::parse("device").is_ready());
        assert!(!DeviceStatus::parse("unauthorized").is_ready());
        assert!(!DeviceStatus::parse("offline").is_ready());
    }
}
