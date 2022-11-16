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
                    Ok(output) => {
                        spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Chocolatey v{} is already installed!", str::from_utf8(&output.stdout).unwrap()).as_str());
                    },
                    Err(_) => {
                        match Command::new("%SystemRoot%\\System32\\WindowsPowerShell\\v1.0\\powershell.exe")
                        .arg("-NoProfile")
                        .arg("-InputFormat")
                        .arg("None")
                        .arg("-ExecutionPolicy")
                        .arg("Bypass")
                        .arg("-Command")
                        .arg("[System.Net.ServicePointManager]::SecurityProtocol = 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))")
                        .status() {
                            Ok(_) => {
                                spinner.stop_and_persist(format!("{}", ">".green()).as_str(), "Successfully installed chocolatey!");
                            }, 
                            Err(_) => {
                                spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to install chocolatey!");
                            }
                        }
                    }
                }
        }
    }
}