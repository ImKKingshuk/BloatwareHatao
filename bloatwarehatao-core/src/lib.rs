//! BloatwareHatao Core Library
//!
//! This crate provides the core functionality for the BloatwareHatao Android bloatware removal tool.
//!
//! # Modules
//!
//! - `adb` - ADB (Android Debug Bridge) communication and command execution
//! - `device` - Device detection, information, and health monitoring
//! - `package` - Package operations (list, uninstall, disable, enable)
//! - `database` - Package database with metadata, safety ratings, and descriptions
//! - `rescue` - Rescue history and restore functionality
//! - `preset` - Removal preset system (built-in and custom presets)
//! - `config` - Configuration management

pub mod adb;
pub mod category;
pub mod config;
pub mod database;
pub mod device;
pub mod error;
pub mod package;
pub mod preset;
pub mod rescue;

pub use error::{Error, Result};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Application name
pub const APP_NAME: &str = "BloatwareHatao";
