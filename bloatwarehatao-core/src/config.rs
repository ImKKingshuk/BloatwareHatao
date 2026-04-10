//! Configuration module
//!
//! Provides application configuration management.

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;

use crate::package::RemovalMode;
use crate::{Error, Result};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Default removal mode
    pub removal_mode: RemovalMode,
    /// Enable dry run mode by default
    pub dry_run: bool,
    /// Create backup before removal
    pub backup_before_remove: bool,
    /// Enable verbose output
    pub verbose: bool,
    /// Check for updates on startup
    pub auto_update_check: bool,
    /// Use offline mode (local package database)
    pub offline_mode: bool,
    /// Enable NDJSON session reports
    pub ndjson_reports: bool,
    /// Maximum safety rating to show by default
    pub max_safety_rating: String,
    /// Custom ADB path (if not in PATH)
    pub adb_path: Option<String>,
    /// Theme preference
    pub theme: Theme,
    /// UI preferences
    pub ui: UiConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            removal_mode: RemovalMode::Uninstall,
            dry_run: false,
            backup_before_remove: true,
            verbose: false,
            auto_update_check: true,
            offline_mode: false,
            ndjson_reports: false,
            max_safety_rating: "advanced".to_string(),
            adb_path: None,
            theme: Theme::System,
            ui: UiConfig::default(),
        }
    }
}

/// Theme preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    System,
    Light,
    Dark,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiConfig {
    /// Show package descriptions
    pub show_descriptions: bool,
    /// Show safety warnings
    pub show_safety_warnings: bool,
    /// Confirm before removal
    pub confirm_removal: bool,
    /// Show progress indicators
    pub show_progress: bool,
    /// Enable animations (TUI)
    pub animations: bool,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            show_descriptions: true,
            show_safety_warnings: true,
            confirm_removal: true,
            show_progress: true,
            animations: true,
        }
    }
}

/// Application directories
#[derive(Debug, Clone)]
pub struct AppDirs {
    /// Config directory
    pub config_dir: PathBuf,
    /// Data directory
    pub data_dir: PathBuf,
    /// Cache directory
    pub cache_dir: PathBuf,
}

impl AppDirs {
    /// Get application directories
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "imkkingshuk", "bloatwarehatao")
            .ok_or_else(|| Error::config("Failed to determine application directories"))?;

        Ok(Self {
            config_dir: proj_dirs.config_dir().to_path_buf(),
            data_dir: proj_dirs.data_dir().to_path_buf(),
            cache_dir: proj_dirs.cache_dir().to_path_buf(),
        })
    }

    /// Ensure all directories exist
    pub fn ensure_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        Ok(())
    }

    /// Get config file path
    pub fn config_file(&self) -> PathBuf {
        self.config_dir.join("config.toml")
    }

    /// Get package database directory
    pub fn packages_dir(&self) -> PathBuf {
        self.data_dir.join("packages")
    }

    /// Get profiles directory
    pub fn profiles_dir(&self) -> PathBuf {
        self.data_dir.join("profiles")
    }

    /// Get backups directory
    pub fn backups_dir(&self) -> PathBuf {
        self.data_dir.join("backups")
    }

    /// Get logs directory
    pub fn logs_dir(&self) -> PathBuf {
        self.data_dir.join("logs")
    }
}

impl Default for AppDirs {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            config_dir: PathBuf::from(".config"),
            data_dir: PathBuf::from(".data"),
            cache_dir: PathBuf::from(".cache"),
        })
    }
}

/// Configuration manager
#[derive(Debug)]
pub struct ConfigManager {
    dirs: AppDirs,
    config: Config,
}

impl ConfigManager {
    /// Create a new config manager
    pub fn new() -> Result<Self> {
        let dirs = AppDirs::new()?;
        dirs.ensure_dirs()?;

        let config = Self::load_from_file(&dirs.config_file()).unwrap_or_default();

        Ok(Self { dirs, config })
    }

    /// Get the current configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get mutable configuration
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Get application directories
    pub fn dirs(&self) -> &AppDirs {
        &self.dirs
    }

    /// Load configuration from a file
    fn load_from_file(path: &Path) -> Result<Config> {
        if !path.exists() {
            return Ok(Config::default());
        }

        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let content = toml::to_string_pretty(&self.config)
            .map_err(|e| Error::config(format!("Failed to serialize config: {}", e)))?;
        
        std::fs::write(self.dirs.config_file(), content)?;
        info!("Saved configuration to {:?}", self.dirs.config_file());
        Ok(())
    }

    /// Reset to default configuration
    pub fn reset(&mut self) {
        self.config = Config::default();
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            dirs: AppDirs::default(),
            config: Config::default(),
        })
    }
}
