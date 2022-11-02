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
 }