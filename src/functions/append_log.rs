pub mod append_log {

    use std::fs::OpenOptions;
    use std::io::{Result, Write};

    pub fn append_log(content: &str, _base_path: &str) -> Result<()> {
        // Open the log file in append mode
        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(_base_path)?;

        // Append the content to the log file
        log_file.write_all(content.as_bytes())?;

        Ok(())
    }
}
