use std::{env, process::{exit, Command}, path::{PathBuf, Path}, fs};
use colored::Colorize;
use directories::BaseDirs;
use spinoff::{Spinner, Spinners, Color};
use std::str;
use os_info::{self, Type};
use is_elevated::is_elevated;

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
        match os_info::get().os_type() {
            Type::Windows => {
                Self::check_for_elevated_privileges();
                Self::install_chocolatey();
                Self::install_git_with_choco();
                Self::clone_akjo_repo();
                Self::add_akjocli_to_path();
            },
            Type::Ubuntu => {
                Self::check_for_elevated_privileges();
                Self::install_git_with_apt();
                Self::clone_akjo_repo();
                Self::add_akjocli_to_path();
            },
            Type::Macos => {
                Self::check_for_elevated_privileges();
                Self::install_brew();
                Self::install_git_with_brew();
                Self::clone_akjo_repo();
                Self::add_akjocli_to_path();
            },
            _ => {
                Self::on_error("AkjoCLI only supports Windows, Linux and MacOS!".to_string());
            }
        }

        println!("{}", "Initialization has been completed successfully! Thanks for using AkjoCLI!".green());
    }

    fn check_for_elevated_privileges() {
        if !is_elevated() {
            Self::on_error(format!("{}", "To initialize AkjoCLI, you need elevated privileges! Run this command again as an administrator.".red()));
        }
    }

    fn install_chocolatey() {
        let spinner = Spinner::new(Spinners::Dots12, "Installing chocolatey...", Color::White);

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
                        Ok(_) => {
                            spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Chocolatey has been successfully installed!").as_str());
                        }, 
                        Err(e) => {
                            spinner.stop_and_persist(format!("{}", "X".red()).as_str(), format!("Failed to install chocolatey! Error: {}", e).as_str());
                        }
                    }
                }
            }
    }

    fn install_git_with_choco() {
        let spinner = Spinner::new(Spinners::Dots12, "Installing git...", Color::White);

        match Command::new("git")
            .arg("--version")
            .output() {
                Ok(_) => {
                    spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Git is already installed!").as_str());
                },
                Err(_) => {
                    match Command::new("choco")
                    .arg("install")
                    .arg("git")
                    .arg("-y")
                    .output() {
                        Ok(output) => {
                            if str::from_utf8(&output.stderr).unwrap().contains("NOT successful") {
                                spinner.stop_and_persist(format!("{}", ">".red()).as_str(), format!("Failed to install git! Please run the AkjoCLI initialziation script as an administrator!").as_str());
                                Self::on_error(str::from_utf8(&output.stderr).unwrap().to_string().red().to_string());
                            } else {
                                spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Git has been successfully installed!").as_str());
                            }
                        }, 
                        Err(e) => {
                            spinner.stop_and_persist(format!("{}", "X".red()).as_str(), format!("Failed to install git! Error: {}", e).as_str());
                        }
                    }
                }
            }
    }

    fn install_git_with_apt() {
        let spinner = Spinner::new(Spinners::Dots12, "Installing git...", Color::White);

        match Command::new("git")
            .arg("--version")
            .output() {
                Ok(_) => {
                    spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Git is already installed!").as_str());
                },
                Err(_) => {
                    match Command::new("sudo")
                    .arg("apt-get")
                    .arg("install")
                    .arg("git")
                    .arg("-y")
                    .output() {
                        Ok(_) => {
                            spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Git has been successfully installed!").as_str());
                        }, 
                        Err(e) => {
                            spinner.stop_and_persist(format!("{}", "X".red()).as_str(), format!("Failed to install git! Error: {}", e).as_str());
                        }
                    }
                }
            }
    }

    fn install_brew() {
        let spinner = Spinner::new(Spinners::Dots12, "Installing Homebrew...", Color::White);

        match Command::new("brew")
            .arg("-v")
            .output() {
                Ok(_) => {
                    spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Homebrew is already installed!").as_str());
                },
                Err(_) => {
                    match Command::new("/bin/bash")
                    .arg("-c")
                    .arg("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)")
                    .output() {
                        Ok(_) => {
                            spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Homebrew has been successfully installed!").as_str());
                        }, 
                        Err(e) => {
                            spinner.stop_and_persist(format!("{}", "X".red()).as_str(), format!("Failed to install Homebrew! Error: {}", e).as_str());
                        }
                    }
                }
            }
    }

    fn install_git_with_brew() {
        let spinner = Spinner::new(Spinners::Dots12, "Installing git...", Color::White);

        match Command::new("git")
            .arg("--version")
            .output() {
                Ok(_) => {
                    spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Git is already installed!").as_str());
                },
                Err(_) => {
                    match Command::new("brew")
                    .arg("install")
                    .arg("git")
                    .output() {
                        Ok(_) => {
                            spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("Git has been successfully installed!").as_str());
                        }, 
                        Err(e) => {
                            spinner.stop_and_persist(format!("{}", "X".red()).as_str(), format!("Failed to install git! Error: {}", e).as_str());
                        }
                    }
                }
            }
    }

    fn get_base_dir() -> PathBuf {
        let base_dirs = match BaseDirs::new() {
            Some(base_dirs) => base_dirs,
            None => {
                Self::on_error("Failed to get base directories!".red().to_string());
                exit(1);
            }
        };

        let base_dir = base_dirs.config_dir().join("AkjoCLI");

        if !base_dir.exists() {
            match fs::create_dir_all(&base_dir) {
                Ok(_) => {},
                Err(e) => {
                    Self::on_error(format!("Failed to create base directory! Error: {}", e).red().to_string());
                    exit(1);
                }
            }
        }

        base_dir
    }

    fn clone_akjo_repo() {
        let spinner = Spinner::new(Spinners::Dots12, "Cloning AkjoRepo repository to base directory...", Color::White);

        if Path::new(&Self::get_base_dir().join("AkjoRepo")).exists() {
            spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("AkjoRepo repository is already cloned!").as_str());
            return;
        }

        match Command::new("git")
            .arg("clone")
            .arg("https://github.com/AkjoStudios/AkjoRepo.git")
            .current_dir(Self::get_base_dir())
            .output() {
                Ok(_) => {
                    spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("AkjoCLI repository has been successfully cloned!").as_str());
                },
                Err(e) => {
                    spinner.stop_and_persist(format!("{}", "X".red()).as_str(), format!("Failed to clone AkjoCLI repository! Error: {}", e).as_str());
                    Self::on_error(format!("{}", e).red().to_string());
                }
            }
    }

    fn add_akjocli_to_path() {
        let spinner = Spinner::new(Spinners::Dots12, "Adding AkjoCLI to PATH...", Color::White);

        if let Some(path) = env::var_os("PATH") {
            let mut paths = env::split_paths(&path).collect::<Vec<_>>();
            paths.push(PathBuf::from(match env::current_exe() {
                Ok(path) => path,
                Err(e) => {
                    Self::on_error(format!("Failed to get current executable path! Error: {}", e).red().to_string());
                    exit(1);
                }
            }).parent().unwrap().to_path_buf());

            let new_path = env::join_paths(paths).unwrap();

            match globalenv::set_var("PATH", match new_path.to_str() {
                Some(path) => path,
                None => {
                    Self::on_error("Failed to convert path to string!".red().to_string());
                    exit(1);
                }
            }) {
                Ok(_) => {
                    spinner.stop_and_persist(format!("{}", ">".green()).as_str(), format!("AkjoCLI has been successfully added to PATH!").as_str());
                },
                Err(e) => {
                    spinner.stop_and_persist(format!("{}", "X".red()).as_str(), format!("Failed to add AkjoCLI to PATH! Error: {}", e).as_str());
                    Self::on_error(format!("{}", e).red().to_string());
                }
            }
        }
    }
}