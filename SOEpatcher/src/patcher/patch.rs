use super::{
    file_entry::FileEntry,
    program::{FileResult, Program},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Patch {
    old_version: String,
    new_version: String,
    deleted_files: Vec<FileEntry>,
    changed_files: Vec<FileEntry>,
    added_files: Vec<FileEntry>,
}

impl Patch {
    pub fn from(old_program: &Program, new_program: &Program) -> Self {
        let mut patch = Patch {
            old_version: old_program.version(),
            new_version: new_program.version(),
            deleted_files: Vec::new(),
            changed_files: Vec::new(),
            added_files: Vec::new(),
        };


        // TODO: Remake this code because it sucks
        for file in new_program.files() {
            match old_program.file_check(file) {
                FileResult::Changed => patch.changed_files.push(file.clone()),
                FileResult::Unchanged => {},
                FileResult::NonExistent => patch.added_files.push(file.clone()),
            }
        }

        for file in old_program.files() {
            match new_program.file_check(file) {
                FileResult::NonExistent => patch.deleted_files.push(file.clone()),
                _ => {}
            }
        }

        patch
    }
}
