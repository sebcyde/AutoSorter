pub mod create_dir {
    use std;
    use std::path::{Path, PathBuf};

    use tokio::fs;

    pub fn create_containers(base_path: &Path) {
        let extensions: Vec<&str> = vec![
            "EXE", "JPG", "VIDEO", "PDF", "CSV", "JS", "GZ", "DB", "PHP", "PPTX", "ZIP", "EXCEL",
            "AI", "TXT", "OTHER",
        ];

        for ext in extensions {
            let ext_pathbuf: PathBuf = Path::new(ext).to_path_buf();
            let binding: PathBuf = Path::new(base_path).join("Files").join(&ext_pathbuf);
            let ext_path: &Path = binding.as_path();
            if !Path::new(ext_path).exists() {
                println!("Creating: {:?}", ext_path);
                std::fs::create_dir_all(ext_path).expect("Failed to create directory");
                println!(" ");
            }
        }
    }

    pub async fn create_dir(new_dir_path: &str) -> String {
        match fs::metadata(new_dir_path).await {
            Ok(_) => {
                println!("Clean directory already exists.");
                new_dir_path.to_string()
            }
            Err(_) => {
                if let Err(err) = fs::create_dir(new_dir_path).await {
                    eprintln!("Error creating clean container: {}", err);
                    String::new()
                } else {
                    println!("Created directory.");
                    println!("New directory: {}", new_dir_path);
                    new_dir_path.to_string()
                }
            }
        }
    }
}
