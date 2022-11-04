use std::str::FromStr;

use clap::Parser;

pub mod global_opts;
use global_opts::GlobalOpts;

pub mod subject_types;
use subject_types::SubjectTypes;

pub mod action_def;
use action_def::Actions;
pub mod actions;

pub mod command_def;
use command_def::Commands;
pub mod commands;
use commands::{NewCommand, EditCommand, DeleteCommand};

pub mod util;

#[derive(Debug, Parser)]
#[clap(name = "AkjoCLI", author = "AkjoStudios", version, about)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let args = App::parse();
    match args.command {
        Commands::New {subject_type, subject_name, options} => {
            match SubjectTypes::from_str(&subject_type) {
                Ok(subject_type) => NewCommand::new(subject_type, subject_name, options).run(),
                Err(err) => NewCommand::on_error(err),
            }
        }
        Commands::Edit {subject_type, subject_name, options} => {
            match SubjectTypes::from_str(&subject_type) {
                Ok(subject_type) => EditCommand::new(subject_type, subject_name, options).run(),
                Err(err) => EditCommand::on_error(err),
            }
        },
        Commands::Delete {subject_type, subject_name, options} => {
            match SubjectTypes::from_str(&subject_type) {
                Ok(subject_type) => DeleteCommand::new(subject_type, subject_name, options).run(),
                Err(err) => DeleteCommand::on_error(err),
            }
        },
    }
}