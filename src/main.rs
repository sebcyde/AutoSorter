// use crate::functions::append_log::append_log::append_log;
use crate::functions::update_log::update_log;
use std::fs;
use std::path::Path;

pub mod functions {
    pub mod append_log;
    pub mod archive;
    pub mod clean_archive;
    pub mod create_directory;
    pub mod update_log;
}

fn main() {
    println!("Starting AutoSorter...");

    let is_at_work: bool = Path::new("C:/Users/sebastian.cyde").exists();

    let _base_path: &Path;
    let _logs_path: &Path;

    if is_at_work {
        _logs_path = Path::new("C:/Users/sebastian.cyde/Documents/AutoSorter/Logs");
        _base_path = Path::new("C:/Users/sebastian.cyde/Documents/AutoSorter");
    } else {
        _logs_path = Path::new("C:/Users/SebCy/Documents/AutoSorter/Logs");
        _base_path = Path::new("C:/Users/SebCy/Documents/AutoSorter");
    }

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
    let _ = update_log::append_log("Example text to append", _logs_path);
}
