pub mod helpers {
    use std::{
        fs::{read_dir, DirEntry, FileType, ReadDir},
        path::PathBuf,
    };

    pub fn get_dir_entries(dir_path: PathBuf) -> Vec<PathBuf> {
        let mut all_entries: Vec<PathBuf> = Vec::new();
        let entries: ReadDir = read_dir(dir_path).unwrap();
        for entry in entries {
            let dir_entry: DirEntry = entry.unwrap();
            let dir_type: FileType = dir_entry.file_type().unwrap();

            if dir_type.is_file() {
                all_entries.push(dir_entry.path());
            }
        }
        return all_entries;
    }
}
