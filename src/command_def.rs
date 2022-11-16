use clap::Subcommand;

use crate::commands::{InitCommandOpts, NewCommandOpts, EditCommandOpts, DeleteCommandOpts};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init {
        #[clap(flatten)]
        options: InitCommandOpts,
    },
    #[clap(about = "Creates a new instance of the given subject type (project, etc.)")]
    New {
        #[clap()]
        subject_type: String,
        #[clap()]
        subject_name: String,
        #[clap(flatten)]
        options: NewCommandOpts,
    },
    #[clap(about = "Edits an existing instance of the given subject type (project, etc.)")]
    Edit {
        #[clap()]
        subject_type: String,
        #[clap()]
        subject_name: String,
        #[clap(flatten)]
        options: EditCommandOpts,
    },
    #[clap(about = "Deletes an existing instance of the given subject type (project, etc.)")]
    Delete {
        #[clap()]
        subject_type: String,
        #[clap()]
        subject_name: String,
        #[clap(flatten)]
        options: DeleteCommandOpts,
    },
}