use std::{fs, process::Command};

use anyhow::{Context, Result};
use chrono::Local;
use daemonize::Daemonize;
use procfs::process::Process;

const SU_TOAST: &str = include_str!("./panic.sh");

fn main() -> Result<()> {
    use fs::OpenOptions;
    let blackhole = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/data/adb/modules/fas_rs_next/daemon.log")?;

    let daemonize = Daemonize::new()
        .stdout(blackhole.try_clone()?)
        .stderr(blackhole);

    daemonize.start().context("Failed to daemon start")?;

    loop {
        let mut check_state = false;
        let all_processes: Vec<Process> = procfs::process::all_processes()
            .expect("Can't read /proc")
            .filter_map(|p| match p {
                Ok(p) => Some(p),
                Err(e) => match e {
                    procfs::ProcError::NotFound(_) => None,
                    procfs::ProcError::Io(_, _) => None,
                    x => {
                        println!("Can't read process due to error {x:?}");
                        None
                    }
                },
            })
            .collect();

        for process in all_processes {
            let cmdlines = match process.cmdline() {
                Ok(s) => s,
                Err(_) => continue,
            };

            if cmdlines.contains(&"fas-rs-next".to_string()) {
                check_state = true;
            }
        }

        if check_state {
            check_state = false;
            let time = Local::now();
            println!("{time} fas-rs-next crashes");
            let _ = Command::new("su")
                .arg("-lp")
                .arg("2000")
                .arg("-c")
                .arg(SU_TOAST)
                .output();
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
