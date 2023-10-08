pub mod editor {
    use std::ffi::OsStr;
    use std::fs::{
        create_dir_all, read_dir, remove_file, rename, set_permissions, DirEntry, File, FileType,
        ReadDir,
    };
    use std::io;
    use std::path::{Path, PathBuf};

    use crate::functions::get_dirs::get_dirs::get_base;
    use crate::functions::update_log::update_log::{append_bug_report, append_log};

    pub fn str_transform(input: &str) -> String {
        return input.to_lowercase().replace(" ", "").replace("-", "_");
    }

    fn move_dir(file_path: PathBuf) {}

    fn move_file(source: &Path, file_name: &OsStr, classification: &str) -> PathBuf {
        let base_path: String = format!("{}/Files/{}", get_base(), classification);
        let dir_binding_path: &Path = Path::new(base_path.as_str());

        if !dir_binding_path.exists() {
            _ = append_log(&format!("Creating {:?}", dir_binding_path));
            _ = create_dir_all(dir_binding_path);
        } else {
            _ = append_log(&format!("Existing Dir Path: {:?}", dir_binding_path));
        }

        let destination_path: PathBuf = dir_binding_path.join(file_name);
        _ = rename(source, &destination_path);
        _ = append_log(&format!("Moved {:?} to {:?}.", file_name, destination_path));

        return destination_path;
    }

    pub fn clean_folder(dir_path: PathBuf) {
        for entry in read_dir(&dir_path).unwrap() {
            let path: PathBuf = entry.unwrap().path();

            if path.is_file() {
                let new_destination: String = fix_casing(path).unwrap();
                classify_file(Path::new(&new_destination).to_path_buf());
            } else if path.is_dir() {
                // Recursive folder clean?
                println!("Recursive clean folder call?");
                println!("{:?}", path.to_str());
                // clean_folder(dir_path.to_owned());
            }
        }
    }

    pub fn classify_file(file_path: PathBuf) -> Option<PathBuf> {
        let path: &Path = Path::new(&file_path);
        let ext: &str = path.extension().unwrap().to_str().unwrap();
        let file_name: &OsStr = path.file_name().unwrap();
        let destination: Option<PathBuf>;

        // debug
        println!("{}", format!("File: {:?} - Ext: {}", file_name, ext));

        // Get file extension
        match ext {
            // Images
            "jpg" | "jpeg" | "avif" | "png" | "gif" | "bmp" | "webp" | "tiff" | "svg" => {
                Some(move_file(path, file_name, "Images"))
            }
            // Documents
            "pdf" | "doc" | "docx" | "txt" | "rtf" => Some(move_file(path, file_name, "Documents")),
            // Spreadsheets
            "xls" | "xlsx" | "csv" => Some(move_file(path, file_name, "Spreadsheets")),
            // Presentations
            "ppt" | "pptx" => Some(move_file(path, file_name, "Presentations")),
            // Audio
            "mp3" | "wav" | "aac" | "flac" | "ogg" => Some(move_file(path, file_name, "Audio")),
            // Video
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" => {
                Some(move_file(path, file_name, "Videos"))
            }
            // Compressed Folders - Might not need
            "zip" | "rar" | "7z" | "tar" | "gz" => {
                Some(move_file(path, file_name, "Compressed_Files"))
            }
            // Code
            "c" | "cpp" | "java" | "py" | "html" | "css" | "js" | "json" | "xml" | "sql" => {
                Some(move_file(path, file_name, "Code"))
            }
            // Executables
            "exe" | "app" => Some(move_file(path, file_name, "Executables")),
            // Fonts
            "ttf" | "otf" => Some(move_file(path, file_name, "Fonts")),
            // Databases
            "db" | "sqlite" => Some(move_file(path, file_name, "Databases")),
            // Other
            _ => Some(move_file(path, file_name, "Other")),
        }
    }

    pub fn fix_casing(file_path: PathBuf) -> Option<String> {
        let source_filename: &str = file_path.file_name().unwrap().to_str().unwrap();
        let transformed_filename: String = str_transform(source_filename);
        let destination: PathBuf = file_path.with_file_name(&transformed_filename);

        println!("From Case Fixer:");
        println!("source_filename: {}", source_filename);
        println!("destination file: {:?}", destination);
        println!(" ");

        if rename(&file_path, &destination).is_ok() {
            _ = append_log(
                format!("File Rename Successful: {}", destination.to_str().unwrap()).as_str(),
            );
            return Some(transformed_filename);
        }

        return None;
    }

    pub fn delete_file(file_path: PathBuf) {
        let source_filename: &str = file_path.file_name().unwrap().to_str().unwrap();
        let res: Result<(), io::Error> = remove_file(&file_path);

        match res {
            Ok(()) => {
                _ = append_log(&format!(
                    "File Deleted: {} at: {}",
                    source_filename,
                    file_path.to_str().unwrap()
                ));
            }
            Err(e) => {
                _ = append_bug_report(&format!(
                    "File Deletion Error at: {}",
                    file_path.to_str().unwrap()
                ));
            }
        }
    }
}
