pub mod archive {

    use std::fs;

    use crate::functions::update_log::update_log::append_log;

    pub fn archive(source_path: &str) -> Result<(), std::io::Error> {
        println!("Archiving!");

        // You can specify the destination path where you want to move the file.
        let destination_path: &str = "/path/to/destination/"; // Replace with your destination directory

        // Create the destination directory if it doesn't exist
        fs::create_dir_all(destination_path)?;

        fs::create_dir_all(destination_path)?;

        // Extract the file name from the source path
        let file_name = match std::path::Path::new(source_path).file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Invalid file name",
                ))
            }
        };

        // Construct the destination path including the file name
        let destination_file_path = format!("{}/{}", destination_path, file_name);

        // Rename (move) the source file to the destination path
        fs::rename(source_path, &destination_file_path)?;

        println!("Archive complete");
        Ok(())
    }

    pub fn clean_archive() {
        _ = append_log("Archive Clean Started.");

        // Basically delete everything over a certain age

        _ = append_log("Archive Clean Complete.");
    }
}
