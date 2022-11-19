use std::{path::PathBuf, process::exit, fs};

use colored::Colorize;
use directories::BaseDirs;

pub fn get_base_dir() -> PathBuf {
    let base_dirs = match BaseDirs::new() {
        Some(base_dirs) => base_dirs,
        None => {
            println!("{}", "Failed to get base directories!".red().to_string());
            exit(1);
        }
    };

    base_dirs.config_dir().join("AkjoCLI")
}

pub fn create_base_dir() {
    let base_dir = get_base_dir();

    if !match base_dir.try_exists() { Ok(exists) => exists, Err(_) => false } {
        match fs::create_dir_all(&base_dir) {
            Ok(_) => {},
            Err(e) => {
                println!("{}", format!("Failed to create base directory! Error: {}", e).red().to_string());
                exit(1);
            }
        }
    }
}