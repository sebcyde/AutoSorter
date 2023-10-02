pub mod watch_folders {

    extern crate notify;

    use notify::{
        Config, EventKind, ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher,
    };
    use std::{
        ffi::OsStr,
        path::{Path, PathBuf},
    };

    use crate::functions::{editor::editor::fix_casing, update_log::update_log::append_log};

    pub fn watch_folder<P: AsRef<Path>>(path: P, logs_path: &Path) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        // Create new watcher
        let mut watcher: ReadDirectoryChangesWatcher =
            RecommendedWatcher::new(tx, Config::default())?;

        // Path to be watched - Recursive
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

        for res in rx {
            match res {
                Ok(event) => {
                    println!("Update Type: {:?}", event.kind);

                    // TODO - From and to events need differentiating

                    // Convert EventKind type to &str
                    let event_kind_str: &str = match event.kind {
                        EventKind::Access(_) => "Access",
                        EventKind::Create(_) => "Create",
                        EventKind::Remove(_) => "Remove",
                        EventKind::Any => "Any",
                        EventKind::Modify(_) => "Modify",
                        EventKind::Other => "Other",
                    };

                    // Convert event path and event target to &str
                    let event_path: PathBuf = event.paths.get(0).unwrap().to_owned();
                    let event_target: &OsStr = Path::file_name(&event_path).unwrap();
                    let event_path_str: &str = event_path.to_str().unwrap();

                    if event_path.is_dir() {
                        println!("It's a directory.");
                    } else if event_path.is_file() {
                        println!("It's a file.");
                        fix_casing(event_path, logs_path);
                    } else {
                        println!("It's neither a directory nor a file.");
                        let _ = append_log(
                            &format!(
                                "Event: {} {:?} at: {}",
                                event_kind_str, event_target, event_path_str
                            ),
                            logs_path,
                        );
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        Ok(())
    }

    pub fn watch_downloads(root_path: &Path, logs_path: &Path) {
        let downloads_path: PathBuf = Path::new(root_path).join("Downloads");
        println!("Starting downloads watcher");
        let _ = append_log("Started downloads watcher.", logs_path);
        let _ = watch_folder(&downloads_path, logs_path);
    }

    pub fn watch_documents(root_path: &Path, logs_path: &Path) {
        let documents_path: PathBuf = Path::new(root_path).join("Documents");
        println!("Starting documents watcher");
        let _ = append_log("Started documents watcher.", logs_path);
        let _ = watch_folder(&documents_path, logs_path);
    }
}
