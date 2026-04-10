//! Preset system module
//!
//! Provides built-in and custom removal presets.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use tracing::info;

use crate::database::SafetyRating;
use crate::Result;

/// A removal preset containing a set of packages to remove
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    /// Unique preset ID
    pub id: String,
    /// Display name
    pub name: String,
    /// Description of what this preset does
    pub description: String,
    /// Preset type
    pub preset_type: PresetType,
    /// Packages to remove
    pub packages: Vec<String>,
    /// Maximum safety rating to include
    #[serde(default)]
    pub max_safety: Option<SafetyRating>,
    /// Categories to target
    #[serde(default)]
    pub categories: Vec<String>,
    /// Tags to filter by
    #[serde(default)]
    pub tags: Vec<String>,
    /// Author (for shared presets)
    pub author: Option<String>,
    /// Version
    pub version: Option<String>,
}

/// Preset type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PresetType {
    /// Built-in preset
    BuiltIn,
    /// User-created preset
    Custom,
    /// Community-shared preset
    Community,
}

impl Preset {
    /// Create a new custom preset
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.into(),
            description: description.into(),
            preset_type: PresetType::Custom,
            packages: Vec::new(),
            max_safety: None,
            categories: Vec::new(),
            tags: Vec::new(),
            author: None,
            version: None,
        }
    }

    /// Create a built-in preset
    pub fn builtin(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        packages: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            preset_type: PresetType::BuiltIn,
            packages,
            max_safety: None,
            categories: Vec::new(),
            tags: Vec::new(),
            author: None,
            version: Some("1.0.0".to_string()),
        }
    }

    /// Add a package to the preset
    pub fn add_package(&mut self, package: impl Into<String>) {
        self.packages.push(package.into());
    }

    /// Add multiple packages
    pub fn add_packages(&mut self, packages: impl IntoIterator<Item = impl Into<String>>) {
        self.packages.extend(packages.into_iter().map(|p| p.into()));
    }

    /// Get unique package set
    pub fn unique_packages(&self) -> HashSet<&str> {
        self.packages.iter().map(|s| s.as_str()).collect()
    }
}

/// Preset manager for loading and saving presets
#[derive(Debug)]
pub struct PresetManager {
    /// Directory for built-in presets
    builtin_dir: PathBuf,
    /// Directory for custom presets
    custom_dir: PathBuf,
}

impl PresetManager {
    /// Create a new preset manager
    pub fn new(builtin_dir: impl Into<PathBuf>, custom_dir: impl Into<PathBuf>) -> Self {
        Self {
            builtin_dir: builtin_dir.into(),
            custom_dir: custom_dir.into(),
        }
    }

    /// Create from a base data directory
    pub fn from_data_dir(data_dir: &Path) -> Self {
        Self {
            builtin_dir: data_dir.join("presets/builtin"),
            custom_dir: data_dir.join("presets/custom"),
        }
    }

    /// Ensure directories exist
    pub fn ensure_dirs(&self) -> Result<()> {
        std::fs::create_dir_all(&self.builtin_dir)?;
        std::fs::create_dir_all(&self.custom_dir)?;
        Ok(())
    }

    /// Get the custom presets directory path
    pub fn custom_dir(&self) -> &Path {
        &self.custom_dir
    }

    /// Get all built-in presets
    pub fn get_builtin_presets(&self) -> Vec<Preset> {
        vec![
            self.privacy_preset(),
            self.performance_preset(),
            self.minimal_preset(),
            self.social_media_preset(),
            self.google_degoogle_preset(),
        ]
    }

    /// Privacy-focused preset
    fn privacy_preset(&self) -> Preset {
        Preset::builtin(
            "privacy",
            "Privacy Focused",
            "Remove apps known for tracking and data collection. Improves privacy while maintaining core functionality.",
            vec![
                "com.facebook.system".to_string(),
                "com.facebook.appmanager".to_string(),
                "com.facebook.services".to_string(),
                "com.google.android.gms.ads".to_string(),
                "com.samsung.android.mobileservice".to_string(),
                "com.miui.analytics".to_string(),
            ],
        )
    }

    /// Performance-focused preset
    fn performance_preset(&self) -> Preset {
        Preset::builtin(
            "performance",
            "Performance Boost",
            "Remove resource-heavy apps that drain battery and slow down your device.",
            vec![
                "com.google.android.apps.youtube.music".to_string(),
                "com.samsung.android.game.gamehome".to_string(),
                "com.miui.videoplayer".to_string(),
                "com.samsung.android.voc".to_string(),
            ],
        )
    }

    /// Minimal preset
    fn minimal_preset(&self) -> Preset {
        Preset::builtin(
            "minimal",
            "Minimal",
            "Keep only essential apps. For users who want a clean, minimal experience.",
            vec![
                "com.google.android.youtube".to_string(),
                "com.google.android.apps.youtube.music".to_string(),
                "com.google.android.apps.maps".to_string(),
                "com.google.android.apps.photos".to_string(),
                "com.google.android.apps.docs".to_string(),
                "com.google.android.gm".to_string(),
            ],
        )
    }

    /// Social media removal preset
    fn social_media_preset(&self) -> Preset {
        Preset::builtin(
            "no-social",
            "No Social Media",
            "Remove pre-installed social media apps and their background services.",
            vec![
                "com.facebook.system".to_string(),
                "com.facebook.appmanager".to_string(),
                "com.facebook.services".to_string(),
                "com.facebook.katana".to_string(),
                "com.instagram.android".to_string(),
                "com.linkedin.android".to_string(),
            ],
        )
    }

    /// De-Google preset
    fn google_degoogle_preset(&self) -> Preset {
        Preset::builtin(
            "degoogle",
            "De-Google",
            "Remove Google apps where possible. Note: Some may affect core functionality.",
            vec![
                "com.google.android.youtube".to_string(),
                "com.google.android.apps.youtube.music".to_string(),
                "com.google.android.apps.maps".to_string(),
                "com.google.android.apps.photos".to_string(),
                "com.google.android.apps.docs".to_string(),
                "com.google.android.gm".to_string(),
                "com.google.android.apps.magazines".to_string(),
                "com.google.android.videos".to_string(),
                "com.google.android.music".to_string(),
            ],
        )
    }

    /// Load custom presets from disk
    pub fn load_custom_presets(&self) -> Result<Vec<Preset>> {
        self.ensure_dirs()?;

        let mut presets = Vec::new();

        for entry in std::fs::read_dir(&self.custom_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "json").unwrap_or(false)
                && let Ok(content) = std::fs::read_to_string(&path)
                && let Ok(preset) = serde_json::from_str::<Preset>(&content) {
                    presets.push(preset);
            }
        }

        Ok(presets)
    }

    /// Save a custom preset
    pub fn save_custom_preset(&self, preset: &Preset) -> Result<PathBuf> {
        self.ensure_dirs()?;

        let filename = format!("{}.json", preset.id);
        let path = self.custom_dir.join(&filename);

        let content = serde_json::to_string_pretty(preset)?;
        std::fs::write(&path, content)?;

        info!("Saved preset: {:?}", path);
        Ok(path)
    }

    /// Create and save a new custom preset
    pub fn create_preset(&self, name: &str, description: &str, packages: Vec<String>) -> Result<Preset> {
        let mut preset = Preset::new(name, description);
        preset.packages = packages;
        self.save_custom_preset(&preset)?;
        Ok(preset)
    }

    /// Delete a custom preset
    pub fn delete_custom_preset(&self, id: &str) -> Result<()> {
        let filename = format!("{}.json", id);
        let path = self.custom_dir.join(&filename);

        if path.exists() {
            std::fs::remove_file(&path)?;
            info!("Deleted preset: {:?}", path);
        }

        Ok(())
    }

    /// Get all presets (built-in + custom)
    pub fn all_presets(&self) -> Result<Vec<Preset>> {
        let mut presets = self.get_builtin_presets();
        presets.extend(self.load_custom_presets()?);
        Ok(presets)
    }

    /// Get preset by ID
    pub fn get_preset(&self, id: &str) -> Result<Option<Preset>> {
        let presets = self.all_presets()?;
        Ok(presets.into_iter().find(|p| p.id == id))
    }

    /// Export a preset to JSON string
    pub fn export_preset(&self, id: &str) -> Result<String> {
        let preset = self.get_preset(id)?
            .ok_or_else(|| crate::Error::preset(format!("Preset not found: {}", id)))?;
        
        serde_json::to_string_pretty(&preset)
            .map_err(|e| crate::Error::preset(format!("Failed to serialize preset: {}", e)))
    }

    /// Import a preset from JSON string
    pub fn import_preset(&self, json_data: &str) -> Result<Preset> {
        let mut preset: Preset = serde_json::from_str(json_data)
            .map_err(|e| crate::Error::preset(format!("Failed to parse preset JSON: {}", e)))?;
        
        // Force the preset type to Custom when importing
        preset.preset_type = PresetType::Custom;
        
        // Generate a new ID to avoid conflicts
        let new_id = format!("imported_{}", uuid::Uuid::new_v4().to_string().split('-').next().unwrap_or("new"));
        preset.id = new_id;
        
        // Save the imported preset
        self.save_custom_preset(&preset)?;
        info!("Imported preset: {} ({})", preset.name, preset.id);
        
        Ok(preset)
    }
}
