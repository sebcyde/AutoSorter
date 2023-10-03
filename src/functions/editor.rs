pub mod editor {
    use std::ffi::OsStr;
    use std::fs::{
        read_dir, remove_file, rename, set_permissions, DirEntry, File, FileType, ReadDir,
    };
    use std::io;
    use std::path::{Path, PathBuf};

    use crate::functions::update_log::update_log::append_log;

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

        // let source_path: &str = file_path.to_str().unwrap();
        let path: &Path = Path::new(&file_path);

        // Get file extension
        if let Some(extension) = path.extension() {
            if let Some(extension_str) = extension.to_str() {
                println!("File extension: {}", extension_str);

                let ext: PathBuf = Path::new(extension_str).to_path_buf();
                let file_name: &OsStr = path.file_name().unwrap();
                let binding: PathBuf = Path::new(final_dir).join(&ext).join(file_name);
                let binding_path: &Path = binding.as_path();
                println!("Moved to: {:?}", binding_path);

                match extension_str {
                    "txt" => {
                        _ = rename(path, binding_path);
                        _ = append_log(&format!("Moved {:?}", file_name), logs_path);
                        println!("TXT File moved successfully");
                    }
                    "js" | "ts" => {
                        _ = rename(path, binding_path);
                        println!("JS/TS File moved successfully");
                        _ = append_log(&format!("Moved {:?}", file_name), logs_path);

                        // let js_ext: PathBuf = Path::new("JS").to_path_buf();
                        // let binding: PathBuf = Path::new(final_dir).join(&js_ext);
                        // let js_path: &Path = binding.as_path();
                        // let file_name: &OsStr = path.file_name().unwrap();
                        // _ = rename(path, js_path);

                        // println!("JS/TS File moved successfully");
                        // let _ = append_log(
                        //     &format!("Moved {:?} to: {:?}", file_name, js_path),
                        //     logs_path,
                        // );
                    }
                    "jpg" | "jpeg" | "png" => {
                        _ = rename(path, binding_path);
                        println!("Image File moved successfully");
                        _ = append_log(&format!("Moved {:?}", file_name), logs_path);

                        // let img_ext: PathBuf = Path::new("IMAGES").to_path_buf();
                        // let binding: PathBuf = Path::new(final_dir).join(&img_ext);
                        // let img_path: &Path = binding.as_path();
                        // let file_name: &OsStr = path.file_name().unwrap();
                        // _ = rename(path, img_path);

                        // println!("Image file moved successfully");
                        // let _ = append_log(
                        //     &format!("Moved {:?} to: {:?}", file_name, img_path),
                        //     logs_path,
                        // );
                    }
                    "mp4" => {
                        let video_ext: PathBuf = Path::new("VIDEOS").to_path_buf();
                        let binding: PathBuf = Path::new(final_dir).join(&video_ext);
                        let video_path: &Path = binding.as_path();
                        let file_name: &OsStr = path.file_name().unwrap();
                        _ = rename(path, video_path);

                        println!("Video file moved successfully");
                        let _ = append_log(
                            &format!("Moved {:?} to: {:?}", file_name, video_path),
                            logs_path,
                        );
                    }
                    "pdf" => {
                        let pdf_ext: PathBuf = Path::new("PDFs").to_path_buf();
                        let binding: PathBuf = Path::new(final_dir).join(&pdf_ext);
                        let pdf_path: &Path = binding.as_path();
                        let file_name: &OsStr = path.file_name().unwrap();
                        _ = rename(path, pdf_path);

                        println!("pdf file moved successfully");
                        let _ = append_log(
                            &format!("Moved {:?} to: {:?}", file_name, pdf_path),
                            logs_path,
                        );
                    }
                    "exe" => {
                        let exe_ext: PathBuf = Path::new("EXE").to_path_buf();
                        let binding: PathBuf = Path::new(final_dir).join(&exe_ext);
                        let exe_path: &Path = binding.as_path();
                        let file_name: &OsStr = path.file_name().unwrap();
                        _ = rename(path, exe_path);

                        println!("exe file moved successfully");
                        let _ = append_log(
                            &format!("Moved {:?} to: {:?}", file_name, exe_path),
                            logs_path,
                        );
                    }
                    _ => {
                        println!("Unknown file type.");
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
