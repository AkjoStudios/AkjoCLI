use std::process::exit;

use crate::actions::Action;
use crate::actions::EditAction;

use crate::subject_types::SubjectTypes;
use crate::commands::EditCommandOpts;

pub struct EditCommand {
    subject_type: SubjectTypes,
    subject_name: String,
    options: EditCommandOpts,
} impl EditCommand {
    pub fn new(subject_type: SubjectTypes, subject_name: String, options: EditCommandOpts) -> Self {
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
        EditAction::new(&self.subject_name, &self.options).on_action(self.subject_type.get_action())
    }
}