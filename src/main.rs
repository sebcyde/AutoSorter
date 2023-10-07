mod types;

use crate::functions::create_directory::create_dir::create_containers;
use crate::functions::get_dirs::get_dirs::{get_base, get_bugs, get_logs, get_root};
use crate::functions::update_log::update_log;
use crate::functions::watch_folders::watch_folders;
use std::fs;
use std::path::{Path, PathBuf};

pub mod functions {
    pub mod archive;
    pub mod clean_archive;
    pub mod create_directory;
    pub mod editor;
    pub mod get_dirs;
    pub mod transfer;
    pub mod update_log;
    pub mod watch_folders;
}

fn main() {
    println!(" ");
    println!("Starting AutoSorter...");
    println!(" ");

    let base_path: &Path = Path::new(get_base().as_str());
    let root_path: &Path = Path::new(get_root().as_str());
    let logs_path: &Path = Path::new(get_logs().as_str());
    let bugs_path: &Path = Path::new(get_bugs().as_str());

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
