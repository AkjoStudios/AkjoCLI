use std::process::exit;

use crate::actions::Action;
use crate::actions::DeleteAction;

use crate::subject_types::SubjectTypes;
use crate::commands::DeleteCommandOpts;

pub struct DeleteCommand {
    subject_type: SubjectTypes,
    subject_name: String,
    options: DeleteCommandOpts,
} impl DeleteCommand {
    pub fn new(subject_type: SubjectTypes, subject_name: String, options: DeleteCommandOpts) -> Self {
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
        DeleteAction::new(&self.subject_name, &self.options).on_action(self.subject_type.get_action())
    }
}