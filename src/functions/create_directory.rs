pub mod create_dir {
    use tokio::fs;

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
