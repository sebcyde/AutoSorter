pub mod update_log {

    use chrono::{DateTime, Utc};
    use std::fs::{self, DirEntry, File, OpenOptions, ReadDir};
    use std::io::{self, BufWriter, Write};
    use std::path::{Path, PathBuf};

    use crate::functions::get_dirs::get_dirs::{get_bugs, get_logs};

    pub fn get_current_time(format: &'static str) -> String {
        let utc: DateTime<Utc> = Utc::now();
        let formatted_dt: String = utc.format(format).to_string();
        return formatted_dt;
    }

    pub fn create_log() -> io::Result<()> {
        let log_path: &Path = Path::new(get_logs().as_str());
        let bugs_path: &Path = Path::new(get_bugs().as_str());

        let formatted_dt: String = get_current_time("%Y%m%d");
        let file_name: String = format!("AutoSort_{}.txt", formatted_dt);
        let full_path: PathBuf = log_path.join(&file_name);

        if Path::new(&full_path).is_file() {
            _ = append_log("Ended Session\n");
            _ = append_log("Started new session");
            return Ok(());
        }

        println!("Creating log...");
        _ = File::create(&full_path);
        append_log(&get_current_time("%d-%m-%Y %H:%M:%S"));
        append_log(" - Log Created.");

        println!("Log created successfully at: {:?}", full_path);
        println!(" ");

        return Ok(());
    }

    pub fn append_log(content: &str) -> Result<(), io::Error> {
        let log_path: &Path = Path::new(get_logs().as_str());
        let bugs_path: &Path = Path::new(get_bugs().as_str());

        // Get the list of log files in the directory
        let mut all_logs: Vec<PathBuf> = Vec::new();
        let entries: ReadDir = fs::read_dir(log_path)?;
        for entry in entries {
            if entry?.file_type()?.is_file() {
                all_logs.push(entry?.path());
            }
        }

        // Get the latest log and open it in append mode
        let latest_log: &PathBuf = all_logs.iter().max().unwrap();
        let current_time: String = get_current_time("%H:%M:%S");
        println!("Updating log at: {}", &current_time);

        let mut options = OpenOptions::new();
        options.append(true);
        let log: File = options.open(latest_log)?;

        let mut writer: BufWriter<File> = BufWriter::new(log);

        let current_time: String = get_current_time("%H:%M:%S");
        println!("Updating log at: {}", &current_time);

        let log_content_bytes: Vec<u8> = format!("{} - {}\n", current_time, content).into_bytes();
        let log_content: &[u8] = &log_content_bytes;

        writer.write_all(log_content)?;
        writer.flush()?;

        return Ok(());
    }
}
