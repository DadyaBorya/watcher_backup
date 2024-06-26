use std::{path::PathBuf, error::Error};
use std::process::{Command};
use std::thread::sleep;
use std::time::Duration;
use chrono::{Datelike, Local, Timelike, Utc};

use clap::Parser;
use cron_parser::parse;
use mappers::{template_to_dir_entry, dir_entry_to_commands};
use services::file_service;
use crate::models::config::Config;
use crate::models::scheduler::{Scheduler};

mod services;
mod mappers;
mod models;

/// Program that convert template to list of rclone commands
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the scheduler
    #[arg(short, long)]
    path: PathBuf,
    /// Is first excecution
    #[arg(short, long)]
    first: String,

    #[arg(short, long)]
    config: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    #[cfg(target_os = "windows")]
    {
        hide_console_window();
    }

    let scheduler_json = file_service::read_file(&args.path)?;
    let scheduler: Scheduler = serde_json::from_str(&scheduler_json)?;

    #[cfg(target_os = "windows")]
    {
        enable_schtask(&scheduler.name, &scheduler.cron, &args.path.to_string_lossy(), &args.config);
    }

    if args.first.to_lowercase() == "y" {
        return Ok(());
    }

    let content = file_service::read_file(&PathBuf::from(scheduler.root))?;

    let entries = template_to_dir_entry::map(content)?;

    let commands = dir_entry_to_commands::map(entries)?;
    let hostname = gethostname::gethostname().to_string_lossy().to_string();
    let root = format!("{}/{}/{}", hostname, scheduler.name, Utc::now().format("%Y_%m_%d_and_%Hh_%Mm_%Ss"));

    for (cloud, _) in scheduler.clouds {
        commands.iter()
            .for_each(|command| {
                command.spawn_rclone_command(&cloud.name(&args.config), &root, scheduler.speed);
                sleep(Duration::from_millis(50));
            });
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe { GetConsoleWindow() };

    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }


    unsafe { winapi::um::wincon::FreeConsole() };
}

#[cfg(target_os = "windows")]
fn gen_action(winkey: &str, path: &str, config: &str) -> String {
    format!(
        r#"{winkey} -p {path} -f n -c {config}"#,
        winkey = winkey,
        path = path,
        config = config
    )
}


#[cfg(target_os = "windows")]
fn enable_schtask(name: &str, cron: &str, path: &str, config_path: &PathBuf) {
    let config_json = file_service::read_file(config_path).unwrap();
    println!("{config_json}");
    let config: Config = serde_json::from_str(&config_json).unwrap();
    let winkey = config.paths.watcher_backup;

    let now = Local::now();
    let next = parse(cron, &now).unwrap();
    let date = format!("{:02}/{:02}/{}", next.month(), next.day(), next.year());
    let time = format!("{:02}:{:02}:{:02}", next.hour(), next.minute(), next.second());


    let action = gen_action(&winkey, &path, &config_path.to_str().unwrap());

    let _ = Command::new("schtasks")
        .arg("/create")
        .arg("/tn")
        .arg(name)
        .arg("/tr")
        .arg(action)
        .arg("/sc")
        .arg("ONCE")
        .arg("/sd")
        .arg(date)
        .arg("/st")
        .arg(time)
        .arg("/f")
        .arg("/Rl")
        .arg("HIGHEST")
        .output()
        .expect("Failed to execute command");
}