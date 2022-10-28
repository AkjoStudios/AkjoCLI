use crate::actions::Action;

use crate::commands::NewCommandOpts;

pub struct NewAction<'a> {
    subject_name: &'a String,
    options: &'a NewCommandOpts,
} impl<'a> Action for NewAction<'a> {
    fn on_project(&self) {
        println!("Creating project {} with options {}", self.subject_name, self.options.to_string());
    }
} impl<'a> NewAction<'a> {
    pub fn new(subject_name: &'a String, options: &'a NewCommandOpts) -> Self {
        Self {
            subject_name,
            options,
        }
    }
}