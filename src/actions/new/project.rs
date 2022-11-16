use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use std::process::exit;
use std::process::Command;
use colored::Colorize;
use email_address::EmailAddress;
use inquire::validator::Validation;
use inquire::{Select, Text, CustomUserError};
use console::Term;
use walkdir::WalkDir;
use convert_case::{Casing, Case};
use octocrab::Octocrab;
use futures::executor::block_on;
use spinoff::{Spinner, Spinners, Color};
use crate::util::{ProjectTypes, FilePathCompleter, NewProject};
use super::NewAction;

pub struct NewProjectAction; impl NewProjectAction {
    pub fn create(action: &NewAction) {
        NewProjectAction::create_project(&NewProjectAction::ask_project_info(action));
    }

    fn create_project(project_info: &NewProject) {
        Self::create_github_repo(Self::get_octocrab(), project_info);
        Self::clone_github_repo(project_info);
        Self::add_akjocli_file(project_info);
        Self::replace_placeholders(project_info);
        Self::commit_new_project_repo(project_info);
        Self::create_akjo_repo_entry();

        match Select::new(format!("Successfully created new project {}!", project_info.get_project_name()).as_str(), vec![
            "Exit",
        ]).without_help_message().prompt() {
            Ok(_) => {},
            Err(_) => exit(-1),
        }

        Term::stdout().clear_screen().unwrap();
    }

    fn ask_project_info(action: &NewAction) -> NewProject {
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

        let mut new_project = NewProject::new();
        // Ask for project type
        new_project.set_project_type(match Select::new("Let's start by defining the type of project you want to create.", ProjectTypes::get_all_names()).prompt() {
            Ok(choice) => ProjectTypes::from_name(&choice).unwrap(),
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        });

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
        new_project.set_project_title(match Text::new("Project Title:").with_initial_value(
            action.subject_name
        ).prompt() {
            Ok(title) => title,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        });

        // Ask for project name (PascalCase | PascalCASE)
        new_project.set_project_name(match Text::new("Project Name:").with_initial_value(
            action.subject_name.to_case(Case::Pascal).as_str()
        ).with_validator(|input: &str| -> Result<Validation, CustomUserError> {
            if input.is_case(Case::Pascal) || NewAction::make_big_pascal_case(input).is_case(Case::Pascal) {
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
        });

        // Ask for project id (kebab-case)
        new_project.set_project_id(match Text::new("Project ID:").with_initial_value(
            action.subject_name.to_case(Case::Kebab).as_str()
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
        });

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
        new_project.set_project_description(match Text::new("Project Description:").with_initial_value(
            match new_project.get_project_type() {
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
        });

        // Ask for project version
        new_project.set_project_version(match Text::new("Project Version:").with_initial_value(
            "0.1.0"
        ).prompt() {
            Ok(version) => version,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        });

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
        new_project.set_author_name(match Text::new("Author Name:").with_default(
           "Lukas Küffer"
        ).prompt() {
            Ok(name) => name,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        });

        // Ask for author email
        new_project.set_author_email(match Text::new("Author E-Mail:").with_default(
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
        });

        // Ask for author github
        new_project.set_author_github(match Text::new("Author GitHub:").with_default(
            "Akjo03"
        ).prompt() {
            Ok(github) => github,
            Err(_) => {
                eprintln!("Error: Failed to parse question!");
                exit(-1);
            },
        });

        // Ask for project path with info at the end
        new_project.set_project_path(match Text::new("Finally, where do you want to save your new project?").with_initial_value(
            env::current_dir().unwrap().join(new_project.get_project_name()).to_str().unwrap()
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
        });

        new_project
    }

    fn get_octocrab() -> Octocrab {
        match Octocrab::builder()
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
            }
    }

    fn create_github_repo(octocrab_instance: Octocrab, project_info: &NewProject) {
        let spinner = Spinner::new(Spinners::Dots12, format!("Creating a GitHub repo for {}...", project_info.get_project_name()), Color::White);
            
        match block_on(
            octocrab_instance.repos("AkjoStudios", project_info.get_project_type().clone().get_template_repo())
                .generate(project_info.get_project_name())
                .owner("AkjoStudios")
                .description(project_info.get_project_description())
                .private(false)
                .send()
        ) {
            Ok(_) => {
                spinner.stop_and_persist(format!("{}", ">".green()).as_str(), "Successfully created GitHub repo!");
            },
            Err(_) => {
                spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to create GitHub repo! Please make sure that you have set the AKJO_GITHUB_TOKEN environment variable!");
                exit(-1);
            },
        }
    }

    fn clone_github_repo(project_info: &NewProject) {
        let spinner = Spinner::new(Spinners::Dots12, format!("Cloning GitHub repo to {}", project_info.get_project_path()), Color::White);

            match Command::new("git")
                .arg("clone")
                .arg(format!("https://github.com/AkjoStudios/{}.git", project_info.get_project_name()))
                .status() {
                    Ok(_) => {
                        spinner.stop_and_persist(format!("{}", ">".green()).as_str(), "Successfully created GitHub repo!");
                    },
                    Err(_) => {
                        spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to create GitHub repo! Please make sure that you have set the AKJO_GITHUB_TOKEN environment variable!");
                        exit(-1);
                    },
                }
    }

    fn add_akjocli_file(project_info: &NewProject) {
        let spinner = Spinner::new(Spinners::Dots12, format!("Creating .akjocli file..."), Color::White);

        let mut file = match File::create(format!("{}/.akjocli", project_info.get_project_path())) {
            Ok(file) => file,
            Err(_) => {
                spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to create .akjocli file!");
                exit(-1);
            },
        };

        match file.write_all(format!(
            "
            project_type={}
            project_title={}
            project_name={}
            project_id={}
            project_description={}
            project_version={}
            author_name={}
            author_email={}
            author_github={}
            ",
            project_info.get_project_type().clone().get_name(),
            project_info.get_project_title(),
            project_info.get_project_name(),
            project_info.get_project_id(),
            project_info.get_project_description(),
            project_info.get_project_version(),
            project_info.get_author_name(),
            project_info.get_author_email(),
            project_info.get_author_github(),
        ).replace(" ", "").replace("\t", "").trim().as_bytes()) {
            Ok(_) => {
                spinner.stop_and_persist(format!("{}", ">".green()).as_str(), "Successfully created .akjocli file!");
            },
            Err(_) => {
                spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to create .akjocli file!");
                exit(-1);
            },
        }
    }

    fn replace_placeholders(project_info: &NewProject) {
        let spinner = Spinner::new(Spinners::Dots12, format!("Replacing placeholders inside the new project..."), Color::White);

        let valid_extensions = [
            "rs",
            "toml",
            "md",
            "json",
            "yml",
            "yaml",
            "txt",
        ];

        for entry in WalkDir::new(project_info.get_project_path()) {
            match entry {
                Ok(entry) => {
                    if entry.path().is_file() {
                        let extension = match entry.path().extension() {
                            Some(extension) => extension.to_str().unwrap(),
                            None => continue,
                        };

                        if !valid_extensions.contains(&extension) {
                            continue;
                        }
                        
                        let mut file = match File::open(entry.path()) {
                            Ok(file) => file,
                            Err(_) => {
                                spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to replace placeholders inside the new project!");
                                exit(-1);
                            },
                        };

                        let mut contents = String::new();

                        match file.read_to_string(&mut contents) {
                            Ok(_) => {
                                let mut file = match File::create(entry.path()) {
                                    Ok(file) => file,
                                    Err(_) => {
                                        spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to replace placeholders inside the new project!");
                                        exit(-1);
                                    },
                                };

                                match file.write_all(contents.replace("PROJECT_TITLE", project_info.get_project_title())
                                    .replace("PROJECT_NAME", project_info.get_project_name())
                                    .replace("PROJECT_ID", project_info.get_project_id())
                                    .replace("PROJECT_DESCRIPTION", project_info.get_project_description())
                                    .replace("PROJECT_VERSION", project_info.get_project_version())
                                    .replace("AUTHOR_NAME", project_info.get_author_name())
                                    .replace("AUTHOR_EMAIL", project_info.get_author_email())
                                    .replace("AUTHOR_GITHUB", project_info.get_author_github())
                                    .as_bytes()) {
                                        Ok(_) => {},
                                        Err(_) => {
                                            spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to replace placeholders inside the new project!");
                                            exit(-1);
                                        },
                                    }
                            },
                            Err(_) => {
                                spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to replace placeholders inside the new project!");
                                exit(-1);
                            },
                        }
                    }
                },
                Err(_) => {
                    spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to replace placeholders inside the new project!");
                    exit(-1);
                },
            }
        }

        spinner.stop_and_persist(format!("{}", ">".green()).as_str(), "Successfully replaced placeholders inside the new project!");
    }

    fn commit_new_project_repo(project_info: &NewProject) {
        let spinner = Spinner::new(Spinners::Dots12, format!("Committing and pushing changes to GitHub repo..."), Color::White);

        match Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(project_info.get_project_path())
            .status() {
                Ok(_) => {},
                Err(_) => {
                    spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to commit and push changes to GitHub repo!");
                    exit(-1);
                },
            }

        match Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("Initial commit")
            .current_dir(project_info.get_project_path())
            .status() {
                Ok(_) => {},
                Err(_) => {
                    spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to commit and push changes to GitHub repo!");
                    exit(-1);
                },
            }

        match Command::new("git")
            .arg("push")
            .arg("-u")
            .arg("origin")
            .arg("main")
            .current_dir(project_info.get_project_path())
            .status() {
                Ok(_) => {
                    spinner.stop_and_persist(format!("{}", ">".green()).as_str(), "Successfully committed and pushed changes to GitHub repo!");
                },
                Err(_) => {
                    spinner.stop_and_persist(format!("{}", "X".red()).as_str(), "Failed to commit and push changes to GitHub repo!");
                    exit(-1);
                },
            }
    }

    fn create_akjo_repo_entry() {
        let spinner = Spinner::new(Spinners::Dots12, format!("Adding entry to AkjoRepo for this new project..."), Color::White);

        spinner.stop_and_persist(format!("{}", ">".green()).as_str(), "NOT IMPLEMENTED: Successfully added entry to AkjoRepo for this new project!");
    }
}