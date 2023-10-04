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
    println!(" ");
    println!("Starting AutoSorter...");
    println!(" ");

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

    let logs_ext: PathBuf = Path::new("Logs").to_path_buf();
    let binding: PathBuf = Path::new(base_path).join(&logs_ext);
    let logs_path: &Path = binding.as_path();

    let bugs_ext: PathBuf = Path::new("Bugs").to_path_buf();
    let binding: PathBuf = Path::new(base_path).join(&bugs_ext);
    let bugs_path: &Path = binding.as_path();

    println!("Base path: {:?}", base_path);
    println!("Logs path: {:?}", logs_path);
    println!("Bug report path: {:?}", bugs_path);
    println!(" ");

    // Create main AutoSorter directory if it doesnt exist
    if !Path::new(base_path).exists() {
        println!("No AutoSorter directory found. Creating...");
        _ = fs::create_dir_all(base_path);
        println!("Created successfully");
        println!(" ");
    };

    // Create directory for logs if it doesnt exist
    if !Path::new(&logs_path).exists() {
        println!("No logs directory found. Creating...");
        _ = fs::create_dir_all(logs_path);
        println!("Created successfully");
        println!(" ");
    };

    // Create directory for bug logger if it doesnt exist
    if !Path::new(&bugs_path).exists() {
        println!("No bug logger directory found. Creating...");
        _ = fs::create_dir_all(bugs_path);
        println!("Created successfully");
        println!(" ");
    };

    // Create a activity log and bug report for today
    _ = update_log::create_log(logs_path);
    _ = update_log::create_log(bugs_path);

    // Watch folders
    watch_folders::watch_downloads(root_path, &logs_path, &bugs_path);
    // watch_folders::watch_documents(root_path, &logs_path);
}
