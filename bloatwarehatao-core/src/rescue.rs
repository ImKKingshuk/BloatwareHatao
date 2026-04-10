//! Rescue and restore module
//!
//! Provides functionality for rescue history and package restoration.

use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::{info, instrument};

use crate::adb::Adb;
use crate::package::PackageManager;
use crate::{Error, Result};

/// Rescue entry containing package list and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RescueEntry {
    /// Unique rescue ID
    pub id: String,
    /// Rescue creation timestamp
    pub created_at: DateTime<Utc>,
    /// Device serial
    pub device_serial: Option<String>,
    /// Device model
    pub device_model: Option<String>,
    /// List of packages in rescue point
    pub packages: Vec<String>,
    /// Optional description
    pub description: Option<String>,
}

impl RescueEntry {
    /// Create a new rescue entry from package list
    pub fn new(packages: Vec<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            device_serial: None,
            device_model: None,
            packages,
            description: None,
        }
    }

    /// Set device information
    pub fn with_device(mut self, serial: impl Into<String>, model: impl Into<String>) -> Self {
        self.device_serial = Some(serial.into());
        self.device_model = Some(model.into());
        self
    }

    /// Set description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Get local timestamp
    pub fn local_time(&self) -> DateTime<Local> {
        self.created_at.with_timezone(&Local)
    }

    /// Get filename for this rescue entry
    pub fn filename(&self) -> String {
        let timestamp = self.created_at.format("%Y%m%d_%H%M%S");
        format!("rescue_{}.json", timestamp)
    }
}

/// Rescue session for tracking removed packages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RescueSession {
    /// Session ID
    pub session_id: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Packages removed in this session
    pub removed_packages: Vec<RemovedPackage>,
}

/// Information about a removed package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemovedPackage {
    /// Package name
    pub name: String,
    /// Removal timestamp
    pub removed_at: DateTime<Utc>,
    /// Removal method used
    pub method: String,
}

impl RescueSession {
    /// Create a new rescue session
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            created_at: Utc::now(),
            removed_packages: Vec::new(),
        }
    }

    /// Add a removed package
    pub fn add(&mut self, package: impl Into<String>, method: impl Into<String>) {
        self.removed_packages.push(RemovedPackage {
            name: package.into(),
            removed_at: Utc::now(),
            method: method.into(),
        });
    }

    /// Get package names only
    pub fn package_names(&self) -> Vec<&str> {
        self.removed_packages.iter().map(|p| p.name.as_str()).collect()
    }
}

/// Rescue manager for handling rescue operations
#[derive(Debug)]
pub struct RescueManager {
    /// Directory to store rescue entries
    rescue_dir: PathBuf,
    /// Directory to store rescue sessions
    session_dir: PathBuf,
}

impl RescueManager {
    /// Create a new rescue manager with the given directories
    pub fn new(rescue_dir: impl Into<PathBuf>, session_dir: impl Into<PathBuf>) -> Self {
        Self {
            rescue_dir: rescue_dir.into(),
            session_dir: session_dir.into(),
        }
    }

    /// Create from a base data directory
    pub fn from_data_dir(data_dir: &Path) -> Self {
        Self {
            rescue_dir: data_dir.join("rescue"),
            session_dir: data_dir.join("sessions"),
        }
    }

    /// Get rescue directory path
    pub fn rescue_dir(&self) -> &Path {
        &self.rescue_dir
    }

    /// Get session directory path
    pub fn session_dir(&self) -> &Path {
        &self.session_dir
    }

    /// Ensure directories exist
    pub fn ensure_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(&self.rescue_dir)?;
        std::fs::create_dir_all(&self.session_dir)?;
        Ok(())
    }

    /// Create a rescue point of installed packages
    #[instrument(skip(self, adb))]
    pub async fn create_rescue_point(&self, adb: &Adb) -> Result<RescueEntry> {
        self.ensure_dirs()?;

        let pm = PackageManager::new(adb.clone());
        let packages = pm.list_packages().await?;

        let entry = RescueEntry::new(packages);
        let path = self.rescue_dir.join(entry.filename());

        let content = serde_json::to_string_pretty(&entry)?;
        std::fs::write(&path, content)?;

        info!("Created rescue point: {:?} with {} packages", path, entry.packages.len());
        Ok(entry)
    }

    /// List all rescue entries (rescue history)
    pub fn list_rescue_history(&self) -> Result<Vec<RescueEntry>> {
        self.ensure_dirs()?;

        let mut entries = Vec::new();

        for entry in std::fs::read_dir(&self.rescue_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "json").unwrap_or(false)
                && let Ok(content) = std::fs::read_to_string(&path)
                && let Ok(rescue_entry) = serde_json::from_str::<RescueEntry>(&content) {
                    entries.push(rescue_entry);
            }
        }

        // Sort by date, newest first
        entries.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(entries)
    }

    /// Load a specific rescue entry by ID
    pub fn load_rescue_entry(&self, id: &str) -> Result<RescueEntry> {
        let entries = self.list_rescue_history()?;
        entries
            .into_iter()
            .find(|e| e.id == id)
            .ok_or_else(|| Error::rescue(format!("Rescue entry not found: {}", id)))
    }

    /// Save a rescue session
    pub fn save_rescue_session(&self, session: &RescueSession) -> Result<PathBuf> {
        self.ensure_dirs()?;

        let timestamp = session.created_at.format("%Y%m%d_%H%M%S");
        let filename = format!("session_{}.json", timestamp);
        let path = self.session_dir.join(&filename);

        let content = serde_json::to_string_pretty(session)?;
        std::fs::write(&path, content)?;

        info!("Saved rescue session: {:?} with {} packages", path, session.removed_packages.len());
        Ok(path)
    }

    /// List all rescue sessions
    pub fn list_rescue_sessions(&self) -> Result<Vec<RescueSession>> {
        self.ensure_dirs()?;

        let mut sessions = Vec::new();

        for entry in std::fs::read_dir(&self.session_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "json").unwrap_or(false)
                && let Ok(content) = std::fs::read_to_string(&path)
                && let Ok(session) = serde_json::from_str::<RescueSession>(&content) {
                    sessions.push(session);
            }
        }

        // Sort by date, newest first
        sessions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(sessions)
    }

    /// Restore packages from a rescue entry
    #[instrument(skip(self, adb))]
    pub async fn restore_from_entry(&self, adb: &Adb, entry: &RescueEntry) -> Result<Vec<(String, bool)>> {
        let pm = PackageManager::new(adb.clone());
        let mut results = Vec::new();

        for package_name in &entry.packages {
            let result = pm.reinstall(package_name).await;
            let success = result.map(|r| r.success).unwrap_or(false);
            results.push((package_name.clone(), success));
        }

        Ok(results)
    }

    /// Restore packages from a rescue session
    #[instrument(skip(self, adb))]
    pub async fn restore_from_session(&self, adb: &Adb, session: &RescueSession) -> Result<Vec<(String, bool)>> {
        let pm = PackageManager::new(adb.clone());
        let mut results = Vec::new();

        for package in &session.removed_packages {
            let result = pm.reinstall(&package.name).await;
            let success = result.map(|r| r.success).unwrap_or(false);
            results.push((package.name.clone(), success));
        }

        Ok(results)
    }
}
