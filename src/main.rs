mod types;

use crate::functions::editor::editor::clean_folder;
use crate::functions::get_dirs::get_dirs::{get_base, get_bugs, get_logs, get_root};
use crate::functions::update_log::update_log::{append_log, create_bug_report, create_log};
use crate::functions::watch_folders::watch_folders::start_watch;
use std::fs;
use std::path::{Path, PathBuf};

pub mod functions {
    pub mod archive;
    pub mod clean_archive;
    pub mod create_directory;
    pub mod editor;
    pub mod get_dirs;
    pub mod helpers;
    pub mod transfer;
    pub mod update_log;
    pub mod watch_folders;
}

fn main() {
    println!(" ");
    println!("Starting AutoSorter...");
    println!(" ");

    let base_binding: String = get_base();
    let logs_binding: String = get_logs();
    let bugs_binding: String = get_bugs();

    let base_path: &Path = Path::new(&base_binding);
    let logs_path: &Path = Path::new(&logs_binding);
    let bugs_path: &Path = Path::new(&bugs_binding);

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
        println!("No bug report directory found. Creating...");
        _ = fs::create_dir_all(bugs_path);
        println!("Created successfully");
        println!(" ");
    };

    // Create an activity log and bug report for today
    _ = create_log();
    _ = create_bug_report();

    _ = append_log("Root folders created successfully. \n");

    // Watch folders - parameter is path from root eg. "Documents" for {root}/Documents
    start_watch("Downloads");
}
