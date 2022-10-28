use std::str::FromStr;

use crate::Actions;

pub enum SubjectTypes {
    Project
} impl FromStr for SubjectTypes {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "project" => Ok(SubjectTypes::Project),
            _ => Err(format!("Subject type \"{}\" was not found!", input).to_string()),
        }
    }
} impl ToString for SubjectTypes {
    fn to_string(&self) -> String {
       match self {
        Self::Project => "project".to_string(),
       }
    }
} impl SubjectTypes {
    pub fn get_action(&self) -> Actions {
        match self {
            Self::Project => Actions::OnProject,
        }
    }
}