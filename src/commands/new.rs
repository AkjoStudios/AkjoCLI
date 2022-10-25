pub struct NewCommand {
    subject_type: String
} impl NewCommand {
    pub fn new(subject_type: String) -> Self {
        Self {
            subject_type
        }
    }

    pub fn run(&self) {

    }
}