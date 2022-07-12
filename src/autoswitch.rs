use crate::{device::Device, proto::lekker, Result};
use serde::Deserialize;
use std::{
    convert::Infallible,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    time::Duration,
};
use sysinfo::{ProcessExt, ProcessRefreshKind, System, SystemExt};

#[derive(Debug, Clone, Deserialize)]
struct Config {
    #[serde(default)]
    interval: u64,
    rules: Vec<Rule>,
}

#[derive(Debug, Clone, Deserialize)]
struct Rule {
    profile: Option<u8>,
    name: Option<String>,
    exe: Option<PathBuf>,
    cmd: Option<Vec<String>>,
}

pub fn run(device: Device, config_path: &Path) -> Result<Infallible> {
    let config: Config = {
        let mut fh = File::open(config_path)?;
        let mut buffer = String::new();
        fh.read_to_string(&mut buffer)?;
        toml::from_str(&buffer)?
    };

    let mut system = System::new();

    loop {
        system.refresh_processes_specifics(ProcessRefreshKind::new());

        for rule in &config.rules {
            if system.processes().values().any(|proc| {
                rule.name.as_ref().map_or(false, |name| proc.name() == name)
                    || rule.exe.as_ref().map_or(false, |exe| proc.exe() == exe)
                    || rule.cmd.as_ref().map_or(false, |cmd| proc.cmd() == cmd)
            }) {
                if let Some(wanted) = rule.profile {
                    let current = device.feature_report(lekker::GetCurrentKeyboardProfileIndex)?;
                    if current != wanted {
                        println!("switching to profile {}", wanted);
                        device.feature_report(lekker::ActivateProfile(wanted))?;
                        // reload seems to be required to get all settings to apply properly
                        device.feature_report(lekker::ReloadProfile)?;
                    }
                }
                break;
            }
        }

        std::thread::sleep(Duration::from_secs(config.interval));
    }
}
