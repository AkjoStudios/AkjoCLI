use std::process::exit;

use colored::Colorize;

use crate::{action_def::Actions, util::base_dir};

pub trait Action {
    fn on_action(&self, action: Actions) {
        if !match base_dir::get_base_dir().as_path().try_exists() { Ok(exists) => exists, Err(_) => false } {
            println!("{}", "Failed to find base directory for AkjoCLI. Please run akjo init first!".red());
            exit(1);
        }
        match action {
            Actions::OnProject => self.on_project(),
        }
    }

    fn on_project(&self);
}