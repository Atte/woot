use crate::{device::Device, proto::lekker, Result};
use active_win_pos_rs::get_active_window;
use color_eyre::eyre::eyre;
use serde::Deserialize;
use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    time::Duration,
};
use sysinfo::{Pid, ProcessExt, ProcessRefreshKind, System, SystemExt};

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

pub fn run(device: Device, config_path: &Path, oneshot: bool) -> Result<()> {
    let config: Config = {
        let mut fh = File::open(config_path)?;
        let mut buffer = String::new();
        fh.read_to_string(&mut buffer)?;
        toml::from_str(&buffer)?
    };

    let mut system = System::new();
    let mut previous_window_id = String::new();

    loop {
        let window = get_active_window().map_err(|_| eyre!("Unable to get active window"))?;
        if window.window_id != previous_window_id {
            previous_window_id = window.window_id;

            // cfg to match https://docs.rs/sysinfo/latest/src/sysinfo/common.rs.html#76
            #[cfg(all(
                not(feature = "unknown-ci"),
                any(
                    target_os = "freebsd",
                    target_os = "linux",
                    target_os = "android",
                    target_os = "macos",
                    target_os = "ios",
                )
            ))]
            let pid = Pid::from(window.process_id as i32);

            #[cfg(not(all(
                not(feature = "unknown-ci"),
                any(
                    target_os = "freebsd",
                    target_os = "linux",
                    target_os = "android",
                    target_os = "macos",
                    target_os = "ios",
                )
            )))]
            let pid = Pid::from(window.process_id as usize);

            // failure is likely just a race condition as a window is closed
            if system.refresh_process_specifics(pid, ProcessRefreshKind::new()) {
                let process = system
                    .process(pid)
                    .ok_or_else(|| eyre!("sysinfo lied about process existing"))?;
                for rule in &config.rules {
                    if rule
                        .name
                        .as_ref()
                        .map_or(false, |name| process.name() == name)
                        || rule.exe.as_ref().map_or(false, |exe| process.exe() == exe)
                        || rule.cmd.as_ref().map_or(false, |cmd| process.cmd() == cmd)
                    {
                        if let Some(wanted) = rule.profile {
                            let current =
                                device.feature_report(lekker::GetCurrentKeyboardProfileIndex)?;
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
            }
        }

        if oneshot {
            break;
        }
        std::thread::sleep(Duration::from_secs(config.interval));
    }

    Ok(())
}
