use clap::{Parser, Subcommand};
use color_eyre::eyre::{eyre, Result};
use hidapi::HidApi;

use crate::device::Device;
use crate::proto::{lekker, USAGE_PAGE};

pub mod device;
pub mod proto;

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
    /// Activate a profile saved on the keyboard
    ActivateProfile {
        /// Profile to activate (0 for digital, 1-3 for analog)
        #[clap(value_parser = clap::value_parser!(u8).range(0..=3))]
        index: u8,
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

    let info = infos
        .into_iter()
        .find(|info| {
            if let Some(ref serial) = args.serial {
                info.serial_number() == Some(serial)
            } else {
                true
            }
        })
        .ok_or_else(|| eyre!("No Wooting devices found!"))?;

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
            eprintln!("{:?}", &serial);
            println!(
                "A{:02}B{:02}{:02}W{:02}{}{}{:05}",
                serial.supplier_number,
                serial.year,
                serial.week_number,
                serial.product_number,
                serial.revision_number,
                match serial
                    .production_stage
                    .ok_or_else(|| eyre!("unknown production stage"))?
                {
                    lekker::ProductionStage::Mass => 'H',
                    lekker::ProductionStage::PVT => 'P',
                    lekker::ProductionStage::DVT => 'T',
                    lekker::ProductionStage::EVT => 'E',
                    lekker::ProductionStage::Prototype => 'X',
                },
                serial.product_id
            );
        }
        Commands::ActivateProfile { index } => {
            device.feature_report(lekker::ActivateProfile(index))?;
            // reload seems to be required to get all settings to apply properly
            device.feature_report(lekker::ReloadProfile)?;
        }
    }

    Ok(())
}
