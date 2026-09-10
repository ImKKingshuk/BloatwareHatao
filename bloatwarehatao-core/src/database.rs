//! Package database module
//!
//! Provides the universal package database with metadata, safety ratings, and descriptions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

pub use crate::category::PackageCategory;
use crate::device::Oem;
use crate::{Error, Result};

/// Safety rating for a package
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SafetyRating {
    /// User - User installed app, safe to remove if desired
    User,
    /// Recommended to remove - bloatware with no dependencies
    Recommended,
    /// Advanced - may affect some features but generally safe
    Advanced,
    /// Unsafe - may break functionality, only for experienced users
    Unsafe,
    /// Danger - critical system component, do not remove
    Danger,
}

impl SafetyRating {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::User => "User Installed",
            Self::Recommended => "Recommended",
            Self::Advanced => "Advanced",
            Self::Unsafe => "Unsafe",
            Self::Danger => "Danger",
        }
    }

    /// Get color hint (for UI)
    pub fn color_hint(&self) -> &'static str {
        match self {
            Self::User => "blue",
            Self::Recommended => "green",
            Self::Advanced => "yellow",
            Self::Unsafe => "orange",
            Self::Danger => "red",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::User => "App installed by the user. Removal will not affect system stability.",
            Self::Recommended => "Safe to remove. Bloatware with no dependencies.",
            Self::Advanced => "May affect some features. Generally safe for most users.",
            Self::Unsafe => "May break functionality. Only for experienced users.",
            Self::Danger => "Critical system component. Removal may cause issues.",
        }
    }
}

/// Package entry in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageEntry {
    /// Package name (e.g., com.google.android.youtube)
    pub name: String,
    /// Human-readable label
    pub label: String,
    /// Description of what the package does
    pub description: String,
    /// Safety rating
    pub safety: SafetyRating,
    /// Category
    pub category: PackageCategory,
    /// OEMs this package is found on (empty = all)
    #[serde(default)]
    pub oems: Vec<Oem>,
    /// What functionality may be affected by removal
    #[serde(default)]
    pub dependencies: Vec<String>,
    /// Alternative packages that provide similar functionality
    #[serde(default)]
    pub alternatives: Vec<String>,
    /// Tags for filtering (e.g., "privacy", "battery-drain")
    #[serde(default)]
    pub tags: Vec<String>,
    /// Community votes (upvotes - downvotes)
    #[serde(default)]
    pub votes: i32,
}

impl PackageEntry {
    /// Create a new package entry
    pub fn new(
        name: impl Into<String>,
        label: impl Into<String>,
        description: impl Into<String>,
        safety: SafetyRating,
        category: PackageCategory,
    ) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            description: description.into(),
            safety,
            category,
            oems: Vec::new(),
            dependencies: Vec::new(),
            alternatives: Vec::new(),
            tags: Vec::new(),
            votes: 0,
        }
    }

    /// Check if safe to remove (Recommended or Advanced)
    pub fn is_safe_to_remove(&self) -> bool {
        matches!(
            self.safety,
            SafetyRating::Recommended | SafetyRating::Advanced
        )
    }
}

/// The package database
#[derive(Debug, Clone, Default)]
pub struct PackageDatabase {
    /// All packages indexed by name
    packages: HashMap<String, PackageEntry>,
    /// Index by OEM
    by_oem: HashMap<Oem, Vec<String>>,
    /// Index by category
    by_category: HashMap<PackageCategory, Vec<String>>,
    /// Index by safety rating
    by_safety: HashMap<SafetyRating, Vec<String>>,
}

impl PackageDatabase {
    /// Create an empty database
    pub fn new() -> Self {
        Self::default()
    }

    /// Load database from JSON files in a directory
    pub fn load_from_dir(dir: &Path) -> Result<Self> {
        let mut db = Self::new();

        if !dir.exists() {
            return Err(Error::config(format!(
                "Package database directory not found: {:?}",
                dir
            )));
        }

        for entry in std::fs::read_dir(dir)
            .map_err(|e| Error::config(format!("Failed to read database dir: {}", e)))?
        {
            let entry =
                entry.map_err(|e| Error::config(format!("Failed to read dir entry: {}", e)))?;
            let path = entry.path();

            if path.extension().map(|e| e == "json").unwrap_or(false) {
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| Error::config(format!("Failed to read {:?}: {}", path, e)))?;

                let packages: Vec<PackageEntry> = serde_json::from_str(&content)?;

                for package in packages {
                    db.add(package);
                }
            }
        }

        Ok(db)
    }

    /// Add a package to the database
    pub fn add(&mut self, entry: PackageEntry) {
        let name = entry.name.clone();

        // Add to OEM index
        if entry.oems.is_empty() {
            // Add to all OEMs if not specified
            for oem in [
                Oem::Samsung,
                Oem::Xiaomi,
                Oem::Huawei,
                Oem::OnePlus,
                Oem::Oppo,
                Oem::Vivo,
                Oem::Realme,
            ] {
                self.by_oem.entry(oem).or_default().push(name.clone());
            }
        } else {
            for oem in &entry.oems {
                self.by_oem.entry(*oem).or_default().push(name.clone());
            }
        }

        // Add to category index
        self.by_category
            .entry(entry.category)
            .or_default()
            .push(name.clone());

        // Add to safety index
        self.by_safety
            .entry(entry.safety)
            .or_default()
            .push(name.clone());

        // Add to main map
        self.packages.insert(name, entry);
    }

    /// Get a package by name
    pub fn get(&self, name: &str) -> Option<&PackageEntry> {
        self.packages.get(name)
    }

    /// Get all packages
    pub fn all(&self) -> impl Iterator<Item = &PackageEntry> {
        self.packages.values()
    }

    /// Get packages for a specific OEM
    pub fn by_oem(&self, oem: Oem) -> Vec<&PackageEntry> {
        self.by_oem
            .get(&oem)
            .map(|names| names.iter().filter_map(|n| self.packages.get(n)).collect())
            .unwrap_or_default()
    }

    /// Get packages by safety rating
    pub fn by_safety(&self, safety: SafetyRating) -> Vec<&PackageEntry> {
        self.by_safety
            .get(&safety)
            .map(|names| names.iter().filter_map(|n| self.packages.get(n)).collect())
            .unwrap_or_default()
    }

    /// Get packages by category
    pub fn by_category(&self, category: PackageCategory) -> Vec<&PackageEntry> {
        self.by_category
            .get(&category)
            .map(|names| names.iter().filter_map(|n| self.packages.get(n)).collect())
            .unwrap_or_default()
    }

    /// Search packages by name or label
    pub fn search(&self, query: &str) -> Vec<&PackageEntry> {
        let query_lower = query.to_lowercase();
        self.packages
            .values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower)
                    || p.label.to_lowercase().contains(&query_lower)
                    || p.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Get safe packages for removal (Recommended rating)
    pub fn get_safe_packages(&self) -> Vec<&PackageEntry> {
        self.by_safety(SafetyRating::Recommended)
    }

    /// Get bloatware packages (Recommended + Advanced)
    pub fn get_bloatware(&self) -> Vec<&PackageEntry> {
        let mut packages = self.by_safety(SafetyRating::Recommended);
        packages.extend(self.by_safety(SafetyRating::Advanced));
        packages
    }

    /// Get total package count
    pub fn len(&self) -> usize {
        self.packages.len()
    }

    /// Check if database is empty
    pub fn is_empty(&self) -> bool {
        self.packages.is_empty()
    }
}

/// Create a default package database with common packages
pub fn create_default_database() -> PackageDatabase {
    let mut db = PackageDatabase::new();

    // Google packages
    db.add(PackageEntry::new(
        "com.google.android.youtube",
        "YouTube",
        "YouTube video streaming app. Can be replaced with YouTube Vanced or NewPipe.",
        SafetyRating::Recommended,
        PackageCategory::Google,
    ));

    db.add(PackageEntry::new(
        "com.google.android.apps.youtube.music",
        "YouTube Music",
        "YouTube's music streaming service.",
        SafetyRating::Recommended,
        PackageCategory::Google,
    ));

    db.add(PackageEntry::new(
        "com.google.android.apps.maps",
        "Google Maps",
        "Google Maps navigation. Consider alternatives like OsmAnd or Magic Earth.",
        SafetyRating::Advanced,
        PackageCategory::Google,
    ));

    db.add(PackageEntry::new(
        "com.google.android.apps.photos",
        "Google Photos",
        "Photo backup and gallery app.",
        SafetyRating::Advanced,
        PackageCategory::Google,
    ));

    db.add(PackageEntry::new(
        "com.google.android.gm",
        "Gmail",
        "Google email client.",
        SafetyRating::Advanced,
        PackageCategory::Google,
    ));

    db.add(PackageEntry::new(
        "com.google.android.apps.docs",
        "Google Drive",
        "Cloud storage and file sync.",
        SafetyRating::Recommended,
        PackageCategory::Google,
    ));

    // Facebook packages
    db.add(PackageEntry::new(
        "com.facebook.system",
        "Facebook System",
        "Facebook system integration. Pre-installed on many devices.",
        SafetyRating::Recommended,
        PackageCategory::Social,
    ));

    db.add(PackageEntry::new(
        "com.facebook.appmanager",
        "Facebook App Manager",
        "Manages Facebook app installations and updates.",
        SafetyRating::Recommended,
        PackageCategory::Social,
    ));

    db.add(PackageEntry::new(
        "com.facebook.services",
        "Facebook Services",
        "Background services for Facebook apps.",
        SafetyRating::Recommended,
        PackageCategory::Social,
    ));

    // Samsung packages
    let mut samsung_bixby = PackageEntry::new(
        "com.samsung.android.bixby.agent",
        "Bixby Voice",
        "Samsung's voice assistant.",
        SafetyRating::Recommended,
        PackageCategory::Generic,
    );
    samsung_bixby.oems = vec![Oem::Samsung];
    db.add(samsung_bixby);

    let mut samsung_pay = PackageEntry::new(
        "com.samsung.android.spay",
        "Samsung Pay",
        "Samsung's mobile payment service.",
        SafetyRating::Advanced,
        PackageCategory::Generic,
    );
    samsung_pay.oems = vec![Oem::Samsung];
    db.add(samsung_pay);

    // Xiaomi packages
    let mut mi_browser = PackageEntry::new(
        "com.mi.globalbrowser",
        "Mi Browser",
        "Xiaomi's built-in browser with ads.",
        SafetyRating::Recommended,
        PackageCategory::Generic,
    );
    mi_browser.oems = vec![Oem::Xiaomi];
    mi_browser.tags = vec!["ads".to_string(), "privacy".to_string()];
    db.add(mi_browser);

    let mut mi_video = PackageEntry::new(
        "com.miui.videoplayer",
        "Mi Video",
        "Xiaomi video player with ads.",
        SafetyRating::Recommended,
        PackageCategory::Generic,
    );
    mi_video.oems = vec![Oem::Xiaomi];
    db.add(mi_video);

    db
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_operations() {
        let db = create_default_database();

        assert!(!db.is_empty());
        assert!(db.get("com.google.android.youtube").is_some());

        let safe = db.get_safe_packages();
        assert!(!safe.is_empty());

        let results = db.search("youtube");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_safety_rating_order() {
        assert!(SafetyRating::Recommended < SafetyRating::Advanced);
        assert!(SafetyRating::Advanced < SafetyRating::Unsafe);
        assert!(SafetyRating::Unsafe < SafetyRating::Danger);
    }
}
