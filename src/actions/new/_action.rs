use std::env;
use std::path::Path;
use std::process::exit;

use colored::Colorize;
use email_address::EmailAddress;

use inquire::validator::Validation;
use inquire::{Select, Text, CustomUserError};
use console::Term;

use convert_case::{Casing, Case};
use octocrab::Octocrab;
use tokio::runtime::Handle;
use unicode_segmentation::UnicodeSegmentation;

use crate::actions::Action;
use crate::commands::NewCommandOpts;

use crate::util::{ProjectTypes, FilePathCompleter};

use futures::executor::block_on;

use spinoff::{Spinner, Spinners, Color};

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

    fn make_big_pascal_case(str: &str) -> String {
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
} impl<'a> Action for NewAction<'a> {
    fn on_project(&self) {
        println!("{}", match env::var("AKJO_GITHUB_TOKEN") { 
            Ok(value) => value, 
            Err(_) => String::from("") 
        });

        // Ask if ready to create project
        if match Select::new("Are you ready to begin your new project?", vec![
            "Yes",
            "No",
        ]).prompt() {
            Ok(choice) => choice == "No",
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        } {
            println!("Alright, see you later!");
            exit(0); 
        }

        Term::stdout().clear_screen().unwrap();

        // Ask for project type
        let project_type = match Select::new("Let's start by defining the type of project you want to create.", ProjectTypes::get_all_names()).prompt() {
            Ok(choice) => ProjectTypes::from_name(&choice).unwrap(),
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // Info about three names
        match Select::new("Now we need to define the title, name and ID for the project.", vec![
            "Alright!",
        ]).without_help_message().prompt() {
            Ok(_) => {},
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        }

        // Ask for project title
        let project_title = match Text::new("Project Title:").with_initial_value(
            self.subject_name
        ).prompt() {
            Ok(title) => title,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // Ask for project name (PascalCase | PascalCASE)
        let project_name = match Text::new("Project Name:").with_initial_value(
            self.subject_name.to_case(Case::Pascal).as_str()
        ).with_validator(|input: &str| -> Result<Validation, CustomUserError> {
            if input.is_case(Case::Pascal) || Self::make_big_pascal_case(input).is_case(Case::Pascal) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Project name must be in PascalCase or PascalCASE!".into()))
            }
        }).with_help_message("The project name must be in PascalCase or PascalCASE").prompt() {
            Ok(name) => name,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // Ask for project id (kebab-case)
        let project_id = match Text::new("Project ID:").with_initial_value(
            self.subject_name.to_case(Case::Kebab).as_str()
        ).with_validator(|input: &str| -> Result<Validation, CustomUserError> {
            if input.is_case(Case::Kebab) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Project ID must be in kebab-case!".into()))
            }
        }).with_help_message("The project ID must be in kebab-case").prompt() {
            Ok(id) => id,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // Info about additional info about the project
        match Select::new("Great! We also need to define some additional information about the project.", vec![
            "Got it!",
        ]).without_help_message().prompt() {
            Ok(_) => {},
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        }

        // Ask for project description
        let project_description = match Text::new("Project Description:").with_initial_value(
            match project_type {
                ProjectTypes::EmptyProject => "An empty project.",
                ProjectTypes::EmptyRustBinary => "An empty Rust binary project.",
                ProjectTypes::EmptyRustLibrary => "An empty Rust library project.",
                ProjectTypes::RustCliApp => "A Rust CLI app project.",
            }
        ).prompt() {
            Ok(description) => description,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // Ask for project version
        let project_version = match Text::new("Project Version:").with_initial_value(
            "0.1.0"
        ).prompt() {
            Ok(version) => version,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // Info about additional info about the author
        match Select::new("Finally we also need some more information about you!", vec![
            "Sure!",
        ]).without_help_message().prompt() {
            Ok(_) => {},
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        }

        // Ask for author name
        let author_name = match Text::new("Author Name:").with_default(
           "Lukas Küffer"
        ).prompt() {
            Ok(name) => name,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // Ask for author email
        let author_email = match Text::new("Author E-Mail:").with_default(
            "lukas.kueffer@outlook.com"
        ).with_validator(|input: &str| -> Result<Validation, CustomUserError> {
            if EmailAddress::is_valid(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("E-Mail address is invalid!".into()))
            }
        }).prompt() {
            Ok(email) => email,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // Ask for author github
        let author_github = match Text::new("Author GitHub:").with_default(
            "Akjo03"
        ).prompt() {
            Ok(github) => github,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // Ask for project path with info at the end
        let project_path = match Text::new("Finally, where do you want to save your new project?").with_initial_value(
            env::current_dir().unwrap().join(&project_name).to_str().unwrap()
        ).with_validator(|input: &str| -> Result<Validation, CustomUserError> {
            if Path::new(input).exists() {
                Ok(Validation::Invalid("Path already exists!".into()))
            } else {
                Ok(Validation::Valid)
            }
        }).with_autocomplete(FilePathCompleter::default()).prompt() {
            Ok(path) => path,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        };

        // === Create project ===
        let octocrab_instance = match Octocrab::builder()
            .personal_token(
                match env::var("AKJO_GITHUB_TOKEN") { 
                    Ok(value) => value, 
                    Err(_) => String::from("") 
                }
            ).build() {
                Ok(octocrab) => octocrab,
                Err(_) => {
                    eprintln!("Error: Failed to connect and authenticate to GitHub!");
                    eprintln!("Please make sure that you have set the AKJO_GITHUB_TOKEN environment variable!");
                    exit(-1);
                },
            };

        // Create a new GitHub repo using the appropriate template repo and project name
        {
            let spinner = Spinner::new(Spinners::Dots12, format!("Creating a GitHub repo for {}...", project_name), Color::White);
            
            match block_on(
                octocrab_instance.repos("AkjoStudios", project_type.get_template_repo())
                    .generate(&project_name)
                    .owner("AkjoStudios")
                    .description(&project_description)
                    .private(false)
                    .send()
            ) {
                Ok(_) => {
                    spinner.stop_and_persist(format!("{}", ">".green()).as_str(), "Successfully created GitHub repo!");
                },
                Err(err) => {
                    spinner.stop_and_persist(format!("{}", "X".red()).as_str(), format!("Failed to create GitHub repo: {}", err).as_str());
                    exit(-1);
                },
            }
            
        }
        
        // Clone the GitHub repo to the specified path.

        // Replace the placeholders inside the project with the specified values

        // Add a .akjocli file to the project that holds all relevant information.

        // Commit and push the changes to the GitHub repo.

        // Add an entry for this new project with the required values to the AkjoRepo

        // Print a success message
    }
}