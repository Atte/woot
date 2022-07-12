use clap::{Parser, Subcommand};
use color_eyre::eyre::{bail, eyre, Result};
use hidapi::HidApi;
use std::path::PathBuf;

use crate::device::Device;
use crate::proto::{lekker, USAGE_PAGE};

mod device;
mod proto;

#[cfg(feature = "autoswitch")]
mod autoswitch;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
    /// Serial of keyboard to control, as shown by `list` (required if multiple Wooting devices are plugged in)
    #[clap(short, long)]
    serial: Option<String>,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
enum Commands {
    /// List available keyboards
    List,
    /// Check connectivity to keyboard
    Ping,
    /// Get keyboard firmware version number
    GetVersion,
    /// Get keyboard serial number
    GetSerial,
    /// Get the currently active profile index
    GetCurrentProfile,
    /// Activate a profile saved on the keyboard
    ActivateProfile {
        /// Profile to activate (0 for digital, 1-3 for analog)
        #[clap(value_parser = clap::value_parser!(u8).range(0..=3))]
        index: u8,
    },
    /// Automatically switch profiles based on a configuration file
    #[cfg(feature = "autoswitch")]
    Autoswitch {
        /// Don't stay running to monitor window changes
        #[clap(short = '1', long)]
        oneshot: bool,
        /// Path to configuration file (see README.md for format)
        config: PathBuf,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::try_init()?;

    let args = Args::parse();
    let hidapi = HidApi::new()?;

    let infos: Vec<_> = hidapi
        .device_list()
        .filter(|info| {
            info.vendor_id() == proto::VENDOR_ID
                && proto::lekker::DEVICE_IDS.contains(&info.product_id())
                && info.usage_page() == USAGE_PAGE
        })
        .collect();

    // handle early to avoid unnecessary I/O
    if args.command == Commands::List {
        for info in infos {
            println!(
                "{}: {}",
                info.product_string().unwrap_or("unknown Wooting device"),
                info.serial_number().unwrap_or("no serial available"),
            );
        }
        return Ok(());
    }

    let info = if let Some(ref serial) = args.serial {
        infos
            .into_iter()
            .find(|info| info.serial_number() == Some(serial))
            .ok_or_else(|| eyre!("No Wooting device matching the given serial found!"))?
    } else if infos.len() > 1 {
        bail!("--serial is required when multiple Wooting devices are plugged in!");
    } else {
        infos
            .first()
            .ok_or_else(|| eyre!("No Wooting devices found!"))?
    };

    let device = Device::new(info.open_device(&hidapi)?);

    match args.command {
        Commands::List => {
            unreachable!("List is handled separately earlier to avoid unnecessary I/O");
        }
        Commands::Ping => {
            device.feature_report(lekker::Ping)?;
            println!("pong");
        }
        Commands::GetVersion => {
            let (major, minor, patch) = device.feature_report(lekker::GetVersion)?;
            println!("{}.{}.{}", major, minor, patch);
        }
        Commands::GetSerial => {
            let serial = device.feature_report(lekker::GetSerial)?;
            log::info!("{:?}", serial);
            println!("{}", serial);
        }
        Commands::GetCurrentProfile => {
            let index = device.feature_report(lekker::GetCurrentKeyboardProfileIndex)?;
            println!("{}", index);
        }
        Commands::ActivateProfile { index } => {
            device.feature_report(lekker::ActivateProfile(index))?;
            // reload seems to be required to get all settings to apply properly
            device.feature_report(lekker::ReloadProfile)?;
        }
        #[cfg(feature = "autoswitch")]
        Commands::Autoswitch { oneshot, config } => {
            autoswitch::run(device, &config, oneshot)?;
        }
    }

    Ok(())
}
