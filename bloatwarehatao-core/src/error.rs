//! Error types for BloatwareHatao

use thiserror::Error;

/// Result type alias using our Error type
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for BloatwareHatao
#[derive(Error, Debug)]
pub enum Error {
    /// ADB-related errors
    #[error("ADB error: {0}")]
    Adb(String),

    /// No device connected
    #[error("No Android device connected. Please connect a device with USB debugging enabled.")]
    NoDevice,

    /// Multiple devices connected without selection
    #[error("Multiple devices connected. Please specify a device: {0:?}")]
    MultipleDevices(Vec<String>),

    /// Device unauthorized
    #[error("Device unauthorized. Please accept the USB debugging prompt on your device.")]
    DeviceUnauthorized,

    /// Package not found
    #[error("Package not found: {0}")]
    PackageNotFound(String),

    /// Invalid package name format
    #[error("Invalid package name format: {0}")]
    InvalidPackageName(String),

    /// Package removal failed
    #[error("Failed to remove package {package}: {reason}")]
    RemovalFailed { package: String, reason: String },

    /// Database errors
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML parsing errors
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// Rescue/restore errors
    #[error("Rescue error: {0}")]
    Rescue(String),

    /// Preset errors
    #[error("Preset error: {0}")]
    Preset(String),

    /// Generic errors
    #[error("{0}")]
    Other(String),
}

impl Error {
    /// Create an ADB error
    pub fn adb(msg: impl Into<String>) -> Self {
        Self::Adb(msg.into())
    }

    /// Create a config error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Create a rescue error
    pub fn rescue(msg: impl Into<String>) -> Self {
        Self::Rescue(msg.into())
    }

    /// Create a preset error
    pub fn preset(msg: impl Into<String>) -> Self {
        Self::Preset(msg.into())
    }

    /// Create an other/generic error
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
}
