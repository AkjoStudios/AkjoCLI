#[derive(Clone)]
pub enum ProjectTypes {
    EmptyProject,
    EmptyRustBinary,
    EmptyRustLibrary,
    RustCliApp,
} impl ProjectTypes {
    pub fn get_name(self) -> String {
        match self {
            ProjectTypes::EmptyProject => "Empty Project".to_string(),
            ProjectTypes::EmptyRustBinary => "Rust Binary".to_string(),
            ProjectTypes::EmptyRustLibrary => "Rust Library".to_string(),
            ProjectTypes::RustCliApp => "Rust CLI App".to_string(),
        }
    }

    pub fn from_name(name: &str) -> Result<ProjectTypes, ()> {
        match name {
            "Empty Project" => Ok(ProjectTypes::EmptyProject),
            "Rust Binary" => Ok(ProjectTypes::EmptyRustBinary),
            "Rust Library" => Ok(ProjectTypes::EmptyRustLibrary),
            "Rust CLI App" => Ok(ProjectTypes::RustCliApp),
            _ => Err(()),
        }
    }

    pub fn get_all_names() -> Vec<String> {
        vec![
            ProjectTypes::EmptyProject.get_name(),
            ProjectTypes::EmptyRustBinary.get_name(),
            ProjectTypes::EmptyRustLibrary.get_name(),
            ProjectTypes::RustCliApp.get_name(),
        ]
    }

    pub fn get_template_repo(self) -> String {
        match self {
            ProjectTypes::EmptyProject => "EmptyProjectTemplate".to_string(),
            ProjectTypes::EmptyRustBinary => "EmptyRustBinaryProjectTemplate".to_string(),
            ProjectTypes::EmptyRustLibrary => "EmptyRustLibraryProjectTemplate".to_string(), 
            ProjectTypes::RustCliApp => "EmptyRustCliAppTemplate".to_string(),
        }
    }
}