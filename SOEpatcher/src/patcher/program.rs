use super::file_entry::FileEntry;
use base64ct::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Program {
    version: String,
    files: Vec<FileEntry>,
}
// TODO: find better name
pub enum FileResult {
    Unchanged,
    Changed,
    NonExistent,
}

impl Program {
    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn files(&self) -> &Vec<FileEntry> {
        &self.files
    }

    pub fn file_check(&self, searched_file: &FileEntry) -> FileResult {
        for file in self.files() {
            if file.path() == searched_file.path() {
                if file.hash() != searched_file.hash() {
                    return FileResult::Changed;
                } else {
                    return FileResult::Unchanged;
                }
            }
        }

        FileResult::NonExistent
    }

    pub fn from_path(path: &Path, version: String) -> Self {
        let mut result = Program {
            version: version,
            files: Vec::new(),
        };

        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();

            let hash = Sha256::digest(fs::read(entry.path()).unwrap());
            let hash_result = Base64::encode_string(&hash);

            result.files.push(FileEntry {
                name: entry.file_name().into_string().unwrap(),
                path: String::from(entry.path().to_str().unwrap()),
                hash: hash_result,
            })
        }

        result
    }
}
