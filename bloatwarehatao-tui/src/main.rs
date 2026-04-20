//! BloatwareHatao TUI Application
//!
//! A beautiful terminal interface for the ultimate Android bloatware removal tool.

mod app;
mod screens;
mod state;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// BloatwareHatao - Ultimate Android Bloatware Removal Tool
#[derive(Parser, Debug)]
#[command(name = "bloatwarehatao")]
#[command(author = "ImKKingshuk")]
#[command(version)]
#[command(about = "Ultimate Android Bloatware Removal Tool", long_about = None)]
struct Cli {
    /// Enable dry run mode (no actual changes)
    #[arg(long)]
    dry_run: bool,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Show device information and exit
    #[arg(long)]
    device_info: bool,

    /// List installed packages and exit
    #[arg(long)]
    list_packages: bool,

    /// Run in offline mode (use local database)
    #[arg(long)]
    offline: bool,

    /// Target device serial (for multiple devices)
    #[arg(short = 's', long)]
    device: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logging
    let filter = if cli.verbose { "debug" } else { "info" };

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| filter.into()),
        )
        .init();

    // Handle non-interactive commands
    if cli.device_info {
        return show_device_info(cli.device.as_deref()).await;
    }

    if cli.list_packages {
        return list_packages(cli.device.as_deref()).await;
    }

    // Run the TUI application
    app::run(cli.dry_run).await
}

async fn show_device_info(device: Option<&str>) -> Result<()> {
    use bloatwarehatao_core::adb::Adb;
    use bloatwarehatao_core::device::Device;

    let adb = if let Some(serial) = device {
        Adb::new().with_device(serial)
    } else {
        Adb::new()
    };

    let device = Device::from_adb(&adb).await?;

    println!();
    println!("📱 Device Information");
    println!("=====================");
    println!("Brand:          {}", device.brand);
    println!("Model:          {}", device.model);
    println!("Manufacturer:   {}", device.manufacturer);
    println!(
        "Android:        {} (SDK {})",
        device.android_version, device.sdk_version
    );
    println!("Build:          {}", device.build_id);
    println!("Serial:         {}", device.serial);
    if let Some(ref patch) = device.security_patch {
        println!("Security Patch: {}", patch);
    }
    println!("OEM Detected:   {}", device.detect_oem().display_name());
    println!();

    Ok(())
}

async fn list_packages(device: Option<&str>) -> Result<()> {
    use bloatwarehatao_core::adb::Adb;
    use bloatwarehatao_core::package::PackageManager;

    let adb = if let Some(serial) = device {
        Adb::new().with_device(serial)
    } else {
        Adb::new()
    };

    let pm = PackageManager::new(adb);
    let packages = pm.list_packages().await?;

    println!();
    println!("📦 Installed Packages ({})", packages.len());
    println!("==========================");
    for package in packages {
        println!("  {}", package);
    }
    println!();

    Ok(())
}
