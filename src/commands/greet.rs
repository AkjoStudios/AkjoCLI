use colored::*;

pub struct GreetCommand {
    name: String
} impl GreetCommand {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }

    pub fn run(&self) {
        println!("Hello, {}!", self.name.green())
    }
}