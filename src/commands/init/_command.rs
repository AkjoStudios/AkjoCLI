use std::process::{exit, Command};
use colored::Colorize;
use spinoff::{Spinner, Spinners, Color};
use std::str;

use crate::commands::InitCommandOpts;

pub struct InitCommand {
    options: InitCommandOpts,
} impl InitCommand {
    pub fn new(options: InitCommandOpts) -> Self {
        Self {
            options,
        }
    }

    pub fn on_error(err: String) {
        eprintln!("{err}");
        exit(1);
    }

    pub fn run(&self) {
        {
            let spinner = Spinner::new(Spinners::Dots12, "Installing chocolatey...", Color::White);

            // Check if chocolatey is installed
            match Command::new("choco")
                .arg("-v")
                .output() {
                    Ok(_) => {
                        spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Chocolatey is already installed!").as_str());
                    },
                    Err(_) => {
                        match Command::new("C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe")
                        .arg("-NoProfile")
                        .arg("-InputFormat")
                        .arg("None")
                        .arg("-ExecutionPolicy")
                        .arg("Bypass")
                        .arg("-Command")
                        .arg("[System.Net.ServicePointManager]::SecurityProtocol = 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))")
                        .output() {
                            Ok(output) => {
                                if str::from_utf8(&output.stderr).unwrap().contains("elevated") {
                                    spinner.stop_and_persist(format!("{}", ">".red()).as_str(), format!("Failed to install chocolatey! Please run the AkjoCLI initialziation scriptas an administrator!").as_str());
                                    Self::on_error(str::from_utf8(&output.stderr).unwrap().to_string().red().to_string());
                                } else {
                                    spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Chocolatey has been successfully installed!").as_str());
                                }
                            }, 
                            Err(e) => {
                                spinner.stop_and_persist(format!("{}", "X".red()).as_str(), format!("Failed to install chocolatey! Error: {}", e).as_str());
                            }
                        }
                    }
                }
        }
    }
}