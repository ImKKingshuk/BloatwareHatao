//! Screens module
//!
//! Individual screen implementations for the TUI.

mod home;
mod package_browser;
mod device_info;
mod health;
mod settings;
mod presets;
mod rescue;
mod user_guide;
mod wireless;
mod about;
mod support;
pub mod dialogs;

pub use home::HomeScreen;
pub use package_browser::PackageBrowserScreen;
pub use device_info::DeviceInfoScreen;
pub use health::HealthScreen;
pub use settings::SettingsScreen;
pub use presets::{PresetsScreen, PresetCreatorState, PresetCreationStep};
pub use rescue::RescueScreen;
pub use user_guide::{UserGuideScreen, UserGuideState};
pub use wireless::WirelessScreen;
pub use about::AboutScreen;
pub use support::SupportScreen;
