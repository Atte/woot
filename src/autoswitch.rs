use crate::{device::Device, proto::lekker, Result};
use active_win_pos_rs::get_active_window;
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
        if let Ok(window) = get_active_window() {
            if window.window_id != previous_window_id {
                log::trace!("window: {:?}", &window);
                previous_window_id = window.window_id;
                process_activated(&config, &mut system, &device, window.process_id)?;
            }
        }

        if oneshot {
            break;
        }

        std::thread::sleep(Duration::from_secs(config.interval));
    }

    Ok(())
}

fn process_activated(
    config: &Config,
    system: &mut System,
    device: &Device,
    process_id: u64,
) -> Result<()> {
    // cfg to match https://docs.rs/sysinfo/latest/src/sysinfo/common.rs.html#76
    let pid;
    cfg_if::cfg_if! {
        if #[cfg(all(
            not(feature = "unknown-ci"),
            any(
                target_os = "freebsd",
                target_os = "linux",
                target_os = "android",
                target_os = "macos",
                target_os = "ios",
            )
        ))] {
            pid = Pid::from(process_id as i32)
        } else if #[cfg(windows)] {
            // resolve thread ID to base process ID
            let base_process_id = unsafe {
                let handle = winapi::um::processthreadsapi::OpenThread(
                    winapi::um::winnt::THREAD_QUERY_LIMITED_INFORMATION,
                    0,
                    process_id as winapi::shared::minwindef::DWORD
                );
                let id = winapi::um::processthreadsapi::GetProcessIdOfThread(handle);
                winapi::um::handleapi::CloseHandle(handle);
                id
            };
            pid = Pid::from(if base_process_id == 0 { process_id as usize } else { base_process_id as usize })
        } else {
            pid = Pid::from(window.process_id as usize)
        }
    };

    system.refresh_process_specifics(pid, ProcessRefreshKind::new());

    // failure is likely just a race condition as a window is closed
    if let Some(process) = system.process(pid) {
        log::trace!("process: {:?}", process);
        for rule in &config.rules {
            if rule
                .name
                .as_ref()
                .map_or(true, |name| process.name() == name)
                && rule.exe.as_ref().map_or(true, |exe| process.exe() == exe)
                && rule.cmd.as_ref().map_or(true, |cmd| process.cmd() == cmd)
            {
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
    }

    Ok(())
}
