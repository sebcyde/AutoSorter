pub mod editor {
    use std::fs::{
        copy, create_dir_all, metadata, read_dir, remove_file, rename, set_permissions, FileType,
        ReadDir,
    };
    use std::path::{Path, PathBuf};

    use crate::functions::update_log::update_log::append_log;

    pub fn fix_casing(file_path: PathBuf, logs_path: &Path) {
        let source_filename: &str = file_path.file_name().unwrap().to_str().unwrap();

        let transformed_filename: String = source_filename
            .to_lowercase()
            .replace(" ", "")
            .replace("-", "_");

        let destination_path: PathBuf = file_path.with_file_name(transformed_filename);

        match rename(file_path.clone(), destination_path.clone()) {
            Ok(()) => {
                println!("File successfully renamed.");

                let _ = append_log(
                    &format!(
                        "File Rename Successful: {} at: {}",
                        source_filename,
                        destination_path.to_str().unwrap()
                    ),
                    logs_path,
                );
            }
            Err(e) => {
                println!("Error renaming file: {:?}", e);
            }
        }
    }

    pub fn delete_file(file_path: PathBuf, logs_path: &Path) {
        let source_filename: &str = file_path.file_name().unwrap().to_str().unwrap();

        match remove_file(file_path.clone()) {
            Ok(()) => {
                let _ = append_log(
                    &format!(
                        "File Deleted: {} at: {}",
                        source_filename,
                        file_path.to_str().unwrap()
                    ),
                    logs_path,
                );
            }
            Err(e) => {
                println!("Error renaming file: {:?}", e);
            }
        }
    }
}
