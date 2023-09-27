use crate::functions::append_log::append_log::append_log;
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

    let _base_path: &Path = Path::new("C:/Users/SebCy/Documents/AutoSorter");
    let _logs_path: &Path = Path::new("C:/Users/SebCy/Documents/AutoSorter/Logs");

    println!("Base path: {:?}", _base_path);
    println!("Logs path: {:?}", _logs_path);
    println!(" ");

    // Create main AutoSorter directory if it doesnt exist
    if !Path::new(_base_path).exists() {
        println!("No AutoSorter directory found. Creating...");
        let _ = fs::create_dir_all(_base_path);
        println!("Created successfully");
        println!(" ");
    };

    // Create directory for logs and initial log file
    if !Path::new(&_logs_path).exists() {
        println!("No logs directory found. Creating...");
        let _ = fs::create_dir_all(_logs_path);
        println!("Created successfully");
        println!(" ");
    };

    let _ = update_log::update_log(_logs_path);

    // let _ = append_log("Example text to append", _base_path);
}
