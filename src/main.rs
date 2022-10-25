use clap::{Parser};

mod global;
use global::{GlobalOpts};

mod commands;
use commands::{NewCommand};

mod command_def;
use command_def::Commands;


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
            NewCommand::new(subject_type).run()
        }
    }
}