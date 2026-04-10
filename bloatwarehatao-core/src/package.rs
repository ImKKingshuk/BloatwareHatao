//! Package operations module
//!
//! Provides functionality for listing, uninstalling, disabling, and enabling packages.

use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, warn};

use crate::adb::Adb;
use crate::{Error, Result};

/// Extract a meaningful app name from a package name
/// 
/// Skips generic suffixes like "android", "app", "mobile", etc.
/// Examples:
/// - `com.linkedin.android` → "Linkedin"
/// - `com.spotify.music` → "Spotify"  
/// - `com.google.android.apps.maps` → "Maps"
pub fn extract_app_name(package_name: &str) -> String {
    let parts: Vec<&str> = package_name.split('.').collect();
    
    // Generic suffixes to skip (case-insensitive)
    let skip_suffixes = [
        "android", "app", "mobile", "lite", "pro", "free", "premium",
        "client", "main", "base", "core", "sdk", "service", "services",
        "provider", "stub", "overlay", "res", "resources", "ui", "launcher",
        "v2", "v3", "beta", "debug", "release", "internal", "test",
    ];
    
    // Common prefixes to skip
    let skip_prefixes = ["com", "org", "net", "io", "me", "in", "co", "tv", "app"];
    
    // Find the best segment (work backwards, skip generic ones)
    let mut best_segment: Option<&str> = None;
    
    for part in parts.iter().rev() {
        let lower = part.to_lowercase();
        
        // Skip if it's a generic suffix
        if skip_suffixes.contains(&lower.as_str()) {
            continue;
        }
        
        // Skip if it's a prefix (only if we haven't found anything yet)
        if skip_prefixes.contains(&lower.as_str()) && best_segment.is_some() {
            break;
        }
        
        // Skip very short segments (likely abbreviations)
        if part.len() < 3 && best_segment.is_some() {
            continue;
        }
        
        // Skip segments that are all numbers or hex-like (like hashes)
        if part.chars().all(|c| c.is_ascii_hexdigit() || c == '_') && part.len() > 8 {
            continue;
        }
        
        best_segment = Some(part);
    }
    
    // Format the segment nicely (capitalize first letter)
    let segment = best_segment.unwrap_or(parts.last().unwrap_or(&package_name));
    
    // Handle camelCase and underscores -> spaces, then title case
    let mut result = String::new();
    let mut prev_was_lower = false;
    
    for c in segment.chars() {
        if c == '_' || c == '-' {
            result.push(' ');
            prev_was_lower = false;
        } else if c.is_uppercase() && prev_was_lower {
            result.push(' ');
            result.push(c);
            prev_was_lower = false;
        } else {
            result.push(c);
            prev_was_lower = c.is_lowercase();
        }
    }
    
    // Title case the result
    result
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Package removal mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RemovalMode {
    /// Uninstall package for current user (recommended)
    #[default]
    Uninstall,
    /// Disable package (can be re-enabled)
    Disable,
    /// Clear package data only
    Clear,
}

impl RemovalMode {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Uninstall => "Uninstall",
            Self::Disable => "Disable",
            Self::Clear => "Clear Data",
        }
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Uninstall => "Remove app for current user (reversible via factory reset)",
            Self::Disable => "Disable app without removing (easily reversible)",
            Self::Clear => "Clear app data and cache only",
        }
    }
}

/// Package information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// Package name (e.g., com.google.android.youtube)
    pub name: String,
    /// Whether the package is currently installed
    pub installed: bool,
    /// Whether it's a system app
    pub is_system: bool,
    /// Whether the package is enabled
    pub enabled: bool,
    /// APK path on device
    pub path: Option<String>,
    /// Version name
    pub version_name: Option<String>,
    /// Version code
    pub version_code: Option<i64>,
}

impl Package {
    /// Create from package name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            installed: true,
            is_system: false,
            enabled: true,
            path: None,
            version_name: None,
            version_code: None,
        }
    }
}

/// Result of a package operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    /// Package name
    pub package: String,
    /// Whether the operation succeeded
    pub success: bool,
    /// Operation performed
    pub operation: String,
    /// Error message if failed
    pub error: Option<String>,
}

/// Package manager for performing operations
#[derive(Debug)]
pub struct PackageManager {
    adb: Adb,
    dry_run: bool,
}

impl PackageManager {
    /// Create a new package manager
    pub fn new(adb: Adb) -> Self {
        Self { adb, dry_run: false }
    }

    /// Enable dry run mode (no actual changes)
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// List all installed packages
    #[instrument(skip(self))]
    pub async fn list_packages(&self) -> Result<Vec<String>> {
        let output = self.adb.shell("pm list packages").await?;
        let packages: Vec<String> = output
            .lines()
            .filter_map(|line| line.strip_prefix("package:"))
            .map(|s| s.trim().to_string())
            .collect();
        
        debug!("Found {} installed packages", packages.len());
        Ok(packages)
    }

    /// List third-party (user-installed) packages
    #[instrument(skip(self))]
    pub async fn list_third_party_packages(&self) -> Result<Vec<String>> {
        let output = self.adb.shell("pm list packages -3").await?;
        let packages: Vec<String> = output
            .lines()
            .filter_map(|line| line.strip_prefix("package:"))
            .map(|s| s.trim().to_string())
            .collect();
        
        debug!("Found {} third-party packages", packages.len());
        Ok(packages)
    }

    /// List system packages
    #[instrument(skip(self))]
    pub async fn list_system_packages(&self) -> Result<Vec<String>> {
        let output = self.adb.shell("pm list packages -s").await?;
        let packages: Vec<String> = output
            .lines()
            .filter_map(|line| line.strip_prefix("package:"))
            .map(|s| s.trim().to_string())
            .collect();
        
        debug!("Found {} system packages", packages.len());
        Ok(packages)
    }

    /// List disabled packages
    #[instrument(skip(self))]
    pub async fn list_disabled_packages(&self) -> Result<Vec<String>> {
        let output = self.adb.shell("pm list packages -d").await?;
        let packages: Vec<String> = output
            .lines()
            .filter_map(|line| line.strip_prefix("package:"))
            .map(|s| s.trim().to_string())
            .collect();
        
        debug!("Found {} disabled packages", packages.len());
        Ok(packages)
    }

    /// Check if a package is installed
    #[instrument(skip(self))]
    pub async fn is_installed(&self, package: &str) -> Result<bool> {
        let output = self.adb.shell(&format!("pm list packages {}", package)).await?;
        Ok(output.contains(&format!("package:{}", package)))
    }

    /// Get the app label (display name) for a package from the device
    /// Uses multiple approaches to find the real app name
    #[instrument(skip(self))]
    pub async fn get_app_label(&self, package: &str) -> Result<String> {
        // Method 1: Try cmd package dump for applicationInfo (more reliable on newer Android)
        let output = self.adb.shell(&format!(
            "cmd package dump {} 2>/dev/null | grep -E 'labelRes=|applicationInfo=' | head -5",
            package
        )).await.unwrap_or_default();
        
        // Method 2: Try dumpsys package with broader grep patterns
        let dumpsys_output = self.adb.shell(&format!(
            "dumpsys package {} 2>/dev/null | grep -iE 'applicationLabel|label=' | head -5",
            package
        )).await.unwrap_or_default();
        
        // Combine outputs for parsing
        let combined = format!("{}\n{}", output, dumpsys_output);
        
        // Parse the label from output - try multiple formats
        for line in combined.lines() {
            let line = line.trim();
            
            // Format: "applicationLabel=AppName" 
            if let Some(label) = line.strip_prefix("applicationLabel=") {
                let label = label.trim().trim_matches('"');
                if !label.is_empty() && label != "null" && !label.starts_with("0x") {
                    return Ok(label.to_string());
                }
            }
            
            // Format: "Application Label: AppName"
            if let Some(label) = line.strip_prefix("Application Label:") {
                let label = label.trim();
                if !label.is_empty() && label != "null" && !label.starts_with("0x") {
                    return Ok(label.to_string());
                }
            }
            
            // Format: "label=AppName" or 'label="AppName"'
            if line.contains("label=")
                && let Some(idx) = line.find("label=") {
                    let rest = &line[idx + 6..];
                    let label = rest.trim().trim_matches('"').trim_matches('\'');
                    // Skip resource IDs like 0x7f... 
                    if !label.is_empty() && label != "null" && !label.starts_with("0x") && !label.contains(':') {
                        return Ok(label.to_string());
                    }
            }
        }
        
        // Method 3: Try aapt approach via pm dump (gets label from APK manifest)
        let pm_output = self.adb.shell(&format!(
            "pm dump {} 2>/dev/null | grep -A1 'application-label' | head -3",
            package
        )).await.unwrap_or_default();
        
        for line in pm_output.lines() {
            let line = line.trim();
            // format: "application-label:'AppName'"
            if line.contains("application-label") {
                if let Some(start) = line.find('\'')
                    && let Some(end) = line.rfind('\'')
                    && end > start + 1 {
                            let label = &line[start + 1..end];
                            if !label.is_empty() && !label.starts_with("0x") {
                                return Ok(label.to_string());
                            }
                }
                // Also try with colon format: "application-label:AppName"
                if let Some(idx) = line.find(':') {
                    let label = line[idx + 1..].trim().trim_matches('\'').trim_matches('"');
                    if !label.is_empty() && !label.starts_with("0x") {
                        return Ok(label.to_string());
                    }
                }
            }
        }
        
        // Fallback: use smart extraction from package name
        Ok(extract_app_name(package))
    }

    /// Get app labels for multiple packages efficiently
    /// Uses a batch approach to minimize ADB calls
    #[instrument(skip(self, packages))]
    pub async fn get_all_app_labels(&self, packages: &[String]) -> Result<std::collections::HashMap<String, String>> {
        let mut labels = std::collections::HashMap::new();
        
        // For efficiency, we'll try to get labels in batches
        // But for now, get them one by one with smart fallback
        for package in packages {
            let label = self.get_app_label(package).await.unwrap_or_else(|_| {
                extract_app_name(package)
            });
            labels.insert(package.clone(), label);
        }
        
        Ok(labels)
    }

    /// Check if a package is a system app
    #[instrument(skip(self))]
    pub async fn is_system_app(&self, package: &str) -> Result<bool> {
        let output = self.adb.shell(&format!("pm path {}", package)).await?;
        Ok(output.contains("/system/") || output.contains("/product/") || output.contains("/vendor/"))
    }

    /// Get detailed package info
    #[instrument(skip(self))]
    pub async fn get_package_info(&self, package: &str) -> Result<Package> {
        let installed = self.is_installed(package).await?;
        if !installed {
            return Err(Error::PackageNotFound(package.to_string()));
        }

        let is_system = self.is_system_app(package).await?;
        
        // Get path
        let path_output = self.adb.shell(&format!("pm path {}", package)).await?;
        let path = path_output
            .lines()
            .next()
            .and_then(|l| l.strip_prefix("package:"))
            .map(|s| s.trim().to_string());

        // Get version info
        let dumpsys = self.adb.shell(&format!("dumpsys package {} | grep -E 'versionName|versionCode'", package)).await.unwrap_or_default();
        let mut version_name = None;
        let mut version_code = None;

        for line in dumpsys.lines() {
            let line = line.trim();
            if let Some(value) = line.strip_prefix("versionName=") {
                version_name = Some(value.to_string());
            } else if let Some(value) = line.strip_prefix("versionCode=") {
                version_code = value.split_whitespace().next()
                    .and_then(|s| s.parse().ok());
            }
        }

        // Check if enabled
        let disabled = self.list_disabled_packages().await?;
        let enabled = !disabled.contains(&package.to_string());

        Ok(Package {
            name: package.to_string(),
            installed,
            is_system,
            enabled,
            path,
            version_name,
            version_code,
        })
    }

    /// Uninstall a package for the current user
    #[instrument(skip(self))]
    pub async fn uninstall(&self, package: &str) -> Result<OperationResult> {
        if !Self::validate_package_name(package) {
            return Err(Error::InvalidPackageName(package.to_string()));
        }

        if self.dry_run {
            info!("[DRY RUN] Would uninstall: {}", package);
            return Ok(OperationResult {
                package: package.to_string(),
                success: true,
                operation: "uninstall (dry run)".to_string(),
                error: None,
            });
        }

        let output = self.adb.shell(&format!("pm uninstall --user 0 {}", package)).await?;
        let success = output.to_lowercase().contains("success");

        if success {
            info!("Successfully uninstalled: {}", package);
        } else {
            warn!("Failed to uninstall {}: {}", package, output);
        }

        Ok(OperationResult {
            package: package.to_string(),
            success,
            operation: "uninstall".to_string(),
            error: if success { None } else { Some(output) },
        })
    }

    /// Disable a package
    #[instrument(skip(self))]
    pub async fn disable(&self, package: &str) -> Result<OperationResult> {
        if !Self::validate_package_name(package) {
            return Err(Error::InvalidPackageName(package.to_string()));
        }

        if self.dry_run {
            info!("[DRY RUN] Would disable: {}", package);
            return Ok(OperationResult {
                package: package.to_string(),
                success: true,
                operation: "disable (dry run)".to_string(),
                error: None,
            });
        }

        let output = self.adb.shell(&format!("pm disable-user --user 0 {}", package)).await?;
        let success = output.contains("disabled") || output.contains("new state: disabled");

        if success {
            info!("Successfully disabled: {}", package);
        } else {
            warn!("Failed to disable {}: {}", package, output);
        }

        Ok(OperationResult {
            package: package.to_string(),
            success,
            operation: "disable".to_string(),
            error: if success { None } else { Some(output) },
        })
    }

    /// Enable a previously disabled package
    #[instrument(skip(self))]
    pub async fn enable(&self, package: &str) -> Result<OperationResult> {
        if !Self::validate_package_name(package) {
            return Err(Error::InvalidPackageName(package.to_string()));
        }

        if self.dry_run {
            info!("[DRY RUN] Would enable: {}", package);
            return Ok(OperationResult {
                package: package.to_string(),
                success: true,
                operation: "enable (dry run)".to_string(),
                error: None,
            });
        }

        let output = self.adb.shell(&format!("pm enable {}", package)).await?;
        let success = output.contains("enabled") || output.contains("new state: enabled");

        if success {
            info!("Successfully enabled: {}", package);
        } else {
            warn!("Failed to enable {}: {}", package, output);
        }

        Ok(OperationResult {
            package: package.to_string(),
            success,
            operation: "enable".to_string(),
            error: if success { None } else { Some(output) },
        })
    }

    /// Clear package data
    #[instrument(skip(self))]
    pub async fn clear(&self, package: &str) -> Result<OperationResult> {
        if !Self::validate_package_name(package) {
            return Err(Error::InvalidPackageName(package.to_string()));
        }

        if self.dry_run {
            info!("[DRY RUN] Would clear data: {}", package);
            return Ok(OperationResult {
                package: package.to_string(),
                success: true,
                operation: "clear (dry run)".to_string(),
                error: None,
            });
        }

        let output = self.adb.shell(&format!("pm clear {}", package)).await?;
        let success = output.to_lowercase().contains("success");

        if success {
            info!("Successfully cleared data: {}", package);
        } else {
            warn!("Failed to clear data {}: {}", package, output);
        }

        Ok(OperationResult {
            package: package.to_string(),
            success,
            operation: "clear".to_string(),
            error: if success { None } else { Some(output) },
        })
    }

    /// Reinstall a previously uninstalled system package
    #[instrument(skip(self))]
    pub async fn reinstall(&self, package: &str) -> Result<OperationResult> {
        if !Self::validate_package_name(package) {
            return Err(Error::InvalidPackageName(package.to_string()));
        }

        if self.dry_run {
            info!("[DRY RUN] Would reinstall: {}", package);
            return Ok(OperationResult {
                package: package.to_string(),
                success: true,
                operation: "reinstall (dry run)".to_string(),
                error: None,
            });
        }

        let output = self.adb.shell(&format!("cmd package install-existing --user 0 {}", package)).await?;
        let success = output.contains("installed") || output.contains("success");

        if success {
            info!("Successfully reinstalled: {}", package);
        } else {
            warn!("Failed to reinstall {}: {}", package, output);
        }

        Ok(OperationResult {
            package: package.to_string(),
            success,
            operation: "reinstall".to_string(),
            error: if success { None } else { Some(output) },
        })
    }

    /// Remove a package using the specified mode
    #[instrument(skip(self))]
    pub async fn remove(&self, package: &str, mode: RemovalMode) -> Result<OperationResult> {
        match mode {
            RemovalMode::Uninstall => self.uninstall(package).await,
            RemovalMode::Disable => self.disable(package).await,
            RemovalMode::Clear => self.clear(package).await,
        }
    }

    /// Batch remove multiple packages
    #[instrument(skip(self, packages))]
    pub async fn batch_remove(
        &self,
        packages: &[String],
        mode: RemovalMode,
    ) -> Vec<OperationResult> {
        let mut results = Vec::with_capacity(packages.len());

        for package in packages {
            match self.remove(package, mode).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    results.push(OperationResult {
                        package: package.clone(),
                        success: false,
                        operation: format!("{:?}", mode),
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        results
    }

    /// Validate package name format
    pub fn validate_package_name(package: &str) -> bool {
        if package.is_empty() || package.len() > 255 {
            return false;
        }

        // Must contain at least one dot
        if !package.contains('.') {
            return false;
        }

        // Each segment must start with a letter
        for segment in package.split('.') {
            if segment.is_empty() {
                return false;
            }
            let first_char = segment.chars().next().unwrap();
            if !first_char.is_ascii_lowercase() {
                return false;
            }
            // Only lowercase letters, digits, and underscores
            if !segment.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_package_name() {
        assert!(PackageManager::validate_package_name("com.google.android.youtube"));
        assert!(PackageManager::validate_package_name("com.samsung.android.app.notes"));
        assert!(PackageManager::validate_package_name("com.example.app_123"));
        
        assert!(!PackageManager::validate_package_name(""));
        assert!(!PackageManager::validate_package_name("nopackage"));
        assert!(!PackageManager::validate_package_name("com.Google.App")); // uppercase
        assert!(!PackageManager::validate_package_name("com.123.app")); // starts with number
        assert!(!PackageManager::validate_package_name("com..app")); // empty segment
    }
}
