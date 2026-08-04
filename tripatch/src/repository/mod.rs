use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,

    pub url: String,

    pub enabled: bool,

    pub priority: u32,
}

pub struct RepositoryManager {
    repos: Vec<Repository>,
}

impl RepositoryManager {
    pub fn new() -> Self {
        Self { repos: Vec::new() }
    }

    pub fn list(&self) -> &[Repository] {
        &self.repos
    }
}
