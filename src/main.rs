// use crate::functions::append_log::append_log::append_log;
use crate::functions::update_log::update_log;
use std::fs;
use std::path::{Path, PathBuf};

pub mod functions {
    pub mod append_log;
    pub mod archive;
    pub mod clean_archive;
    pub mod create_directory;
    pub mod update_log;
}

fn main() {
    println!("Starting AutoSorter...");

    let base_path: &Path;

    let is_at_work: bool = Path::new("C:/Users/sebastian.cyde").exists();

    if is_at_work {
        base_path = Path::new("C:/Users/sebastian.cyde/Documents/AutoSorter");
    } else {
        base_path = Path::new("C:/Users/SebCy/Documents/AutoSorter");
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

    // Create directory for logs and initial log file
    if !Path::new(&logs_path).exists() {
        println!("No logs directory found. Creating...");
        let _ = fs::create_dir_all(logs_path);
        println!("Created successfully");
        println!(" ");
    };

    let _ = update_log::update_log(logs_path);
    let _ = update_log::append_log("Example text to append", logs_path);
}
