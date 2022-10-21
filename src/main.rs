mod global;
mod commands;

use clap::{Parser, Subcommand};
use commands::{GreetCommand};
use global::{GlobalOpts};

#[derive(Debug, Parser)]
#[clap(name = "PROJECT_NAME", author = "AkjoStudios", version, about)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Greets the given name or uses \"World\" as default.")]
    Greet {
        #[clap(default_value_t=String::from("World"))]
        name: String
    }
}

fn main() {
    let args = App::parse();
    match args.command {
        Commands::Greet {name} => {
            GreetCommand::new(name).run()
        }
    }
}