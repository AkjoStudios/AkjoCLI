use std::process::exit;

use crate::actions::Action;
use crate::actions::NewAction;

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
        NewAction::new(&self.subject_name, &self.options).on_action(self.subject_type.get_action())
    }
}