//! Screens module
//!
//! Individual screen implementations for the TUI.

mod about;
mod device_info;
pub mod dialogs;
mod health;
mod home;
mod package_browser;
mod presets;
mod rescue;
mod settings;
mod support;
mod user_guide;
mod wireless;

pub use about::AboutScreen;
pub use device_info::DeviceInfoScreen;
pub use health::HealthScreen;
pub use home::HomeScreen;
pub use package_browser::PackageBrowserScreen;
pub use presets::{PresetCreationStep, PresetCreatorState, PresetsScreen};
pub use rescue::RescueScreen;
pub use settings::SettingsScreen;
pub use support::SupportScreen;
pub use user_guide::{UserGuideScreen, UserGuideState};
pub use wireless::WirelessScreen;
