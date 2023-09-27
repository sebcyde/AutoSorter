pub mod update_log {

    use chrono::{DateTime, Utc};
    use std::fs::{self, DirEntry, File, OpenOptions, ReadDir};
    use std::io::{self, BufWriter, Write};
    use std::path::{Path, PathBuf};

    pub fn update_log(log_path: &Path) -> io::Result<()> {
        let utc: DateTime<Utc> = Utc::now();
        let formatted_dt: String = utc.format("%Y%m%d%H%M%S").to_string();
        let file_name: String = format!("AutoSort_{}.txt", formatted_dt);
        let full_path: PathBuf = log_path.join(&file_name);

        // Creates log based on seconds so should technically always be new
        if !Path::new(&full_path).is_file() {
            println!("Creating log...");

            let f: File = File::create(&full_path)?;
            let mut writer: BufWriter<File> = BufWriter::new(f);

            writer.write_all(
                format!("{}\n", utc.format("%d-%m-%Y %H:%M:%S.").to_string()).as_bytes(),
            )?;

            writer.write_all(b"Rust is awesome.\n")?;
            writer.flush()?;

            println!("Log created successfully at: {:?}", full_path);
            println!(" ");

            Ok(())
        } else {
            // Will never invoke
            Ok(())
        }
    }

    pub fn append_log(content: &str, log_path: &Path) -> Result<(), io::Error> {
        println!("Updating log...");

        // Ensure the log directory exists, create it if it doesn't
        fs::create_dir_all(log_path)?;

        // Get the list of log files in the directory
        let mut all_logs: Vec<PathBuf> = Vec::new();
        let entries: ReadDir = fs::read_dir(log_path)?;
        for entry in entries {
            let entry: DirEntry = entry?;
            if entry.file_type()?.is_file() {
                all_logs.push(entry.path());
            }
        }

        // Get the latest log and open it in append mode
        let latest_log: &PathBuf = all_logs.iter().max().unwrap();

        // Use OpenOptions to open the file in append mode
        let mut options = OpenOptions::new();
        options.append(true); // Open in append mode
        let log: File = options.open(latest_log)?;

        let mut writer: BufWriter<File> = BufWriter::new(log);

        writer.write_all(content.as_bytes())?;
        writer.flush()?;

        println!("Log updated successfully.");
        println!(" ");

        Ok(())
    }
}
