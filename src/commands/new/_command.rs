use std::process::exit;

use crate::subject_types::SubjectTypes;
use crate::commands::NewCommandOpts;

pub struct NewCommand {
    subject_type: SubjectTypes,
    subject_name: String,
    options: NewCommandOpts,
} impl NewCommand {
    pub fn new(subject_type: SubjectTypes, subject_name: String, options: NewCommandOpts) -> Self {
        Self {
            subject_type,
            subject_name,
            options,
        }
    }

    pub fn on_error(err: String) {
        eprintln!("{err}");
        exit(1);
    }

    pub fn run(&self) {
        println!("Creating a new {} called \"{}\" with options {}!", self.subject_type.to_string(), self.subject_name, self.options.to_string());
    }
}