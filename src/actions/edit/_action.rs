use crate::actions::Action;

use crate::commands::EditCommandOpts;

pub struct EditAction<'a> {
    subject_name: &'a String,
    options: &'a EditCommandOpts,
} impl<'a> Action for EditAction<'a> {
    fn on_project(&self) {
        println!("Editing project {} with options {}", self.subject_name, self.options.to_string());
    }
} impl<'a> EditAction<'a> {
    pub fn new(subject_name: &'a String, options: &'a EditCommandOpts) -> Self {
        Self {
            subject_name,
            options,
        }
    }
}