use std::process::exit;

use crate::subject_types::SubjectTypes;

pub struct NewCommand {
    subject_type: SubjectTypes,
    subject_name: String,
} impl NewCommand {
    pub fn new(subject_type: SubjectTypes, subject_name: String) -> Self {
        Self {
            subject_type,
            subject_name,
        }
    }

    pub fn on_error(err: String) {
        eprintln!("{err}");
        exit(1);
    }

    pub fn run(&self) {
        println!("Creating a new {} called \"{}\"!", self.subject_type.to_string(), self.subject_name);
    }
}