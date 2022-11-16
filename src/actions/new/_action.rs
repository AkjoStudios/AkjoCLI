use convert_case::{Casing, Case};
use unicode_segmentation::UnicodeSegmentation;

use crate::{commands::NewCommandOpts, actions::Action};
use super::project::NewProjectAction;

pub struct NewAction<'a> {
    pub subject_name: &'a String,
    _options: &'a NewCommandOpts,
} impl<'a> NewAction<'a> {
    pub fn new(subject_name: &'a String, _options: &'a NewCommandOpts) -> Self {
        Self {
            subject_name,
            _options,
        }
    }

    pub fn is_non_alphabetic_uppercase(char: char) -> bool {
        !(char.is_alphabetic() && char.is_uppercase())
    }

    pub fn make_big_pascal_case(str: &str) -> String {
        let last_upper_word_start = 
            str.graphemes(true).count()
            - str.graphemes(true)
                .rev()
                .position(
                    |grapheme| 
                    grapheme.chars().all(Self::is_non_alphabetic_uppercase)
                ).unwrap();

        let last_word = str.get(last_upper_word_start..).unwrap();
        
        if last_word.is_empty() { 
            str.to_string()
        } else {
            format!("{}{}", str.get(..last_upper_word_start).unwrap(), last_word.to_case(Case::Pascal))
        }
    }

    pub fn reformat_akjocli_file(input: &String) -> String {
        let lines = input.lines().collect::<Vec<&str>>();
        let mut new_lines = Vec::<&str>::new();
        
        for line in lines {
            if line.is_empty() {
                continue;
            }

            new_lines.push(line.trim_start());
        }

        new_lines.join("\n")
    }
} impl<'a> Action for NewAction<'a> {
    fn on_project(&self) {
        NewProjectAction::create(self);
    }
}