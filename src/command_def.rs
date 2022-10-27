use clap::{Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Creates a new instance of the given subject type (project, etc.)")]
    New {
        #[clap()]
        subject_type: String
    },
    #[clap(about = "Edits an existing instance of the given subject type (project, etc.)")]
    Edit {
        #[clap()]
        subject_type: String
    },
    #[clap(about = "Deletes an existing instance of the given subject type (project, etc.)")]
    Delete {
        #[clap()]
        subject_type: String
    },
}