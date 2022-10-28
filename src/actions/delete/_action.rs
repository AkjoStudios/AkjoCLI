use crate::actions::Action;

use crate::commands::DeleteCommandOpts;

pub struct DeleteAction<'a> {
    subject_name: &'a String,
    options: &'a DeleteCommandOpts,
} impl<'a> Action for DeleteAction<'a> {
    fn on_project(&self) {
        println!("Deleting project {} with options {}", self.subject_name, self.options.to_string());
    }
} impl<'a> DeleteAction<'a> {
    pub fn new(subject_name: &'a String, options: &'a DeleteCommandOpts) -> Self {
        Self {
            subject_name,
            options,
        }
    }
}