//! Application state and main loop

use std::io::{self, Stdout};
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

use bloatwarehatao_core::adb::Adb;
use bloatwarehatao_core::category::PackageCategory;
use bloatwarehatao_core::database::{PackageDatabase, SafetyRating};
use bloatwarehatao_core::device::{Device, DeviceHealth};
use bloatwarehatao_core::package::PackageManager;
use bloatwarehatao_core::preset::PresetType;

use crate::screens::{HomeScreen, PackageBrowserScreen, DeviceInfoScreen, HealthScreen, SettingsScreen, PresetsScreen, RescueScreen, UserGuideScreen, WirelessScreen, AboutScreen, SupportScreen, dialogs};
use crate::state::{AppState, DeviceState, DialogType, PackageItem, PackageOperation, SelectionMode, WirelessField};

/// Application screens
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// Main menu
    Home,
    /// Package browser
    PackageBrowser,
    /// User Guide
    UserGuide,
    /// Device information
    DeviceInfo,
    /// Health check
    Health,
    /// Presets
    Presets,
    /// Rescue History
    Rescue,
    /// Settings
    Settings,
    /// Wireless ADB
    Wireless,
    /// About
    About,
    /// Support/Sponsor
    Support,
    /// Help overlay
    Help,
}

/// Background task messages
#[derive(Debug)]
pub enum Message {

    /// Rescue point created
    RescuePointCreated(Result<bloatwarehatao_core::rescue::RescueEntry, String>),
    /// Rescue restored
    RescueRestored(Result<(usize, usize), String>), // (success, failed)
    /// Session restored
    SessionRestored(Result<(usize, usize), String>), // (success, failed)
    /// Single label updated from background fetch
    LabelUpdate { package_name: String, label: String },
    /// All labels fetched - update count
    LabelProgress { fetched: usize, total: usize },
    /// Background label fetch complete
    LabelsFetchComplete,
}

/// Application state
pub struct App {
    /// Whether the app should quit
    pub should_quit: bool,
    /// Current screen/view
    pub current_screen: Screen,
    /// Previous screen (for back navigation)
    previous_screen: Option<Screen>,
    /// Selected menu index (for home screen)
    pub selected_index: usize,
    /// Status message
    pub status_message: Option<String>,
    /// Shared application state
    pub state: AppState,
    /// Message receiver from background tasks
    rx: mpsc::UnboundedReceiver<Message>,
    /// Message sender for background tasks
    tx: mpsc::UnboundedSender<Message>,
}

impl App {
    /// Create new app instance
    pub fn new(dry_run: bool) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        
        Self {
            should_quit: false,
            current_screen: Screen::Home,
            previous_screen: None,
            selected_index: 0,
            status_message: None,
            state: AppState::new(dry_run),
            rx,
            tx,
        }
    }

    /// Initialize the app (load database, check device)
    pub async fn init(&mut self) -> Result<()> {
        // Load package database
        self.load_database().await?;
        
        // Check for connected device
        self.check_device().await;
        
        Ok(())
    }

    /// Load the package database
    async fn load_database(&mut self) -> Result<()> {
        // Try multiple locations for the packages directory
        // This ensures the DB loads whether run from project root, crate dir, or installed location
        let mut candidates: Vec<PathBuf> = vec![
            PathBuf::from("packages"),  // Relative to CWD (project root)
        ];

        // Add path relative to executable (for installed binaries)
        if let Ok(exe_path) = std::env::current_exe()
            && let Some(exe_dir) = exe_path.parent() {
                candidates.push(exe_dir.join("packages"));
                candidates.push(exe_dir.join("../packages"));
                candidates.push(exe_dir.join("../../packages"));
        }

        // Add path relative to cargo manifest (compile-time, always available in dev)
        let manifest_packages = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("packages");
        candidates.push(manifest_packages);

        for candidate in candidates {
            let canonical = candidate.canonicalize().ok();
            let path_to_check = canonical.as_ref().unwrap_or(&candidate);
            
            if path_to_check.exists() && path_to_check.is_dir() {
                match PackageDatabase::load_from_dir(path_to_check) {
                    Ok(db) => {
                        info!("Loaded {} packages from database at {:?}", db.len(), path_to_check);
                        self.state.database = Some(db);
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Failed to load package database from {:?}: {}", path_to_check, e);
                    }
                }
            }
        }
        
        debug!("No packages directory found in any candidate location");
        Ok(())
    }

    /// Check for connected device
    pub async fn check_device(&mut self) {
        self.state.device = DeviceState::Checking;
        
        let adb = Adb::new();
        
        // Check if ADB is available
        if !adb.is_available().await {
            self.state.device = DeviceState::Error("ADB not found".to_string());
            return;
        }

        // Get list of devices
        match adb.devices().await {
            Ok(devices) => {
                let ready_devices: Vec<_> = devices.iter()
                    .filter(|d| d.status.is_ready())
                    .collect();

                if ready_devices.is_empty() {
                    // Check if any device is unauthorized
                    let unauthorized = devices.iter().any(|d| {
                        matches!(d.status, bloatwarehatao_core::adb::DeviceStatus::Unauthorized)
                    });
                    
                    if unauthorized {
                        self.state.device = DeviceState::Unauthorized;
                    } else {
                        self.state.device = DeviceState::NotConnected;
                    }
                } else {
                    // Use first ready device
                    let device_info = &ready_devices[0];
                    let adb = adb.with_device(&device_info.serial);
                    
                    // Get detailed device info
                    match Device::from_adb(&adb).await {
                        Ok(device) => {
                            self.state.device = DeviceState::Connected(device.into());
                        }
                        Err(e) => {
                            self.state.device = DeviceState::Error(e.to_string());
                        }
                    }
                }
            }
            Err(e) => {
                self.state.device = DeviceState::Error(e.to_string());
            }
        }
    }

    /// Load packages from device and database
    pub async fn load_packages(&mut self) {
        self.state.browser.loading = true;
        self.state.browser.status = Some("Loading packages...".to_string());

        let mut packages = Vec::new();
        
        // Get installed packages from device if connected
        if let DeviceState::Connected(ref info) = self.state.device {
            let adb = Adb::new().with_device(&info.serial);
            let pm = PackageManager::new(adb);
            
            // Get installed and system packages
            match tokio::join!(pm.list_packages(), pm.list_system_packages()) {
                (Ok(installed), Ok(system)) => {
                    for name in installed {
                        let is_sys = system.contains(&name);
                        
                        // Use smart extraction for fast loading (no slow ADB calls per package)
                        let real_label = bloatwarehatao_core::package::extract_app_name(&name);
                        
                        let mut pkg = if let Some(db) = &self.state.database {
                            if let Some(entry) = db.get(&name) {
                                // Start from database entry for safety/category/description
                                let mut item = PackageItem::from_db_entry(entry);
                                // Override with smart extracted label (faster than device query)
                                item.label = real_label;
                                item
                            } else {
                                PackageItem::from_installed(name, is_sys)
                            }
                        } else {
                            let mut item = PackageItem::from_installed(name, is_sys);
                            item.label = real_label;
                            item
                        };
                        pkg.installed = true;
                        pkg.is_system = is_sys;
                        
                        packages.push(pkg);
                    }
                }
                (Err(e), _) | (_, Err(e)) => {
                     self.state.browser.status = Some(format!("Error loading packages: {}", e));
                }
            }
        }
        
        // Add database packages only if NO device is connected
        if !matches!(self.state.device, DeviceState::Connected(_))
            && let Some(db) = &self.state.database {
                for entry in db.all() {
                    packages.push(PackageItem::from_db_entry(entry));
                }
        }

        // Sort by label
        packages.sort_by(|a, b| a.label.to_lowercase().cmp(&b.label.to_lowercase()));

        // Collect package names for background fetching
        let package_names: Vec<String> = packages.iter().map(|p| p.name.clone()).collect();
        let total_packages = package_names.len();

        self.state.browser.packages = packages;
        self.state.browser.apply_filter();
        self.state.browser.loading = false;
        self.state.browser.status = Some(format!(
            "Loaded {} packages (fetching real names...)",
            self.state.browser.filtered_indices.len()
        ));

        // Start background label fetching if device is connected
        if let DeviceState::Connected(ref info) = self.state.device {
            let serial = info.serial.clone();
            let tx = self.tx.clone();
            
            self.state.browser.fetching_labels = true;
            self.state.browser.labels_total = total_packages;
            self.state.browser.labels_fetched = 0;
            
            // Spawn background task to fetch real app labels
            tokio::spawn(async move {
                let adb = Adb::new().with_device(&serial);
                let pm = PackageManager::new(adb);
                
                let mut fetched = 0;
                for package in package_names {
                    // Fetch real label from device
                    if let Ok(label) = pm.get_app_label(&package).await {
                        // Only send update if label is different from smart extraction
                        let smart_label = bloatwarehatao_core::package::extract_app_name(&package);
                        if label != smart_label {
                            let _ = tx.send(Message::LabelUpdate {
                                package_name: package,
                                label,
                            });
                        }
                    }
                    
                    fetched += 1;
                    // Send progress update every 10 packages
                    if fetched % 10 == 0 {
                        let _ = tx.send(Message::LabelProgress {
                            fetched,
                            total: total_packages,
                        });
                    }
                }
                
                // Signal completion
                let _ = tx.send(Message::LabelsFetchComplete);
            });
        }
    }

    /// Load device health
    pub async fn load_health(&mut self) {
        self.state.health.loading = true;
        self.state.health.error = None;

        if let DeviceState::Connected(ref info) = self.state.device {
            let adb = Adb::new().with_device(&info.serial);
            
            match DeviceHealth::from_adb(&adb).await {
                Ok(health) => {
                    self.state.health.health = Some(health);
                }
                Err(e) => {
                    self.state.health.error = Some(e.to_string());
                }
            }
        }
        
        self.state.health.loading = false;
    }

    /// Handle keyboard input
    pub async fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) {
        // Global shortcuts
        if modifiers.contains(KeyModifiers::CONTROL) {
            match key {
                KeyCode::Char('c') | KeyCode::Char('q') => {
                    self.should_quit = true;
                    return;
                }
                _ => {}
            }
        }

        // Handle dialog input first
        if self.state.dialog.is_open() {
            self.handle_dialog_key(key).await;
            return;
        }

        match self.current_screen {
            Screen::Home => self.handle_home_key(key).await,
            Screen::PackageBrowser => self.handle_browser_key(key).await,
            Screen::UserGuide => self.handle_user_guide_key(key).await,
            Screen::DeviceInfo => self.handle_device_info_key(key).await,
            Screen::Health => self.handle_health_key(key).await,
            Screen::Presets => self.handle_presets_key(key),
            Screen::Rescue => self.handle_rescue_key(key),
            Screen::Settings => self.handle_settings_key(key).await,
            Screen::Wireless => self.handle_wireless_key(key).await,
            Screen::About => self.handle_generic_back_key(key),
            Screen::Support => self.handle_support_key(key),
            Screen::Help => self.current_screen = self.previous_screen.unwrap_or(Screen::Home),
        }
    }

    async fn handle_home_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max_index = self.menu_items().len() - 1;
                if self.selected_index < max_index {
                    self.selected_index += 1;
                }
            }
            KeyCode::Enter => {
                self.handle_menu_select().await;
            }
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('?') => {
                self.previous_screen = Some(self.current_screen);
                self.current_screen = Screen::Help;
            }
            KeyCode::Char('r') => {
                self.check_device().await;
            }
            _ => {}
        }
    }

    async fn handle_browser_key(&mut self, key: KeyCode) {
        let browser = &mut self.state.browser;
        
        // If search is active, handle search input
        if browser.search_active {
            match key {
                KeyCode::Esc => {
                    browser.search_active = false;
                }
                KeyCode::Enter => {
                    browser.search_active = false;
                    browser.apply_filter();
                }
                KeyCode::Backspace => {
                    browser.filter.search.pop();
                    browser.apply_filter();
                }
                KeyCode::Char(c) => {
                    browser.filter.search.push(c);
                    browser.apply_filter();
                }
                _ => {}
            }
            return;
        }

        match key {
            KeyCode::Up | KeyCode::Char('k') => browser.cursor_up(),
            KeyCode::Down | KeyCode::Char('j') => browser.cursor_down(),
            KeyCode::PageUp => browser.page_up(10),
            KeyCode::PageDown => browser.page_down(10),
            KeyCode::Home | KeyCode::Char('g') => browser.go_to_top(),
            KeyCode::End | KeyCode::Char('G') => browser.go_to_bottom(),
            KeyCode::Char(' ') => {
                // Toggle selection and switch to multi mode
                browser.selection_mode = SelectionMode::Multi;
                browser.toggle_selection();
            }
            KeyCode::Char('/') => {
                browser.search_active = true;
            }
            KeyCode::Char('c') => {
                // Clear search
                browser.filter.search.clear();
                browser.apply_filter();
            }
            KeyCode::Char('i') => {
                // Toggle installed filter
                browser.filter.installed_only = !browser.filter.installed_only;
                browser.apply_filter();
                browser.status = Some(if browser.filter.installed_only {
                    "Showing installed only".to_string()
                } else {
                    "Showing all packages".to_string()
                });
            }
            KeyCode::Enter => {
                // Show action menu if packages are selected
                let count = self.state.browser.selected_count();
                if count > 0 {
                    self.state.dialog.show_action_menu();
                } else if let Some(pkg) = self.state.browser.selected_package()
                    && pkg.installed {
                        // Select current package and show action menu
                        self.state.browser.selection_mode = SelectionMode::Multi;
                        self.state.browser.toggle_selection();
                        self.state.dialog.show_action_menu();
                }
            }
            KeyCode::Char('u') => {
                // Quick uninstall selected
                self.start_operation(PackageOperation::Uninstall).await;
            }
            KeyCode::Char('d') => {
                // Quick disable selected
                self.start_operation(PackageOperation::Disable).await;
            }
            KeyCode::Char('e') => {
                // Quick enable selected
                self.start_operation(PackageOperation::Enable).await;
            }
            KeyCode::Tab => {
                // Switch to next tab
                browser.filter.status_tab = browser.filter.status_tab.next();
                browser.apply_filter();
                browser.cursor = 0;
            }
            KeyCode::BackTab => {
                // Switch to previous tab
                browser.filter.status_tab = browser.filter.status_tab.prev();
                browser.apply_filter();
                browser.cursor = 0;
            }
            KeyCode::Esc => {
                if browser.selection_mode == SelectionMode::Multi {
                    browser.selection_mode = SelectionMode::Single;
                    browser.clear_selection();
                } else {
                    self.current_screen = Screen::Home;
                }
            }
            KeyCode::Char('q') => {
                self.current_screen = Screen::Home;
            }
            KeyCode::Char('?') => {
                self.previous_screen = Some(self.current_screen);
                self.current_screen = Screen::Help;
            }
            KeyCode::Char('r') => {
                self.load_packages().await;
            }
            KeyCode::Char('f') => {
                // Toggle filter panel
                browser.filter_panel_open = !browser.filter_panel_open;
            }
            // Safety filters (1-4)
            KeyCode::Char('1') => {
                browser.filter.safety = if browser.filter.safety == Some(SafetyRating::Recommended) {
                    None
                } else {
                    Some(SafetyRating::Recommended)
                };
                browser.apply_filter();
                browser.cursor = 0;
            }
            KeyCode::Char('2') => {
                browser.filter.safety = if browser.filter.safety == Some(SafetyRating::Advanced) {
                    None
                } else {
                    Some(SafetyRating::Advanced)
                };
                browser.apply_filter();
                browser.cursor = 0;
            }
            KeyCode::Char('3') => {
                browser.filter.safety = if browser.filter.safety == Some(SafetyRating::Unsafe) {
                    None
                } else {
                    Some(SafetyRating::Unsafe)
                };
                browser.apply_filter();
                browser.cursor = 0;
            }
            KeyCode::Char('4') => {
                browser.filter.safety = if browser.filter.safety == Some(SafetyRating::Danger) {
                    None
                } else {
                    Some(SafetyRating::Danger)
                };
                browser.apply_filter();
                browser.cursor = 0;
            }
            // Category filters (a-z for dynamic categories)
            KeyCode::Char(c @ 'a'..='z') => {
                // Get unique categories from packages
                let mut unique_cats: Vec<PackageCategory> = browser.packages
                    .iter()
                    .map(|p| p.category)
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                unique_cats.sort_by(|a, b| a.display_name().cmp(b.display_name()));
                
                let idx = (c as u8 - b'a') as usize;
                if let Some(&cat) = unique_cats.get(idx) {
                    browser.filter.category = if browser.filter.category == Some(cat) {
                        None
                    } else {
                        Some(cat)
                    };
                    browser.apply_filter();
                    browser.cursor = 0;
                }
            }
            // Clear all filters
            KeyCode::Char('0') => {
                browser.filter.safety = None;
                browser.filter.category = None;
                browser.apply_filter();
                browser.cursor = 0;
            }
            _ => {}
        }
    }

    async fn handle_device_info_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.current_screen = Screen::Home;
            }
            KeyCode::Char('r') => {
                self.check_device().await;
            }
            KeyCode::Char('?') => {
                self.previous_screen = Some(self.current_screen);
                self.current_screen = Screen::Help;
            }
            _ => {}
        }
    }

    async fn handle_health_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.current_screen = Screen::Home;
            }
            KeyCode::Char('r') => {
                self.load_health().await;
            }
            KeyCode::Char('?') => {
                self.previous_screen = Some(self.current_screen);
                self.current_screen = Screen::Help;
            }
            _ => {}
        }
    }

    #[allow(dead_code)]
    fn handle_help_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter | KeyCode::Char('?') => {
                self.current_screen = self.previous_screen.unwrap_or(Screen::Home);
                self.previous_screen = None;
            }
            _ => {}
        }
    }

    fn handle_generic_back_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.current_screen = Screen::Home;
            }
            _ => {}
        }
    }

    fn handle_support_key(&mut self, key: KeyCode) {
        let count = self.state.support.items.len();
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.support.selected > 0 {
                    self.state.support.selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.state.support.selected < count.saturating_sub(1) {
                    self.state.support.selected += 1;
                }
            }
            KeyCode::Enter => {
                // Copy to clipboard
                if let Some(item) = self.state.support.items.get(self.state.support.selected) {
                    match arboard::Clipboard::new() {
                        Ok(mut clipboard) => {
                            if let Err(e) = clipboard.set_text(&item.value) {
                                self.state.support.copy_status = Some((
                                    format!("Failed to copy: {}", e), 
                                    std::time::Instant::now()
                                ));
                            } else {
                                self.state.support.copy_status = Some((
                                    format!("Copied {} to clipboard!", item.label), 
                                    std::time::Instant::now()
                                ));
                            }
                        }
                        Err(e) => {
                           self.state.support.copy_status = Some((
                                format!("Clipboard error: {}", e), 
                                std::time::Instant::now()
                            ));
                        }
                    }
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.state.support.copy_status = None;
                self.current_screen = Screen::Home;
            }
            _ => {}
        }
    }

    async fn handle_settings_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.settings_selected > 0 {
                    self.state.settings_selected -= 1;
                    self.state.settings_item_selected = 0;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.state.settings_selected < 3 {
                    self.state.settings_selected += 1;
                    self.state.settings_item_selected = 0;
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                // Toggle setting based on category and item
                self.toggle_current_setting();
            }
            KeyCode::Char('s') => {
                // Save settings
                match self.state.save_config() {
                    Ok(_) => {
                        self.status_message = Some("✓ Settings saved successfully".to_string());
                    }
                    Err(e) => {
                        self.status_message = Some(format!("✗ Failed to save: {}", e));
                    }
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.current_screen = Screen::Home;
            }
            KeyCode::Char('?') => {
                self.previous_screen = Some(self.current_screen);
                self.current_screen = Screen::Help;
            }
            _ => {}
        }
    }

    fn toggle_current_setting(&mut self) {
        match self.state.settings_selected {
            0 => {
                // General settings
                match self.state.settings_item_selected {
                    0 => self.state.toggle_dry_run(),
                    1 => self.state.toggle_auto_update(),
                    2 => self.state.toggle_offline_mode(),
                    3 => self.state.toggle_verbose(),
                    _ => {}
                }
            }
            1 => {
                // Removal settings
                match self.state.settings_item_selected {
                    0 => {} // Removal mode needs cycling, not toggle
                    1 => self.state.toggle_safety_warnings(),
                    2 => self.state.toggle_confirm_removal(),
                    3 => self.state.toggle_backup_before_remove(),
                    _ => {}
                }
            }
            2 => {
                // Appearance settings - toggle desc/progress/animations
                match self.state.settings_item_selected {
                    0 => {} // Theme needs cycling
                    1 => self.state.config.ui.show_descriptions = !self.state.config.ui.show_descriptions,
                    2 => self.state.config.ui.show_progress = !self.state.config.ui.show_progress,
                    3 => self.state.config.ui.animations = !self.state.config.ui.animations,
                    _ => {}
                }
            }
            _ => {}
        }
        self.status_message = Some("Setting changed. Press 's' to save.".to_string());
    }

    fn handle_presets_key(&mut self, key: KeyCode) {
        if self.state.preset_creator.active {
            self.handle_preset_creator_key(key);
            return;
        }

        let preset_count = self.state.presets.len();
        match key {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.presets_selected > 0 {
                    self.state.presets_selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.state.presets_selected < preset_count.saturating_sub(1) {
                    self.state.presets_selected += 1;
                }
            }
            KeyCode::Enter => {
                // Apply preset - select its packages in browser
                let preset_name = self.state.presets
                    .get(self.state.presets_selected)
                    .map(|p| p.name.clone());
                
                if let Some(count) = self.state.apply_preset(self.state.presets_selected) {
                    if count > 0 {
                        self.status_message = Some(format!(
                            "✓ Applied '{}': {} packages selected. Go to Package Browser to review.",
                            preset_name.unwrap_or_default(),
                            count
                        ));
                    } else {
                        self.status_message = Some(format!(
                            "No installed packages matched preset '{}'",
                            preset_name.unwrap_or_default()
                        ));
                    }
                } else {
                    self.status_message = Some("Failed to apply preset".to_string());
                }
            }
            KeyCode::Char('r') => {
                // Reload presets
                self.state.reload_presets();
                self.status_message = Some("✓ Presets reloaded".to_string());
            }
            KeyCode::Char('n') | KeyCode::Char('+') => {
                // Open Preset Creator
                self.state.preset_creator = crate::screens::PresetCreatorState::default();
                self.state.preset_creator.active = true;
                self.status_message = Some("Create new preset: Enter details".to_string());
            }
            KeyCode::Char('e') => {
                // Export selected preset
                if let Some(preset) = self.state.presets.get(self.state.presets_selected) {
                    let preset_id = preset.id.clone();
                    let preset_name = preset.name.clone();
                    match self.export_preset(&preset_id) {
                        Ok(path) => {
                            self.status_message = Some(format!(
                                "✓ Exported '{}' to: {}",
                                preset_name, path
                            ));
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Export failed: {}", e));
                        }
                    }
                }
            }
            KeyCode::Char('d') => {
                // Delete custom preset (only custom presets can be deleted)
                if let Some(preset) = self.state.presets.get(self.state.presets_selected) {
                    if preset.preset_type == PresetType::Custom {
                        let preset_id = preset.id.clone();
                        let preset_name = preset.name.clone();
                        match self.delete_preset(&preset_id) {
                            Ok(()) => {
                                self.state.reload_presets();
                                self.status_message = Some(format!(
                                    "✓ Deleted preset '{}'",
                                    preset_name
                                ));
                            }
                            Err(e) => {
                                self.status_message = Some(format!("Delete failed: {}", e));
                            }
                        }
                    } else {
                        self.status_message = Some("Cannot delete built-in presets".to_string());
                    }
                }
            }
            KeyCode::Char('i') => {
                // Import preset - prompt for path (simplified: use default location)
                self.status_message = Some("Import: Place JSON file in ~/.config/bloatwarehatao/presets/custom/ and press 'r' to reload".to_string());
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.current_screen = Screen::Home;
            }
            KeyCode::Char('?') => {
                self.previous_screen = Some(self.current_screen);
                self.current_screen = Screen::Help;
            }
            _ => {}
        }
    }

    fn handle_rescue_key(&mut self, key: KeyCode) {
        let rescue_count = self.state.rescue_entries.len();
        match key {
            KeyCode::Tab => {
                self.state.rescue_tab = (self.state.rescue_tab + 1) % 2;
                self.state.rescue_selected = 0;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.rescue_selected > 0 {
                    self.state.rescue_selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.state.rescue_selected < rescue_count.saturating_sub(1) {
                    self.state.rescue_selected += 1;
                }
            }
            KeyCode::Char('n') => {
                // Create new rescue point
                self.create_rescue_point();
            }
            KeyCode::Char('r') => {
                // Reload rescue history
                self.state.reload_rescue();
                self.status_message = Some(format!(
                    "✓ Reloaded {} rescue points",
                    self.state.rescue_entries.len()
                ));
            }
            KeyCode::Enter => {
                // Restore selected
                if self.state.rescue_tab == 0 {
                    // Restore from Rescue Entry
                    if let Some(entry) = self.state.get_rescue_entry(self.state.rescue_selected) {
                        self.restore_rescue_entry(entry.clone());
                    }
                } else {
                    // Restore from Rescue Session
                    if let Some(session) = self.state.get_rescue_session(self.state.rescue_selected) {
                        self.restore_rescue_session(session.clone());
                    }
                }
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                self.current_screen = Screen::Home;
            }
            KeyCode::Char('?') => {
                self.previous_screen = Some(self.current_screen);
                self.current_screen = Screen::Help;
            }
            _ => {}
        }
    }

    fn handle_preset_creator_key(&mut self, key: KeyCode) {
        use crate::screens::PresetCreationStep;
        
        // Scope to allow borrowing creator, then saving
        let mut save_needed = false;

        {
            let creator = &mut self.state.preset_creator;
            
            match creator.step {
                PresetCreationStep::NameInput => {
                    match key {
                        KeyCode::Enter => {
                            if !creator.name_input.trim().is_empty() {
                                creator.step = PresetCreationStep::DescriptionInput;
                            } else {
                                self.status_message = Some("Name cannot be empty".to_string());
                            }
                        }
                        KeyCode::Backspace => { creator.name_input.pop(); }
                        KeyCode::Char(c) => { creator.name_input.push(c); }
                        KeyCode::Esc => { creator.active = false; }
                        _ => {}
                    }
                }
                PresetCreationStep::DescriptionInput => {
                    match key {
                        KeyCode::Enter => {
                            creator.step = PresetCreationStep::PackageSelection;
                            // Initialize available packages from browser items
                            // Check if browser items are loaded, otherwise load logic might be needed
                            // For now assume browser items are populated (they usually are on startup)
                            if self.state.browser.packages.is_empty() {
                                // Fallback or warning
                            }
                            creator.available_packages = self.state.browser.packages.iter().map(|p| p.name.clone()).collect();
                            creator.list_state.borrow_mut().select(Some(0));
                        }
                        KeyCode::Backspace => { creator.description_input.pop(); }
                        KeyCode::Char(c) => { creator.description_input.push(c); }
                        KeyCode::Esc => { creator.step = PresetCreationStep::NameInput; }
                        _ => {}
                    }
                }
                PresetCreationStep::PackageSelection => {
                    let count = creator.available_packages.len();
                    let mut list_state = creator.list_state.borrow_mut();
                    let selected_idx = list_state.selected().unwrap_or(0);
                    
                    match key {
                        KeyCode::Up | KeyCode::Char('k') => {
                            if selected_idx > 0 {
                                list_state.select(Some(selected_idx - 1));
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if selected_idx < count.saturating_sub(1) {
                                list_state.select(Some(selected_idx + 1));
                            }
                        }
                        KeyCode::Char(' ') => {
                            if let Some(pkg) = creator.available_packages.get(selected_idx) {
                                if creator.selected_packages.contains(pkg) {
                                    creator.selected_packages.remove(pkg);
                                } else {
                                    creator.selected_packages.insert(pkg.clone());
                                }
                            }
                        }
                        KeyCode::Enter => {
                            save_needed = true;
                        }
                        KeyCode::Esc => { 
                            creator.step = PresetCreationStep::DescriptionInput;
                        }
                        _ => {}
                    }
                }
                PresetCreationStep::Confirm => {
                    // Not used currently, skip
                }
            }
        }
        
        if save_needed {
            self.save_created_preset();
        }
    }

    fn save_created_preset(&mut self) {
        let creator = &mut self.state.preset_creator;
        let name = creator.name_input.clone();
        let desc = creator.description_input.clone();
        let packages: Vec<String> = creator.selected_packages.iter().cloned().collect();
        
        if packages.is_empty() {
            self.status_message = Some("Cannot save preset without packages".to_string());
            return;
        }

        if let Some(pm) = &self.state.preset_manager {
            match pm.create_preset(&name, &desc, packages) {
                Ok(_) => {
                    self.status_message = Some(format!("✓ Preset '{}' created", name));
                    // Reload presets
                    if let Ok(presets) = pm.all_presets() {
                         self.state.presets = presets;
                    }
                    creator.active = false;
                }
                Err(e) => {
                    self.state.dialog.show_error("Failed to create preset", e.to_string());
                }
            }
        } else {
            self.state.dialog.show_error("Error", "Preset manager not available");
        }
    }
    async fn handle_wireless_key(&mut self, key: KeyCode) {
        // If editing
        if let Some(field) = self.state.wireless.editing {
            match key {
                KeyCode::Enter => {
                    self.state.wireless.editing = None;
                    self.state.wireless.status = Some("Input saved.".to_string());
                }
                KeyCode::Esc => {
                    self.state.wireless.editing = None;
                    self.state.wireless.status = Some("Input cancelled.".to_string());
                }
                KeyCode::Backspace => {
                     match field {
                        WirelessField::Port => { self.state.wireless.port_input.pop(); }
                        WirelessField::Address => { self.state.wireless.address_input.pop(); }
                     }
                }
                KeyCode::Char(c) => {
                    match field {
                        WirelessField::Port => {
                            if c.is_numeric() {
                                self.state.wireless.port_input.push(c);
                            }
                        }
                        WirelessField::Address => {
                            self.state.wireless.address_input.push(c);
                        }
                    }
                }
                _ => {}
            }
            return;
        }

        // Normal mode
        match key {
            KeyCode::Char('p') => {
                 self.state.wireless.editing = Some(WirelessField::Port);
                 self.state.wireless.status = Some("Editing Port... (Enter to save)".to_string());
                 self.state.wireless.port_input.clear();
            }
            KeyCode::Char('i') => {
                 self.state.wireless.editing = Some(WirelessField::Address);
                 self.state.wireless.status = Some("Editing Address... (Enter to save)".to_string());
                 self.state.wireless.address_input.clear();
            }
            KeyCode::Char('e') => self.enable_wireless_adb().await,
            KeyCode::Char('c') => self.connect_wireless().await,
            KeyCode::Char('q') | KeyCode::Esc => self.current_screen = Screen::Home,
            _ => {}
        }
    }

    async fn enable_wireless_adb(&mut self) {
        if let DeviceState::Connected(info) = &self.state.device {
             let serial = info.serial.clone();
             let port_str = self.state.wireless.port_input.clone();
             let port = port_str.parse::<u16>().unwrap_or(5555);
             
             self.state.wireless.status = Some("Enabling wireless mode...".to_string());
             
             let adb = Adb::new().with_device(&serial);
             match adb.tcpip(port).await {
                 Ok(_) => {
                     self.state.wireless.status = Some(format!("Success! Wireless enabled on port {}. Now disconnect USB and connect via Wi-Fi.", port));
                     // Try to get IP
                     if let Ok(Some(ip)) = adb.get_device_ip().await {
                         self.state.wireless.address_input = format!("{}:{}", ip, port);
                     }
                 }
                 Err(e) => {
                     self.state.wireless.status = Some(format!("Error: {}", e));
                 }
             }
        } else {
             self.state.wireless.status = Some("Error: No device connected via USB.".to_string());
        }
    }

    async fn connect_wireless(&mut self) {
        let address = self.state.wireless.address_input.clone();
        if address.is_empty() {
             self.state.wireless.status = Some("Error: Address is empty.".to_string());
             return;
        }

        self.state.wireless.status = Some(format!("Connecting to {}...", address));
        
        let adb = Adb::new();
        match adb.connect(&address).await {
            Ok(_) => {
                 self.state.wireless.status = Some(format!("Successfully connected to {}!", address));
                 self.check_device().await; // Refresh device list
            }
            Err(e) => {
                self.state.wireless.status = Some(format!("Error: {}", e));
            }
        }
    }

    async fn handle_user_guide_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => self.current_screen = Screen::Home,
            KeyCode::Up | KeyCode::Char('k') => {
                if self.state.user_guide.scroll_offset > 0 {
                    self.state.user_guide.scroll_offset -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                 self.state.user_guide.scroll_offset += 1;
            }
            KeyCode::PageUp => {
                if self.state.user_guide.scroll_offset > 5 {
                    self.state.user_guide.scroll_offset -= 5;
                } else {
                    self.state.user_guide.scroll_offset = 0;
                }
            }
            KeyCode::PageDown => {
                self.state.user_guide.scroll_offset += 5;
            }
            _ => {}
        }
    }

    async fn handle_menu_select(&mut self) {
        match self.selected_index {
            0 => {
                // Package Browser
                self.current_screen = Screen::PackageBrowser;
                self.load_packages().await;
            }
            1 => {
                // User Guide
                self.current_screen = Screen::UserGuide;
                self.state.user_guide = crate::screens::UserGuideState::default();
            }
            2 => {
                // Presets
                self.current_screen = Screen::Presets;
            }
            3 => {
                // Device Info
                self.current_screen = Screen::DeviceInfo;
            }
            4 => {
                // Health Check
                self.current_screen = Screen::Health;
                self.load_health().await;
            }
            5 => {
                // Rescue History
                self.current_screen = Screen::Rescue;
            }
            6 => {
                // Settings
                self.current_screen = Screen::Settings;
            }
            7 => {
                // Wireless ADB
                self.current_screen = Screen::Wireless;
            }
            8 => {
                // Support/Sponsor
                self.current_screen = Screen::Support;
            }
            9 => {
                // About
                self.current_screen = Screen::About;
            }
            10 => {
                // Exit
                self.should_quit = true;
            }
            _ => {}
        }
    }

    /// Handle dialog key input
    async fn handle_dialog_key(&mut self, key: KeyCode) {
        let dialog_type = self.state.dialog.active.clone();
        
        match dialog_type {
            Some(DialogType::Confirm { operation, packages, .. }) => {
                match key {
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.state.dialog.selected = 0;
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.state.dialog.selected = 1;
                    }
                    KeyCode::Enter => {
                        if self.state.dialog.selected == 0 {
                            // Yes - execute operation
                            self.state.dialog.close();
                            self.execute_operation(operation, packages).await;
                        } else {
                            // No - close dialog
                            self.state.dialog.close();
                        }
                    }
                    KeyCode::Char('y') => {
                        self.state.dialog.close();
                        self.execute_operation(operation, packages).await;
                    }
                    KeyCode::Char('n') | KeyCode::Esc => {
                        self.state.dialog.close();
                    }
                    _ => {}
                }
            }
            Some(DialogType::ActionMenu { selected }) => {
                match key {
                    KeyCode::Up | KeyCode::Char('k') => {
                        if selected > 0
                            && let Some(DialogType::ActionMenu { selected: s }) = &mut self.state.dialog.active {
                                *s -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if selected < dialogs::action_count() - 1
                            && let Some(DialogType::ActionMenu { selected: s }) = &mut self.state.dialog.active {
                                *s += 1;
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(op) = dialogs::operation_by_index(selected) {
                            self.state.dialog.close();
                            self.start_operation(op).await;
                        }
                    }
                    KeyCode::Esc => {
                        self.state.dialog.close();
                    }
                    _ => {}
                }
            }
            Some(DialogType::Progress { .. }) => {
                // Progress dialog doesn't respond to keys (operation in progress)
            }
            Some(DialogType::Result { .. }) | Some(DialogType::Error { .. }) => {
                match key {
                    KeyCode::Enter | KeyCode::Esc => {
                        self.state.dialog.close();
                        // Refresh package list after operation
                        self.load_packages().await;
                    }
                    _ => {}
                }
            }
            None => {}
        }
    }

    /// Start a package operation with confirmation
    async fn start_operation(&mut self, operation: PackageOperation) {
        // Get selected packages
        let packages: Vec<String> = self.state.browser.packages
            .iter()
            .filter(|p| p.selected && p.installed)
            .map(|p| p.name.clone())
            .collect();

        if packages.is_empty() {
            self.state.dialog.show_error(
                "No Packages Selected",
                "Please select at least one installed package.",
            );
            return;
        }

        // Build confirmation message
        let message = if packages.len() == 1 {
            format!("Are you sure you want to {} this package?", operation.display_name().to_lowercase())
        } else {
            format!(
                "Are you sure you want to {} {} packages?",
                operation.display_name().to_lowercase(),
                packages.len()
            )
        };

        // Show confirmation dialog
        self.state.dialog.show_confirm(
            format!("{} Packages", operation.display_name()),
            message,
            operation,
            packages,
        );
    }

    /// Execute a package operation
    async fn execute_operation(&mut self, operation: PackageOperation, packages: Vec<String>) {
        // Show progress dialog
        self.state.dialog.show_progress(
            format!("{}ing packages...", operation.display_name()),
            packages.len(),
        );

        let mut success = Vec::new();
        let mut failed: Vec<(String, String)> = Vec::new();

        // Get device serial
        let serial = match &self.state.device {
            DeviceState::Connected(info) => info.serial.clone(),
            _ => {
                self.state.dialog.show_error("No Device", "No device connected.");
                return;
            }
        };

        let adb = Adb::new().with_device(&serial);
        let pm = PackageManager::new(adb);

        for (index, package) in packages.iter().enumerate() {
            // Update progress
            self.state.dialog.update_progress(index, package);

            // In dry run mode, simulate success
            if self.state.dry_run {
                info!("[DRY RUN] Would {} package: {}", operation.display_name().to_lowercase(), package);
                success.push(package.clone());
                continue;
            }

            // Execute the operation
            let result = match operation {
                PackageOperation::Uninstall => pm.uninstall(package).await,
                PackageOperation::Disable => pm.disable(package).await,
                PackageOperation::Enable => pm.enable(package).await,
                PackageOperation::Reinstall => pm.reinstall(package).await,
                PackageOperation::ClearData => pm.clear(package).await,
            };

            match result {
                Ok(_) => {
                    info!("{} package: {}", operation.display_name(), package);
                    success.push(package.clone());
                }
                Err(e) => {
                    error!("Failed to {} {}: {}", operation.display_name().to_lowercase(), package, e);
                    failed.push((package.clone(), e.to_string()));
                }
            }
        }

        // Show result dialog
        let title = if failed.is_empty() {
            format!("{} Complete", operation.display_name())
        } else if success.is_empty() {
            format!("{} Failed", operation.display_name())
        } else {
            format!("{} Partially Complete", operation.display_name())
        };

        self.state.dialog.show_result(title, success, failed);

        // Clear selections
        self.state.browser.clear_selection();
        self.state.browser.selection_mode = SelectionMode::Single;
    }



    /// Get main menu items
    pub fn menu_items(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("📦 Package Browser", "Browse and remove bloatware packages"),
            ("📘 User Guide", "Learn how to use BloatwareHatao"),
            ("📋 Profiles", "Apply preset or custom removal profiles"),
            ("📱 Device Info", "View connected device information"),
            ("❤️ Health Check", "Check device battery, storage, memory"),
            ("💾 Backup & Restore", "Manage package backups"),
            ("⚙️ Settings", "Configure application settings"),
            ("📡 Wireless ADB", "Connect via Wi-Fi"),
            ("💖 Support/Sponsor", "Donate & Support Development"),
            ("ℹ️ About", "App Info & License"),
            ("🚪 Exit", "Exit BloatwareHatao"),
        ]
    }

    /// Export a preset to JSON file
    fn export_preset(&self, preset_id: &str) -> Result<String, String> {
        let pm = match &self.state.preset_manager {
            Some(pm) => pm,
            None => return Err("Preset manager not available".to_string()),
        };
        
        // Get the export path
        let export_dir = pm.custom_dir().parent()
            .map(|p| p.join("exports"))
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        
        // Create exports directory if needed
        std::fs::create_dir_all(&export_dir)
            .map_err(|e| format!("Failed to create exports directory: {}", e))?;
        
        // Export the preset
        let json = pm.export_preset(preset_id)
            .map_err(|e| e.to_string())?;
        
        // Write to file
        let filename = format!("{}.json", preset_id);
        let filepath = export_dir.join(&filename);
        std::fs::write(&filepath, &json)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(filepath.display().to_string())
    }

    /// Delete a custom preset
    fn delete_preset(&self, preset_id: &str) -> Result<(), String> {
        let pm = match &self.state.preset_manager {
            Some(pm) => pm,
            None => return Err("Preset manager not available".to_string()),
        };
        
        pm.delete_custom_preset(preset_id)
            .map_err(|e| e.to_string())
    }

    /// Create a new rescue point
    fn create_rescue_point(&mut self) {
        let _rm = match &self.state.rescue_manager {
            Some(rm) => rm,
            None => {
                self.status_message = Some("Rescue manager not available".to_string());
                return;
            }
        };

        // If not connected and live run, show error
        if !self.state.dry_run {
            if let DeviceState::Connected(_info) = &self.state.device {
                // Continue
            } else {
                self.state.dialog.show_error("No Device", "Please connect a device to create a rescue point.");
                return;
            }
        }
        
        let tx = self.tx.clone();
        
        // Show progress via status (simpler than dialog for now)
        self.status_message = Some("Creating rescue point... please wait".to_string());
        
        // Dry run handling
        if self.state.dry_run {
            self.status_message = Some("[DRY RUN] Would create rescue point of all packages".to_string());
            return;
        }

        let serial = if let DeviceState::Connected(info) = &self.state.device {
            info.serial.clone()
        } else {
            return;
        };

        // Needs to own RM or clone path info to pass to thread
        let rescue_dir = self.state.rescue_manager.as_ref().unwrap().rescue_dir().to_path_buf();
        let session_dir = self.state.rescue_manager.as_ref().unwrap().session_dir().to_path_buf();

        tokio::spawn(async move {
            use bloatwarehatao_core::rescue::RescueManager;
            let rm = RescueManager::new(rescue_dir, session_dir);
            let adb = Adb::new().with_device(&serial);
            
            let result = rm.create_rescue_point(&adb).await.map_err(|e| e.to_string());
            let _ = tx.send(Message::RescuePointCreated(result));
        });
    }

    /// Restore from rescue entry
    fn restore_rescue_entry(&mut self, backup: bloatwarehatao_core::rescue::RescueEntry) {
        if !self.state.dry_run {
            if let DeviceState::Connected(_info) = &self.state.device {
                // Continue
            } else {
                self.state.dialog.show_error("No Device", "Please connect a device to restore.");
                return;
            }
        }

        let tx = self.tx.clone();
        self.status_message = Some(format!("Restoring backup {}... please wait", backup.id));

        if self.state.dry_run {
            self.status_message = Some(format!("[DRY RUN] Would restore {} packages", backup.packages.len()));
            return;
        }

        let serial = if let DeviceState::Connected(info) = &self.state.device {
            info.serial.clone()
        } else {
            return;
        };

        let bm_opt = self.state.rescue_manager.as_ref().map(|rm| (rm.rescue_dir().to_path_buf(), rm.session_dir().to_path_buf()));
        
        if let Some((rescue_dir, session_dir)) = bm_opt {
             tokio::spawn(async move {
                use bloatwarehatao_core::rescue::RescueManager;
                let rm = RescueManager::new(rescue_dir, session_dir);
                let adb = Adb::new().with_device(&serial);
                
                match rm.restore_from_entry(&adb, &backup).await {
                    Ok(results) => {
                        let success = results.iter().filter(|(_, s)| *s).count();
                        let failed = results.len() - success;
                        let _ = tx.send(Message::RescueRestored(Ok((success, failed))));
                    }
                    Err(e) => {
                        let _ = tx.send(Message::RescueRestored(Err(e.to_string())));
                    }
                }
            });
        }
    }

    /// Restore from rescue session
    fn restore_rescue_session(&mut self, session: bloatwarehatao_core::rescue::RescueSession) {
        if !self.state.dry_run {
            if let DeviceState::Connected(_info) = &self.state.device {
                // Continue
            } else {
                self.state.dialog.show_error("No Device", "Please connect a device to restore.");
                return;
            }
        }

        let tx = self.tx.clone();
        self.status_message = Some(format!("Restoring session {}... please wait", session.session_id));

        if self.state.dry_run {
            self.status_message = Some(format!("[DRY RUN] Would restore {} packages", session.removed_packages.len()));
            return;
        }

        let serial = if let DeviceState::Connected(info) = &self.state.device {
            info.serial.clone()
        } else {
            return;
        };

        let bm_opt = self.state.rescue_manager.as_ref().map(|rm| (rm.rescue_dir().to_path_buf(), rm.session_dir().to_path_buf()));
        
        if let Some((rescue_dir, session_dir)) = bm_opt {
             tokio::spawn(async move {
                use bloatwarehatao_core::rescue::RescueManager;
                let rm = RescueManager::new(rescue_dir, session_dir);
                let adb = Adb::new().with_device(&serial);
                
                match rm.restore_from_session(&adb, &session).await {
                    Ok(results) => {
                        let success = results.iter().filter(|(_, s)| *s).count();
                        let failed = results.len() - success;
                        let _ = tx.send(Message::SessionRestored(Ok((success, failed))));
                    }
                    Err(e) => {
                        let _ = tx.send(Message::SessionRestored(Err(e.to_string())));
                    }
                }
            });
        }
    }
}
/// Run the TUI application
pub async fn run(dry_run: bool) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(dry_run);
    
    // Initialize app
    app.init().await?;

    // Main loop
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
    loop {
        // Draw UI
        terminal.draw(|f| draw(f, app))?;

        // Handle events with timeout for async operations
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            app.handle_key(key.code, key.modifiers).await;
        }

        let should_reload = false;
        // Process background messages
        while let Ok(msg) = app.rx.try_recv() {
            match msg {

                Message::RescuePointCreated(result) => {
                    match result {
                        Ok(entry) => {
                            app.status_message = Some(format!("✓ Rescue Point created: {}", entry.id));
                            app.state.reload_rescue();
                        }
                        Err(e) => {
                            app.state.dialog.show_error("Creation Failed", &e);
                        }
                    }
                }
                Message::RescueRestored(result) => {
                    match result {
                        Ok((success, failed)) => {
                            app.state.dialog.show_result(
                                "Rescue Point Restored".to_string(),
                                vec!["Restore successful".to_string(); success], // Dummy for now, or change dialog API
                                vec![("Restore failed".to_string(), "check logs".to_string()); failed],
                            );
                            app.status_message = Some(format!("✓ Restore complete: {} restored, {} failed", success, failed));
                        }
                        Err(e) => {
                             app.state.dialog.show_error("Restore Failed", &e);
                        }
                    }
                }
                Message::SessionRestored(result) => {
                    match result {
                        Ok((success, failed)) => {
                            app.state.dialog.show_result(
                                "Session Restored".to_string(),
                                vec!["Restore successful".to_string(); success],
                                vec![("Restore failed".to_string(), "check logs".to_string()); failed],
                            );
                            app.status_message = Some(format!("✓ Session restore complete: {} restored, {} failed", success, failed));
                        }
                        Err(e) => {
                             app.state.dialog.show_error("Session Restore Failed", &e);
                        }
                    }
                }
                Message::LabelUpdate { package_name, label } => {
                    // Update single package label from background fetch
                    app.state.browser.update_package_label(&package_name, label);
                }
                Message::LabelProgress { fetched, total } => {
                    app.state.browser.labels_fetched = fetched;
                    app.state.browser.labels_total = total;
                    if fetched < total {
                        app.state.browser.status = Some(format!(
                            "Fetching real app names... ({}/{})",
                            fetched, total
                        ));
                    }
                }
                Message::LabelsFetchComplete => {
                    app.state.browser.fetching_labels = false;
                    // Re-sort and re-apply filter after all labels updated
                    app.state.browser.packages.sort_by(|a, b| {
                        a.label.to_lowercase().cmp(&b.label.to_lowercase())
                    });
                    app.state.browser.apply_filter();
                    app.state.browser.status = Some(format!(
                        "Loaded {} packages (names updated)",
                        app.state.browser.filtered_indices.len()
                    ));
                }

            }
        }

        if should_reload {
            app.load_packages().await;
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

/// Main draw function
fn draw(f: &mut ratatui::Frame, app: &App) {
    match app.current_screen {
        Screen::Home => HomeScreen::draw(f, app),
        Screen::PackageBrowser => PackageBrowserScreen::draw(f, app),
        Screen::UserGuide => UserGuideScreen::draw(f, app),
        Screen::DeviceInfo => DeviceInfoScreen::draw(f, app),
        Screen::Health => HealthScreen::draw(f, app),
        Screen::Settings => SettingsScreen::draw(f, app, app.state.settings_selected),
        Screen::Wireless => WirelessScreen::draw(f, app),
        Screen::About => AboutScreen::draw(f, app),
        Screen::Support => SupportScreen::draw(f, app),
        Screen::Presets => PresetsScreen::draw(f, app),
        Screen::Rescue => RescueScreen::draw(f, app, app.state.rescue_tab, app.state.rescue_selected),
        Screen::Help => draw_help(f, app),
    }

    // Draw dialog overlay if active
    dialogs::draw_dialogs(f, &app.state.dialog);
}

/// Draw help overlay
fn draw_help(f: &mut ratatui::Frame, _app: &App) {
    use ratatui::{

        style::{Color, Modifier, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Clear, Paragraph, Wrap},
    };

    let area = centered_rect(60, 80, f.area());
    
    f.render_widget(Clear, area);

    let help_text = vec![
        Line::from(Span::styled(
            "BloatwareHatao - Keyboard Shortcuts",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Navigation", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  ↑/k       Move up"),
        Line::from("  ↓/j       Move down"),
        Line::from("  Enter     Select/confirm"),
        Line::from("  Esc       Go back"),
        Line::from("  PgUp/PgDn Page up/down"),
        Line::from("  g/G       Go to top/bottom"),
        Line::from(""),
        Line::from(Span::styled("Package Browser", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  Space     Toggle selection"),
        Line::from("  /         Search packages"),
        Line::from("  c         Clear search"),
        Line::from("  i         Toggle installed filter"),
        Line::from("  r         Refresh list"),
        Line::from(""),
        Line::from(Span::styled("General", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  ?         Show this help"),
        Line::from("  q         Quit / Go back"),
        Line::from("  Ctrl+c    Force quit"),
        Line::from(""),
        Line::from(Span::styled(
            "Press any key to close",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(help, area);
}

/// Create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    use ratatui::layout::{Constraint, Direction, Layout};
    
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
