use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub hash: String,
}

impl FileEntry {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn hash(&self) -> &String {
        &self.hash
    }
}