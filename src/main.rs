use crate::functions::create_directory::create_dir::create_containers;
use crate::functions::update_log::update_log;
use crate::functions::watch_folders::watch_folders;
use std::fs;
use std::path::{Path, PathBuf};

pub mod functions {
    pub mod archive;
    pub mod clean_archive;
    pub mod create_directory;
    pub mod editor;
    pub mod update_log;
    pub mod watch_folders;
}

fn main() {
    println!("Starting AutoSorter...");

    let is_at_work: bool = Path::new("C:/Users/sebastian.cyde").exists();
    let base_path: &Path;
    let root_path: &Path;

    if is_at_work {
        base_path = Path::new("C:/Users/sebastian.cyde/Documents/AutoSorter");
        root_path = Path::new("C:/Users/sebastian.cyde");
    } else {
        base_path = Path::new("C:/Users/SebCy/Documents/AutoSorter");
        root_path = Path::new("C:/Users/SebCy");
    }

    let logs_ext: PathBuf = Path::new("logs").to_path_buf();
    let binding: PathBuf = Path::new(base_path).join(&logs_ext);
    let logs_path: &Path = binding.as_path();

    println!("Base path: {:?}", base_path);
    println!("Logs path: {:?}", logs_path);
    println!(" ");

    // Create main AutoSorter directory if it doesnt exist
    if !Path::new(base_path).exists() {
        println!("No AutoSorter directory found. Creating...");
        let _ = fs::create_dir_all(base_path);
        println!("Created successfully");
        println!(" ");
    };

    // Create directory for logs if it doesnt exist
    if !Path::new(&logs_path).exists() {
        println!("No logs directory found. Creating...");
        let _ = fs::create_dir_all(logs_path);
        println!("Created successfully");
        println!(" ");
    };

    // Create a log for today
    let _ = update_log::create_log(logs_path);

    create_containers(base_path);

    // Watch folders
    watch_folders::watch_downloads(root_path, &logs_path);
    // watch_folders::watch_documents(root_path, &logs_path);
}
