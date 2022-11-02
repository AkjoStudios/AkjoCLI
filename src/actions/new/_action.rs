use std::process::exit;

use console::Term;
use convert_case::{Casing, Case};
use requestty::{Question, prompt_one, prompt};
use unicode_segmentation::UnicodeSegmentation;

use crate::actions::Action;
use crate::commands::NewCommandOpts;

use crate::util::ProjectTypes;

pub struct NewAction<'a> {
    subject_name: &'a String,
    _options: &'a NewCommandOpts,
} impl<'a> NewAction<'a> {
    pub fn new(subject_name: &'a String, _options: &'a NewCommandOpts) -> Self {
        Self {
            subject_name,
            _options,
        }
    }

    fn is_non_alphabetic_uppercase(char: char) -> bool {
        !(char.is_alphabetic() && char.is_uppercase())
    }

    fn make_big_kebab_case(str: &str) -> String {
        let last_upper_word_start = 
            str.graphemes(true).count()
            - str.graphemes(true)
                .rev()
                .position(
                    |grapheme| 
                    grapheme.chars().all(Self::is_non_alphabetic_uppercase)
                ).unwrap();

        let last_word = str.get(last_upper_word_start..).unwrap();
        
        println!("last_word: {}", last_word);
        if last_word.is_empty() { 
            str.to_string()
        } else {
            format!("{}{}", str.get(..last_upper_word_start).unwrap(), last_word.to_case(Case::Pascal))
        }
    }
} impl<'a> Action for NewAction<'a> {
    fn on_project(&self) {
        if match prompt_one(Question::select("start")
            .message(format!("Are you ready to begin your new project called {}?", self.subject_name))
            .choices(vec![
                "Yes",
                "No",
            ]).build()
        ) {
            Ok(answer) => match answer.as_list_item() {
                Some(answer) => answer.text == "No",
                None => true,
            },
            Err(_) => {
                eprintln!("Failed to parse question!");
                exit(-1);
            },
        } {
            println!("Alright then, see you next time :)");
            exit(0);
        }

        Term::stdout().clear_screen().unwrap();

        let project_type = match prompt_one(Question::select("project_type")
            .message("Alright, let's get started by choosing the type of project you want to create.")
            .choices(vec![
                "Empty Project",
                "Rust Binary",
                "Rust Library",
                "Rust CLI App",
            ]).build()
        ) {
            Ok(answer) => match answer.as_list_item() {
                Some(answer) => ProjectTypes::from_name(&answer.text).unwrap(),
                None => ProjectTypes::EmptyProject,
            },
            Err(_) => {
                eprintln!("Failed to parse question!");
                exit(-1);
            },
        };

        match prompt_one(Question::select("three_name_info")
            .message("Great. We now have to define the ID, name and title of the project.")
            .choice("Alright!")
            .build()
        ) {
            Ok(_) => {},
            Err(_) => {
                eprintln!("Failed to parse question!");
                exit(-1);
            }
        };

        let (project_title, project_name, project_id) = match prompt([
            Question::input("project_title")
                .message("Project Title")
                .default(self.subject_name)
                .build(),
            Question::input("project_name")
                .message("Project Name (PascalCase | PascalCASE)")
                .default(self.subject_name.to_case(Case::Pascal))
                .validate(
                    |project_name, _| 
                    if project_name.is_case(Case::Pascal) || NewAction::<'a>::make_big_kebab_case(project_name).is_case(Case::Pascal) {
                        Ok(())
                    } else {
                        Err("The name of your new project must be in PascalCase!".to_string())
                    }
                )
                .build(),
            Question::input("project_id")
                .message("Project ID (kebab-case)")
                .default(self.subject_name.to_case(Case::Kebab))
                .validate(
                    |project_id, _| 
                    if project_id.is_case(Case::Kebab) || NewAction::<'a>::make_big_kebab_case(project_id).is_case(Case::Kebab) {
                        Ok(())
                    } else {
                        Err("The id of your new project must be in kebab-case!".to_string())
                    }
                )
                .build()
        ]) {
            Ok(answers) => {
                (
                    match answers.get("project_title") {
                        Some(value) => match value.as_string() {
                            Some(value) => value.to_string(),
                            None => self.subject_name.to_string()
                        },
                        None => self.subject_name.to_string()
                    },
                    match answers.get("project_name") {
                        Some(value) => match value.as_string() {
                            Some(value) => value.to_string(),
                            None => self.subject_name.to_case(Case::Pascal)
                        },
                        None => self.subject_name.to_case(Case::Pascal)
                    },
                    match answers.get("project_id") {
                        Some(value) => match value.as_string() {
                            Some(value) => value.to_string(),
                            None => self.subject_name.to_case(Case::Kebab)
                        },
                        None => self.subject_name.to_case(Case::Kebab)
                    },
                )
            },
            Err(_) => {
                eprintln!("Failed to parse question!");
                exit(-1);
            }
        };
    }
}