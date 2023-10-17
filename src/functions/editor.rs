pub mod editor {
    use std::ffi::OsStr;
    use std::fs::{self, create_dir_all, read_dir, remove_dir, remove_file, rename, File};
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};

    use crate::functions::get_dirs::get_dirs::get_base;
    use crate::functions::update_log::update_log::{append_bug_report, append_log};

    use walkdir::WalkDir;
    use zip::{write::FileOptions, ZipWriter};

    pub fn str_transform(input: &str) -> String {
        return input.to_lowercase().replace(" ", "").replace("-", "_");
    }

    pub fn is_directory_empty(path: &std::path::Path) -> Result<bool, std::io::Error> {
        let entries = fs::read_dir(path)?;

        for _ in entries {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn move_dir(file_path: &PathBuf) -> Option<PathBuf> {
        let base_path: String = format!("{}/Files/Folders", get_base());
        let dir_binding_path: &Path = Path::new(base_path.as_str());
        _ = create_dir_all(dir_binding_path);

        // New Dir Pathing
        let old_dir_name: &OsStr = file_path.file_name().unwrap();
        let new_dir_name: String = str_transform(old_dir_name.to_str().unwrap());
        let destination: PathBuf = dir_binding_path.join(new_dir_name);

        if is_directory_empty(&file_path).unwrap() {
            _ = remove_dir(&file_path);
            return None;
        }

        // Create the destination directory
        _ = create_dir_all(&destination);

        // Moving files
        for entry in read_dir(&file_path).unwrap() {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                let destination_path = destination.join(file_path.file_name().unwrap());
                if let Err(err) = rename(&file_path, &destination_path) {
                    println!("Error moving file: {:?}", err);
                }
            }
        }

        // Moving the directory (if all files were moved)
        if let Err(err) = rename(&file_path, &destination) {
            println!("Error moving directory: {:?}", err);
        }

        // Logging
        let log_content: String = format!("Moved {:?} to {:?}.", file_path, destination);
        _ = append_log(&log_content);

        return Some(destination);
    }

    pub fn move_file(source: &Path, file_name: &OsStr, classification: &str) -> PathBuf {
        let base_path: String = format!("{}/Files/{}", get_base(), classification);
        let dir_binding_path: &Path = Path::new(base_path.as_str());
        _ = create_dir_all(dir_binding_path);

        let destination_path: PathBuf = dir_binding_path.join(file_name);
        _ = rename(source, &destination_path);
        _ = append_log(&format!("Moved {:?} to {:?}.", file_name, destination_path));

        return destination_path;
    }

    pub fn clean_folder(dir_path: &PathBuf) {
        for entry in read_dir(&dir_path).unwrap() {
            let path: PathBuf = entry.unwrap().path();

            if path.is_file() {
                let new_destination: PathBuf = fix_casing(path);
                classify_file(new_destination);
            } else if path.is_dir() {
                move_dir(&path);
            }
        }

        // Remove any empty directories
        for entry in read_dir(&dir_path).unwrap() {
            let path: PathBuf = entry.unwrap().path();
            if path.is_dir() {
                _ = remove_dir(&dir_path);
            }
        }
    }

    pub fn classify_file(file_path: PathBuf) -> Option<PathBuf> {
        let path: &Path = Path::new(&file_path);
        let file_name: &OsStr = path.file_name().unwrap();

        if path.extension().is_none() {
            move_file(path, file_name, "Other");
            return None;
        }

        let ext: &str = path.extension().unwrap().to_str().unwrap();

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
            // Folders
            "zip" | "rar" | "7z" | "tar" | "gz" => Some(move_file(path, file_name, "Folders")),
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

    pub fn fix_casing(file_path: PathBuf) -> PathBuf {
        println!(" ");

        let source_filename: &str = file_path.file_name().unwrap().to_str().unwrap();
        let transformed_filename: String = str_transform(source_filename);
        let destination: PathBuf = file_path.with_file_name(&transformed_filename);
        let return_path: PathBuf = destination.clone();

        println!("From Case Fixer:");
        println!("source filepath: {:?}", &file_path);
        println!("source_filename: {:?}", &source_filename);
        println!("destination file: {:?}", &destination);
        println!(" ");

        _ = fs::rename(&file_path, &destination);
        let destination_str: &str = destination.to_str().unwrap();
        let log_content: String = format!("File Rename Successful: {}", destination_str);
        println!("Directory Casing Fixed");
        _ = append_log(&log_content);

        return return_path;
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
            Err(_) => {
                _ = append_bug_report(&format!(
                    "File Deletion Error at: {}",
                    file_path.to_str().unwrap()
                ));
            }
        }
    }

    use io::Read;

    pub fn zip_directory(source: &PathBuf) -> io::Result<()> {
        // Create a new ZIP archive.
        let zip_path: PathBuf = Path::with_extension(&source, "zip");
        let archive: File = File::create(zip_path).unwrap();
        let mut zip: ZipWriter<File> = ZipWriter::new(archive);

        // Walk through the source directory and add its contents to the ZIP archive.
        for entry in WalkDir::new(source) {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_file() {
                let relative_path = entry_path.strip_prefix(source);
                let options: FileOptions = FileOptions::default().unix_permissions(0o755);

                zip.start_file(relative_path.unwrap().to_str().unwrap(), options)?;

                let mut buffer = Vec::new();
                let source_file = File::open(entry_path)?;
                source_file.take(u64::MAX).read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
            }
        }

        Ok(())
    }
}
