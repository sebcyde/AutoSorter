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
        logs_path: &Path,
    ) {
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
    }

    pub fn clean_folder(dir_path: PathBuf, logs_path: &Path) {
        let entries: ReadDir = read_dir(&dir_path).unwrap();

        for entry in entries {
            let entry: DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();

            if path.is_file() {
                fix_casing(path.clone(), logs_path);
                move_file(path, logs_path)
            } else if path.is_dir() {
                clean_folder(dir_path.to_owned(), logs_path);
            }
        }
    }

    pub fn move_file(file_path: PathBuf, logs_path: &Path) {
        let is_at_work: bool = Path::new("C:/Users/sebastian.cyde").exists();
        let final_dir: &Path;

        if is_at_work {
            final_dir = Path::new("C:/Users/sebastian.cyde/Documents/AutoSorter/Files");
        } else {
            final_dir = Path::new("C:/Users/SebCy/Documents/AutoSorter/Files");
        }

        let path: &Path = Path::new(&file_path);

        // Get file extension
        if let Some(extension) = path.extension() {
            if let Some(extension_str) = extension.to_str() {
                let file_name: &OsStr = path.file_name().unwrap();
                match extension_str {
                    // Images
                    "jpg" | "jpeg" | "avif" | "png" | "gif" | "bmp" | "webp" | "tiff" | "svg" => {
                        final_move_step(final_dir, path, file_name, "Images", logs_path);
                    }
                    // Documents
                    "pdf" | "doc" | "docx" | "txt" | "rtf" => {
                        final_move_step(final_dir, path, file_name, "Documents", logs_path);
                    }
                    // Spreadsheets
                    "xls" | "xlsx" | "csv" => {
                        final_move_step(final_dir, path, file_name, "Spreadsheets", logs_path);
                    }
                    // Presentations
                    "ppt" | "pptx" => {
                        final_move_step(final_dir, path, file_name, "Presentations", logs_path);
                    }
                    // Audio
                    "mp3" | "wav" | "aac" | "flac" | "ogg" => {
                        final_move_step(final_dir, path, file_name, "Audio", logs_path);
                    }
                    // Video
                    "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" => {
                        final_move_step(final_dir, path, file_name, "Videos", logs_path);
                    }
                    // Compressed Folders - Might not need
                    "zip" | "rar" | "7z" | "tar" | "gz" => {
                        final_move_step(final_dir, path, file_name, "Compressed_Files", logs_path);
                    }
                    // Code
                    "c" | "cpp" | "java" | "py" | "html" | "css" | "js" | "json" | "xml"
                    | "sql" => {
                        final_move_step(final_dir, path, file_name, "Code", logs_path);
                    }
                    // Executables
                    "exe" | "app" => {
                        final_move_step(final_dir, path, file_name, "Executables", logs_path);
                    }
                    // Fonts
                    "ttf" | "otf" => {
                        final_move_step(final_dir, path, file_name, "Fonts", logs_path);
                    }
                    // Databases
                    "db" | "sqlite" => {
                        final_move_step(final_dir, path, file_name, "Databases", logs_path);
                    }
                    // Other
                    _ => {
                        final_move_step(final_dir, path, file_name, "Unclassified", logs_path);
                    }
                }
            } else {
                println!("File extension is not a valid UTF-8 string.");
            }
        } else {
            println!("No file extension found.");
        }
    }

    pub fn move_dir(file_path: PathBuf, logs_path: &Path) {}

    pub fn fix_casing(file_path: PathBuf, logs_path: &Path) {
        if let Some(source_filename) = file_path.file_name().and_then(|os_str| os_str.to_str()) {
            let transformed_filename = source_filename
                .to_lowercase()
                .replace(" ", "")
                .replace("-", "_");

            let destination_path = file_path.with_file_name(&transformed_filename);

            if let Err(e) = rename(&file_path, &destination_path) {
                println!("Error renaming file: {:?}", e);
                return;
            }

            if !transformed_filename.eq_ignore_ascii_case(source_filename) {
                println!("File successfully renamed.");
                let log_message = format!(
                    "File Rename Successful: {} at: {}",
                    source_filename,
                    destination_path.to_str().unwrap_or("Invalid UTF-8 path")
                );
                let _ = append_log(&log_message, logs_path);
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
