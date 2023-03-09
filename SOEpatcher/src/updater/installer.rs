use log::{error, info};
use std::fs;
use std::path::PathBuf;
use zip::ZipArchive;

pub fn install_archive(archive_path: PathBuf) {
    let file = match fs::File::open(archive_path) {
        Ok(file) => file,
        Err(error) => {
            error!("Unable to open the archive, this shouldn't have happened");
            info!("Message: {:?}", error);
            return;
        }
    };

    let mut archive = match ZipArchive::new(file) {
        Ok(archive) => archive,
        Err(error) => {
            error!("Unable to open the file as an archive");
            info!("Message: {:?}", error);
            return;
        }
    };

    if let Err(error) = archive.extract(".") {
        error!("Unable to extract the archive");
        info!("Message: {:?}", error);
    }
}
