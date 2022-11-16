use super::ProjectTypes;

pub struct NewProject {
    project_type: ProjectTypes,
    project_title: String,
    project_name: String,
    project_id: String,
    project_description: String,
    project_version: String,
    author_name: String,
    author_email: String,
    author_github: String,
    project_path: String,
} impl NewProject {
    pub fn new() -> Self {
        Self {
            project_type: ProjectTypes::EmptyProject,
            project_title: String::new(),
            project_name: String::new(),
            project_id: String::new(),
            project_description: String::new(),
            project_version: String::new(),
            author_name: String::new(),
            author_email: String::new(),
            author_github: String::new(),
            project_path: String::new(),
        }
    }

    pub fn set_project_type(&mut self, project_type: ProjectTypes) {
        self.project_type = project_type;
    }

    pub fn set_project_title(&mut self, project_title: String) {
        self.project_title = project_title;
    }

    pub fn set_project_name(&mut self, project_name: String) {
        self.project_name = project_name;
    }

    pub fn set_project_id(&mut self, project_id: String) {
        self.project_id = project_id;
    }

    pub fn set_project_description(&mut self, project_description: String) {
        self.project_description = project_description;
    }

    pub fn set_project_version(&mut self, project_version: String) {
        self.project_version = project_version;
    }   

    pub fn set_author_name(&mut self, author_name: String) {
        self.author_name = author_name;
    }

    pub fn set_author_email(&mut self, author_email: String) {
        self.author_email = author_email;
    }

    pub fn set_author_github(&mut self, author_github: String) {
        self.author_github = author_github;
    }

    pub fn set_project_path(&mut self, project_path: String) {
        self.project_path = project_path;
    }

    pub fn get_project_type(&self) -> &ProjectTypes {
        &self.project_type
    }

    pub fn get_project_title(&self) -> &String {
        &self.project_title
    }

    pub fn get_project_name(&self) -> &String {
        &self.project_name
    }

    pub fn get_project_id(&self) -> &String {
        &self.project_id
    }

    pub fn get_project_description(&self) -> &String {
        &self.project_description
    }

    pub fn get_project_version(&self) -> &String {
        &self.project_version
    }

    pub fn get_author_name(&self) -> &String {
        &self.author_name
    }

    pub fn get_author_email(&self) -> &String {
        &self.author_email
    }

    pub fn get_author_github(&self) -> &String {
        &self.author_github
    }

    pub fn get_project_path(&self) -> &String {
        &self.project_path
    }
}