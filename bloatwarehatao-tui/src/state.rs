
//! Application state management
//!
//! Shared state for the TUI application with device and package data.

use bloatwarehatao_core::rescue::{RescueEntry, RescueSession, RescueManager};
use bloatwarehatao_core::config::{Config, ConfigManager};
use bloatwarehatao_core::database::{PackageDatabase, PackageEntry, SafetyRating, PackageCategory};
use bloatwarehatao_core::device::{Device, DeviceHealth};
use bloatwarehatao_core::preset::{Preset, PresetManager};
use bloatwarehatao_core::package::{RemovalMode, extract_app_name};

use crate::screens::{UserGuideState, PresetCreatorState};

/// Device connection state
#[derive(Debug, Clone)]
pub enum DeviceState {
    /// Not checked yet
    Unknown,
    /// Checking for devices
    Checking,
    /// No device connected
    NotConnected,
    /// Device connected but not authorized
    Unauthorized,
    /// Device ready
    Connected(DeviceInfo),
    /// Error occurred
    Error(String),
}

/// Device information for display
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub serial: String,
    pub model: String,
    pub brand: String,
    pub android_version: String,
    pub sdk_version: u32,
    pub oem: String,
}

impl From<Device> for DeviceInfo {
    fn from(d: Device) -> Self {
        let oem = d.detect_oem().display_name().to_string();
        Self {
            serial: d.serial,
            model: d.model,
            brand: d.brand,
            android_version: d.android_version,
            sdk_version: d.sdk_version,
            oem,
        }
    }
}

/// Package with combined database and device info
#[derive(Debug, Clone)]
pub struct PackageItem {
    /// Package name (e.g., com.google.android.youtube)
    pub name: String,
    /// Display label (e.g., YouTube)
    pub label: String,
    /// Description
    pub description: String,
    /// Safety rating
    pub safety: SafetyRating,
    /// Category
    pub category: PackageCategory,
    /// Is this from the device (installed)?
    pub installed: bool,
    /// Is this in the database?
    #[allow(dead_code)]
    pub in_database: bool,
    /// Is this selected for operation?
    pub selected: bool,
    /// Is this a system app?
    pub is_system: bool,
}

impl PackageItem {
    /// Create from database entry
    pub fn from_db_entry(entry: &PackageEntry) -> Self {
        Self {
            name: entry.name.clone(),
            label: entry.label.clone(),
            description: entry.description.clone(),
            safety: entry.safety,
            category: entry.category,
            installed: false,
            in_database: true,
            selected: false,
            is_system: false,
        }
    }

    /// Create from installed package name (unknown in database)
    pub fn from_installed(name: String, is_system: bool) -> Self {
        // Use bh-core's smart label extraction
        let label = extract_app_name(&name);

        let (safety, category, description) = if is_system {
            (
                SafetyRating::Danger,
                PackageCategory::Other,
                "Unknown system package".to_string(),
            )
        } else {
            (
                SafetyRating::User,
                PackageCategory::UserInstalled,
                "User installed application".to_string(),
            )
        };

        Self {
            name,
            label,
            description,
            safety,
            category,
            installed: true,
            in_database: false,
            selected: false,
            is_system,
        }
    }

    /// Get safety color name for UI
    #[allow(dead_code)]
    pub fn safety_color(&self) -> &'static str {
        self.safety.color_hint()
    }
}

/// Status filter tab for package browser
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusTab {
    #[default]
    All,
    Installed,
    Disabled,
    Uninstalled,
}

impl StatusTab {
    pub fn label(&self) -> &'static str {
        match self {
            StatusTab::All => "All Apps",
            StatusTab::Installed => "Installed",
            StatusTab::Disabled => "Disabled",
            StatusTab::Uninstalled => "Uninstalled",
        }
    }
    
    pub fn next(&self) -> Self {
        match self {
            StatusTab::All => StatusTab::Installed,
            StatusTab::Installed => StatusTab::Disabled,
            StatusTab::Disabled => StatusTab::Uninstalled,
            StatusTab::Uninstalled => StatusTab::All,
        }
    }
    
    pub fn prev(&self) -> Self {
        match self {
            StatusTab::All => StatusTab::Uninstalled,
            StatusTab::Installed => StatusTab::All,
            StatusTab::Disabled => StatusTab::Installed,
            StatusTab::Uninstalled => StatusTab::Disabled,
        }
    }
}

/// Filter criteria for package list
#[derive(Debug, Clone, Default)]
pub struct PackageFilter {
    /// Search query
    pub search: String,
    /// Status tab filter
    pub status_tab: StatusTab,
    /// Show only installed packages (legacy, replaced by status_tab)
    pub installed_only: bool,
    /// Filter by safety rating
    pub safety: Option<SafetyRating>,
    /// Filter by category
    pub category: Option<PackageCategory>,
    /// Show system apps
    pub show_system: bool,
}

impl PackageFilter {
    pub fn matches(&self, pkg: &PackageItem) -> bool {
        // Status tab filter
        match self.status_tab {
            StatusTab::All => {} // Show all
            StatusTab::Installed => {
                if !pkg.installed {
                    return false;
                }
            }
            StatusTab::Disabled => {
                // For now, we don't have a "disabled" field, so skip this filter
                // In the future, we could add a `disabled` field to PackageItem
                // For now, show nothing in disabled tab (or implement later)
                return false; // TODO: implement when we have disabled status
            }
            StatusTab::Uninstalled => {
                if pkg.installed {
                    return false;
                }
            }
        }

        // Installed filter (legacy)
        if self.installed_only && !pkg.installed {
            return false;
        }

        // System filter
        if !self.show_system && pkg.is_system {
            return false;
        }

        // Safety filter
        // Safety filter
        if let Some(safety) = self.safety
            && pkg.safety != safety {
                return false;
        }

        // Category filter
        // Category filter
        if let Some(category) = self.category
            && pkg.category != category {
                return false;
        }

        // Search filter
        if !self.search.is_empty() {
            let query = self.search.to_lowercase();
            let name_match = pkg.name.to_lowercase().contains(&query);
            let label_match = pkg.label.to_lowercase().contains(&query);
            if !name_match && !label_match {
                return false;
            }
        }

        true
    }
}

/// Selection mode for package browser
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// Single selection (for viewing details)
    Single,
    /// Multi-selection (for batch operations)
    Multi,
}

/// Package browser state
#[derive(Debug, Clone)]
pub struct PackageBrowserState {
    /// All packages (merged from device + database)
    pub packages: Vec<PackageItem>,
    /// Filtered package indices
    pub filtered_indices: Vec<usize>,
    /// Current cursor position in filtered list
    pub cursor: usize,
    /// Vertical scroll offset
    pub scroll_offset: usize,
    /// Current filter
    pub filter: PackageFilter,
    /// Selection mode
    pub selection_mode: SelectionMode,
    /// Search input active
    pub search_active: bool,
    /// Filter panel open
    pub filter_panel_open: bool,
    /// Is loading
    pub loading: bool,
    /// Status message
    pub status: Option<String>,
    /// Background label fetching in progress
    pub fetching_labels: bool,
    /// Number of labels fetched so far
    pub labels_fetched: usize,
    /// Total labels to fetch
    pub labels_total: usize,
}

impl Default for PackageBrowserState {
    fn default() -> Self {
        Self {
            packages: Vec::new(),
            filtered_indices: Vec::new(),
            cursor: 0,
            scroll_offset: 0,
            filter: PackageFilter {
                show_system: true,
                ..Default::default()
            },
            selection_mode: SelectionMode::Single,
            search_active: false,
            filter_panel_open: false,
            loading: false,
            status: None,
            fetching_labels: false,
            labels_fetched: 0,
            labels_total: 0,
        }
    }
}

impl PackageBrowserState {
    /// Get the currently selected package
    pub fn selected_package(&self) -> Option<&PackageItem> {
        self.filtered_indices
            .get(self.cursor)
            .and_then(|&idx| self.packages.get(idx))
    }

    /// Get mutable reference to currently selected package
    pub fn selected_package_mut(&mut self) -> Option<&mut PackageItem> {
        let idx = self.filtered_indices.get(self.cursor).copied();
        idx.and_then(move |i| self.packages.get_mut(i))
    }

    /// Toggle selection on current package
    pub fn toggle_selection(&mut self) {
        if let Some(pkg) = self.selected_package_mut() {
            pkg.selected = !pkg.selected;
        }
    }

    /// Get all selected packages
    #[allow(dead_code)]
    pub fn get_selected(&self) -> Vec<&PackageItem> {
        self.packages.iter().filter(|p| p.selected).collect()
    }

    /// Get count of selected packages
    pub fn selected_count(&self) -> usize {
        self.packages.iter().filter(|p| p.selected).count()
    }

    /// Clear all selections
    pub fn clear_selection(&mut self) {
        for pkg in &mut self.packages {
            pkg.selected = false;
        }
    }

    /// Apply filter and update filtered_indices
    pub fn apply_filter(&mut self) {
        self.filtered_indices = self
            .packages
            .iter()
            .enumerate()
            .filter(|(_, pkg)| self.filter.matches(pkg))
            .map(|(i, _)| i)
            .collect();

        // Reset cursor if out of bounds
        if self.cursor >= self.filtered_indices.len() {
            self.cursor = self.filtered_indices.len().saturating_sub(1);
        }
        self.scroll_offset = 0;
    }

    /// Update a package's label by package name
    /// Returns true if the package was found and updated
    pub fn update_package_label(&mut self, package_name: &str, new_label: String) -> bool {
        for pkg in &mut self.packages {
            if pkg.name == package_name {
                pkg.label = new_label;
                return true;
            }
        }
        false
    }

    /// Update multiple package labels at once
    #[allow(dead_code)]
    pub fn update_package_labels(&mut self, labels: &std::collections::HashMap<String, String>) {
        for pkg in &mut self.packages {
            if let Some(label) = labels.get(&pkg.name) {
                pkg.label = label.clone();
            }
        }
        // Re-sort by label after updating
        self.packages.sort_by(|a, b| a.label.to_lowercase().cmp(&b.label.to_lowercase()));
        // Re-apply filter to update indices
        self.apply_filter();
    }

    /// Move cursor up
    pub fn cursor_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor down
    pub fn cursor_down(&mut self) {
        if self.cursor + 1 < self.filtered_indices.len() {
            self.cursor += 1;
        }
    }

    /// Page up
    pub fn page_up(&mut self, page_size: usize) {
        self.cursor = self.cursor.saturating_sub(page_size);
    }

    /// Page down
    pub fn page_down(&mut self, page_size: usize) {
        let max = self.filtered_indices.len().saturating_sub(1);
        self.cursor = (self.cursor + page_size).min(max);
    }

    /// Go to top
    pub fn go_to_top(&mut self) {
        self.cursor = 0;
    }

    /// Go to bottom
    pub fn go_to_bottom(&mut self) {
        self.cursor = self.filtered_indices.len().saturating_sub(1);
    }
}

/// Device health state
#[derive(Debug, Clone, Default)]
pub struct HealthState {
    pub health: Option<DeviceHealth>,
    pub loading: bool,
    pub error: Option<String>,
}

/// Package operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageOperation {
    Uninstall,
    Disable,
    Enable,
    #[allow(dead_code)]
    Reinstall,
    ClearData,
}

impl PackageOperation {
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Uninstall => "Uninstall",
            Self::Disable => "Disable",
            Self::Enable => "Enable",
            Self::Reinstall => "Reinstall",
            Self::ClearData => "Clear Data",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Uninstall => "🗑️",
            Self::Disable => "⏸️",
            Self::Enable => "▶️",
            Self::Reinstall => "🔄",
            Self::ClearData => "🧹",
        }
    }

    pub fn is_destructive(&self) -> bool {
        matches!(self, Self::Uninstall | Self::ClearData)
    }
}

/// Dialog type
#[derive(Debug, Clone)]
pub enum DialogType {
    /// Confirmation dialog with yes/no options
    Confirm {
        title: String,
        message: String,
        operation: PackageOperation,
        packages: Vec<String>,
    },
    /// Progress dialog showing operation status
    Progress {
        title: String,
        current: usize,
        total: usize,
        current_package: String,
    },
    /// Result dialog showing operation results
    Result {
        title: String,
        success: Vec<String>,
        failed: Vec<(String, String)>,
    },
    /// Action menu for selecting operation
    ActionMenu {
        selected: usize,
    },
    /// Error dialog
    Error {
        title: String,
        message: String,
    },
}

/// Dialog state
#[derive(Debug, Clone, Default)]
pub struct DialogState {
    /// Currently active dialog
    pub active: Option<DialogType>,
    /// Selection index for dialogs with options
    pub selected: usize,
}

impl DialogState {
    pub fn show_confirm(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        operation: PackageOperation,
        packages: Vec<String>,
    ) {
        self.active = Some(DialogType::Confirm {
            title: title.into(),
            message: message.into(),
            operation,
            packages,
        });
        self.selected = 1; // Default to "No"
    }

    pub fn show_progress(
        &mut self,
        title: impl Into<String>,
        total: usize,
    ) {
        self.active = Some(DialogType::Progress {
            title: title.into(),
            current: 0,
            total,
            current_package: String::new(),
        });
    }

    pub fn update_progress(&mut self, current: usize, package: impl Into<String>) {
        if let Some(DialogType::Progress { current: c, current_package, .. }) = &mut self.active {
            *c = current;
            *current_package = package.into();
        }
    }

    pub fn show_result(
        &mut self,
        title: impl Into<String>,
        success: Vec<String>,
        failed: Vec<(String, String)>,
    ) {
        self.active = Some(DialogType::Result {
            title: title.into(),
            success,
            failed,
        });
    }

    pub fn show_action_menu(&mut self) {
        self.active = Some(DialogType::ActionMenu { selected: 0 });
        self.selected = 0;
    }

    pub fn show_error(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.active = Some(DialogType::Error {
            title: title.into(),
            message: message.into(),
        });
    }

    pub fn close(&mut self) {
        self.active = None;
        self.selected = 0;
    }

    pub fn is_open(&self) -> bool {
        self.active.is_some()
    }
}

/// Operation state for tracking batch operations
/// Operation state for tracking batch operations
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct OperationState {
    /// Is an operation in progress?
    pub in_progress: bool,
    /// Current operation type
    pub operation: Option<PackageOperation>,
    /// Removal mode for uninstall
    pub removal_mode: RemovalMode,
    /// Packages being operated on
    pub packages: Vec<String>,
    /// Current package index
    pub current_index: usize,
    /// Successful operations
    pub success: Vec<String>,
    /// Failed operations (package name, error message)
    pub failed: Vec<(String, String)>,
}

#[allow(dead_code)]
impl OperationState {
    pub fn start(&mut self, operation: PackageOperation, packages: Vec<String>) {
        self.in_progress = true;
        self.operation = Some(operation);
        self.packages = packages;
        self.current_index = 0;
        self.success.clear();
        self.failed.clear();
    }

    pub fn record_success(&mut self, package: String) {
        self.success.push(package);
        self.current_index += 1;
    }

    pub fn record_failure(&mut self, package: String, error: String) {
        self.failed.push((package, error));
        self.current_index += 1;
    }

    pub fn finish(&mut self) {
        self.in_progress = false;
    }

    pub fn current_package(&self) -> Option<&String> {
        self.packages.get(self.current_index)
    }

    pub fn progress_percent(&self) -> u16 {
        if self.packages.is_empty() {
            return 100;
        }
        ((self.current_index as f32 / self.packages.len() as f32) * 100.0) as u16
    }
}

/// Wireless input field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WirelessField {
    Port,
    Address,
}

/// Wireless ADB state
#[derive(Debug, Clone)]
pub struct WirelessState {
    pub port_input: String,
    pub address_input: String,
    #[allow(dead_code)]
    pub device_ip: Option<String>,
    pub status: Option<String>,
    pub editing: Option<WirelessField>,
}

impl Default for WirelessState {
    fn default() -> Self {
        Self {
            port_input: "5555".to_string(),
            address_input: String::new(),
            device_ip: None,
            status: None,
            editing: None,
        }
    }
}

/// Support item type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportType {
    GitHub,
    Crypto(String), // Ticker
}

/// Support item
#[derive(Debug, Clone)]
pub struct SupportItem {
    pub label: String,
    pub value: String,
    pub type_: SupportType,
}

/// Support screen state
#[derive(Debug, Clone)]
pub struct SupportState {
    pub items: Vec<SupportItem>,
    pub selected: usize,
    pub copy_status: Option<(String, std::time::Instant)>, // Message, Timestamp
}

impl Default for SupportState {
    fn default() -> Self {
        Self {
            items: vec![
                SupportItem {
                    label: "GitHub Sponsors".to_string(),
                    value: "https://github.com/sponsors/ImKKingshuk".to_string(),
                    type_: SupportType::GitHub,
                },
                SupportItem {
                    label: "Bitcoin (BTC)".to_string(),
                    value: "bc1q0kj3ey4d5q4y6t26v28sd7h7cs6k9ghrk8k603".to_string(),
                    type_: SupportType::Crypto("BTC".to_string()),
                },
                SupportItem {
                    label: "Ethereum (EVM)".to_string(),
                    value: "0x2f6C64389D5CC1A7Ad517eFa7bef31a6b2c91157".to_string(),
                    type_: SupportType::Crypto("ETH".to_string()),
                },
                SupportItem {
                    label: "USDT (EVM)".to_string(),
                    value: "0x2f6C64389D5CC1A7Ad517eFa7bef31a6b2c91157".to_string(),
                    type_: SupportType::Crypto("USDT".to_string()),
                },
                SupportItem {
                    label: "BNB (EVM)".to_string(),
                    value: "0x2f6C64389D5CC1A7Ad517eFa7bef31a6b2c91157".to_string(),
                    type_: SupportType::Crypto("BNB".to_string()),
                },
                SupportItem {
                    label: "Solana (SOL)".to_string(),
                    value: "8e8psTV8xTxM8Cfprzn2fdu4FctkK6Up62vepUfG3hvf".to_string(),
                    type_: SupportType::Crypto("SOL".to_string()),
                },
                SupportItem {
                    label: "Toncoin (TON)".to_string(),
                    value: "UQBXXCoqkOIDMyASA6z1WWWuDzW1V2D0QHpgyFXUmNIx-cti".to_string(),
                    type_: SupportType::Crypto("TON".to_string()),
                },
                SupportItem {
                    label: "Dogecoin (DOGE)".to_string(),
                    value: "D72xXqpvqR2UWk1r22BKCjYG5dSTW9N86r".to_string(),
                    type_: SupportType::Crypto("DOGE".to_string()),
                },
                SupportItem {
                    label: "XRP".to_string(),
                    value: "rnMXNWYG2bpJ8e1AL8cLMrpZ8DVzYraMbi".to_string(),
                    type_: SupportType::Crypto("XRP".to_string()),
                },
                SupportItem {
                    label: "Polygon (POL/EVM)".to_string(),
                    value: "0x2f6C64389D5CC1A7Ad517eFa7bef31a6b2c91157".to_string(),
                    type_: SupportType::Crypto("POL".to_string()),
                },
                SupportItem {
                    label: "Avalanche (AVAX/EVM)".to_string(),
                    value: "0x2f6C64389D5CC1A7Ad517eFa7bef31a6b2c91157".to_string(),
                    type_: SupportType::Crypto("AVAX".to_string()),
                },
            ],
            selected: 0,
            copy_status: None,
        }
    }
}

/// Application shared state
#[derive(Debug)]
pub struct AppState {
    /// Device connection state
    pub device: DeviceState,
    /// Package database
    pub database: Option<PackageDatabase>,
    /// Package browser state
    pub browser: PackageBrowserState,
    /// Device health state
    pub health: HealthState,
    /// Wireless ADB state
    pub wireless: WirelessState,
    /// Dialog state
    pub dialog: DialogState,
    /// Operation state
    #[allow(dead_code)]
    pub operation: OperationState,
    /// Settings selected category
    pub settings_selected: usize,
    /// Settings sub-item selected
    pub settings_item_selected: usize,
    /// Presets selected index
    pub presets_selected: usize,
    /// Loaded presets
    pub presets: Vec<Preset>,
    /// Preset manager
    pub preset_manager: Option<PresetManager>,
    /// Rescue tab (0 = rescue points, 1 = sessions)
    pub rescue_tab: usize,
    /// Rescue selected index
    pub rescue_selected: usize,
    /// Loaded rescue entries
    pub rescue_entries: Vec<RescueEntry>,
    /// Loaded rescue sessions
    pub rescue_sessions: Vec<RescueSession>,
    /// Rescue manager
    pub rescue_manager: Option<RescueManager>,
    /// Dry run mode
    pub dry_run: bool,
    /// Configuration
    pub config: Config,
    /// Config manager for saving
    pub config_manager: Option<ConfigManager>,
    /// User Guide state
    pub user_guide: UserGuideState,
    /// Support screen state
    pub support: SupportState,
    /// Preset creator state
    pub preset_creator: PresetCreatorState,
    /// Last error message
    #[allow(dead_code)]
    pub last_error: Option<String>,
}

impl AppState {
    pub fn new(dry_run: bool) -> Self {
        // Try to load config
        let config_manager = ConfigManager::new().ok();
        let config = config_manager
            .as_ref()
            .map(|cm| cm.config().clone())
            .unwrap_or_default();
        
        // Try to load presets and rescue history
        let (preset_manager, rescue_manager, presets, rescue_entries, rescue_sessions) = 
            if let Ok(dirs) = bloatwarehatao_core::config::AppDirs::new() {
                let pm = PresetManager::from_data_dir(dirs.data_dir.as_path());
                let rm = RescueManager::from_data_dir(dirs.data_dir.as_path());
                
                let prsts = pm.get_builtin_presets();
                let entries = rm.list_rescue_history().unwrap_or_default();
                let sessions = rm.list_rescue_sessions().unwrap_or_default();
                
                (Some(pm), Some(rm), prsts, entries, sessions)
            } else {
                (None, None, Vec::new(), Vec::new(), Vec::new())
            };
        
        Self {
            device: DeviceState::Unknown,
            database: None,
            browser: PackageBrowserState::default(),
            health: HealthState::default(),
            wireless: WirelessState::default(),
            dialog: DialogState::default(),
            operation: OperationState::default(),
            settings_selected: 0,
            settings_item_selected: 0,
            presets_selected: 0,
            presets,
            preset_manager,
            rescue_tab: 0,
            rescue_selected: 0,
            rescue_entries,
            rescue_sessions,
            rescue_manager,
            dry_run: dry_run || config.dry_run,
            config,
            config_manager,
            user_guide: UserGuideState::default(),
            support: SupportState::default(),
            preset_creator: PresetCreatorState::default(),
            last_error: None,
        }
    }
    
    /// Reload presets from disk
    pub fn reload_presets(&mut self) {
        if let Some(ref pm) = self.preset_manager {
            self.presets = pm.get_builtin_presets();
            if let Ok(custom) = pm.load_custom_presets() {
                self.presets.extend(custom);
            }
        }
    }
    
    /// Reload rescue history from disk
    pub fn reload_rescue(&mut self) {
        if let Some(ref rm) = self.rescue_manager {
            self.rescue_entries = rm.list_rescue_history().unwrap_or_default();
            self.rescue_sessions = rm.list_rescue_sessions().unwrap_or_default();
        }
    }
    
    /// Get preset by index
    #[allow(dead_code)]
    pub fn get_preset(&self, index: usize) -> Option<&Preset> {
        self.presets.get(index)
    }
    
    /// Get rescue entry by index
    pub fn get_rescue_entry(&self, index: usize) -> Option<&RescueEntry> {
        self.rescue_entries.get(index)
    }

    /// Get rescue session by index
    pub fn get_rescue_session(&self, index: usize) -> Option<&RescueSession> {
        self.rescue_sessions.get(index)
    }
    
    /// Toggle dry run mode
    pub fn toggle_dry_run(&mut self) {
        self.dry_run = !self.dry_run;
        self.config.dry_run = self.dry_run;
    }
    
    /// Toggle auto update check
    pub fn toggle_auto_update(&mut self) {
        self.config.auto_update_check = !self.config.auto_update_check;
    }
    
    /// Toggle offline mode
    pub fn toggle_offline_mode(&mut self) {
        self.config.offline_mode = !self.config.offline_mode;
    }
    
    /// Toggle verbose output
    pub fn toggle_verbose(&mut self) {
        self.config.verbose = !self.config.verbose;
    }
    
    /// Toggle safety warnings
    pub fn toggle_safety_warnings(&mut self) {
        self.config.ui.show_safety_warnings = !self.config.ui.show_safety_warnings;
    }
    
    /// Toggle confirm removal
    pub fn toggle_confirm_removal(&mut self) {
        self.config.ui.confirm_removal = !self.config.ui.confirm_removal;
    }
    
    /// Toggle backup before remove
    pub fn toggle_backup_before_remove(&mut self) {
        self.config.backup_before_remove = !self.config.backup_before_remove;
    }
    
    /// Save configuration to disk
    pub fn save_config(&mut self) -> Result<(), String> {
        if let Some(ref mut cm) = self.config_manager {
            *cm.config_mut() = self.config.clone();
            cm.save().map_err(|e| e.to_string())
        } else {
            Err("Config manager not available".to_string())
        }
    }
    
    /// Apply a preset - select its packages in the browser
    pub fn apply_preset(&mut self, preset_index: usize) -> Option<usize> {
        let preset = self.presets.get(preset_index)?;
        let preset_packages: std::collections::HashSet<_> = 
            preset.packages.iter().map(|s| s.as_str()).collect();
        
        let mut selected_count = 0;
        for pkg in &mut self.browser.packages {
            if preset_packages.contains(pkg.name.as_str()) && pkg.installed {
                pkg.selected = true;
                selected_count += 1;
            }
        }
        
        self.browser.selection_mode = SelectionMode::Multi;
        Some(selected_count)
    }
}
