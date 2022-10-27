use std::process::exit;

use crate::subject_types::SubjectTypes;

pub struct DeleteCommand {
    subject_type: SubjectTypes
} impl DeleteCommand {
    pub fn new(subject_type: SubjectTypes) -> Self {
        Self {
            subject_type
        }
    }

    pub fn on_error(err: String) {
        eprintln!("{err}");
        exit(1);
    }

    pub fn run(&self) {
        println!("Running delete command on subject type \"{}\"!", self.subject_type.to_string());
    }
}