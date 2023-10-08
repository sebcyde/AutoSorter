pub mod editor {
    use std::ffi::OsStr;
    use std::fs::{
        create_dir_all, read_dir, remove_file, rename, set_permissions, DirEntry, File, FileType,
        ReadDir,
    };
    use std::io;
    use std::path::{Path, PathBuf};

    use crate::functions::update_log::update_log::append_log;

    fn final_move_step(
        destination: &Path,
        source: &Path,
        file_name: &OsStr,
        classification: &str,
    ) -> PathBuf {
        let dir_binding: PathBuf = Path::new(destination).join(classification);
        let dir_binding_path: &Path = dir_binding.as_path();

        if !dir_binding_path.exists() {
            println!("Creating {:?}", dir_binding_path);
            _ = append_log(&format!("Creating {:?}", dir_binding_path), logs_path);
            _ = create_dir_all(dir_binding_path);
        } else {
            println!("Existing Dir Path: {:?}", dir_binding_path);
        }

        let destination_path = dir_binding_path.join(file_name);
        _ = rename(source, &destination_path);
        _ = append_log(
            &format!("Moved {:?} to {:?}.", file_name, destination_path),
            logs_path,
        );

        println!("{} file moved successfully", classification);
        println!(" ");
        return destination_path;
    }

    pub fn clean_folder(dir_path: PathBuf) {
        let entries: ReadDir = read_dir(&dir_path).unwrap();

        for entry in entries {
            let entry: DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();

            if path.is_file() {
                fix_casing(&path);
                let destination: Option<PathBuf> = move_file(path);
                if destination.is_some() {
                    println!("File Move successful");
                } else {
                    println!("Error moving file");
                }
            } else if path.is_dir() {
                clean_folder(dir_path.to_owned());
            }
        }
    }

    pub fn move_file(file_path: PathBuf) -> Option<PathBuf> {
        let is_at_work: bool = Path::new("C:/Users/sebastian.cyde").exists();
        let final_dir: &Path;

        if is_at_work {
            final_dir = Path::new("C:/Users/sebastian.cyde/Documents/AutoSorter/Files");
        } else {
            final_dir = Path::new("C:/Users/SebCy/Documents/AutoSorter/Files");
        }

        let path: &Path = Path::new(&file_path);
        let destination: Option<PathBuf>;

        // Get file extension
        if let Some(extension) = path.extension() {
            if let Some(extension_str) = extension.to_str() {
                let file_name: &OsStr = path.file_name().unwrap();
                match extension_str {
                    // Images
                    "jpg" | "jpeg" | "avif" | "png" | "gif" | "bmp" | "webp" | "tiff" | "svg" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Images", logs_path);
                        return Some(destination);
                    }
                    // Documents
                    "pdf" | "doc" | "docx" | "txt" | "rtf" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Documents", logs_path);
                        return Some(destination);
                    }
                    // Spreadsheets
                    "xls" | "xlsx" | "csv" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Spreadsheets", logs_path);
                        return Some(destination);
                    }
                    // Presentations
                    "ppt" | "pptx" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Presentations", logs_path);
                        return Some(destination);
                    }
                    // Audio
                    "mp3" | "wav" | "aac" | "flac" | "ogg" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Audio", logs_path);
                        return Some(destination);
                    }
                    // Video
                    "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Videos", logs_path);
                        return Some(destination);
                    }
                    // Compressed Folders - Might not need
                    "zip" | "rar" | "7z" | "tar" | "gz" => {
                        let destination = final_move_step(
                            final_dir,
                            path,
                            file_name,
                            "Compressed_Files",
                            logs_path,
                        );
                        return Some(destination);
                    }
                    // Code
                    "c" | "cpp" | "java" | "py" | "html" | "css" | "js" | "json" | "xml"
                    | "sql" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Code", logs_path);
                        return Some(destination);
                    }
                    // Executables
                    "exe" | "app" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Executables", logs_path);
                        return Some(destination);
                    }
                    // Fonts
                    "ttf" | "otf" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Fonts", logs_path);
                        return Some(destination);
                    }
                    // Databases
                    "db" | "sqlite" => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Databases", logs_path);
                        return Some(destination);
                    }
                    // Other
                    _ => {
                        let destination =
                            final_move_step(final_dir, path, file_name, "Unclassified", logs_path);
                        return Some(destination);
                    }
                }
            } else {
                println!("File extension is not a valid UTF-8 string.");
                return None;
            }
        } else {
            println!("No file extension found.");
            return None;
        }
    }

    pub fn move_dir(file_path: PathBuf, logs_path: &Path) {}

    pub fn fix_casing(file_path: PathBuf) {
        if let Some(source_filename) = file_path.file_name().and_then(|os_str| os_str.to_str()) {
            let transformed_filename: String = source_filename
                .to_lowercase()
                .replace(" ", "")
                .replace("-", "_");

            let destination_path: PathBuf = file_path.with_file_name(&transformed_filename);

            if let Err(e) = rename(&file_path, &destination_path) {
                println!("Error renaming file: {:?}", e);
                return;
            }

            if !transformed_filename.eq_ignore_ascii_case(source_filename) {
                println!("File rename successful.");
                _ = append_log(
                    format!(
                        "File Rename Successful: {} at: {}",
                        source_filename,
                        destination_path.to_str().unwrap()
                    )
                    .as_str(),
                );
            }
        } else {
            println!("Error: Invalid file name.");
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
