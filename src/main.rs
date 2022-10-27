use std::str::FromStr;

use clap::{Parser};

mod global_opts;
use global_opts::GlobalOpts;

mod commands;
use commands::{NewCommand, EditCommand, DeleteCommand};

mod command_def;
use command_def::Commands;
use subject_types::SubjectTypes;

mod subject_types;


#[derive(Debug, Parser)]
#[clap(name = "AkjoCLI", author = "AkjoStudios", version, about)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let args = App::parse();
    match args.command {
        Commands::New {subject_type} => {
            match SubjectTypes::from_str(&subject_type) {
                Ok(subject_type) => NewCommand::new(subject_type).run(),
                Err(err) => NewCommand::on_error(err),
            }
        }
        Commands::Edit {subject_type} => {
            match SubjectTypes::from_str(&subject_type) {
                Ok(subject_type) => EditCommand::new(subject_type).run(),
                Err(err) => EditCommand::on_error(err),
            }
        },
        Commands::Delete {subject_type} => {
            match SubjectTypes::from_str(&subject_type) {
                Ok(subject_type) => DeleteCommand::new(subject_type).run(),
                Err(err) => DeleteCommand::on_error(err),
            }
        },
    }
}