use clap::{Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Creates a new instance of the given subject type (project, etc.)")]
    New {
        #[clap()]
        subject_type: String
    }
}

#[derive(Debug, Subcommand)]
pub enum SubjectTypes {
    Project
}