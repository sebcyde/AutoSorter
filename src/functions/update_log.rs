pub mod update_log {

    use chrono::Utc;
    use std::fs::File;
    use std::io::{self, BufWriter, Write};
    use std::path::Path;

    pub fn update_log(log_path: &Path) -> io::Result<()> {
        let utc = Utc::now();
        let formatted_dt = utc.format("%Y%m%d%H%M%S").to_string();
        let file_name = format!("AutoSort_{}.txt", formatted_dt);
        let full_path = log_path.join(&file_name);

        // Creates log based on seconds so should technically always be new
        if !Path::new(&full_path).is_file() {
            println!("Creating log...");

            let f: File = File::create(&full_path)?;
            let mut writer: BufWriter<File> = BufWriter::new(f);

            writer.write_all(b"Hello, world!\n")?;
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
}
