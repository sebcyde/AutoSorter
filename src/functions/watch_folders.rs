pub mod watch_folders {

    extern crate notify;

    use notify::{
        Config, EventKind, ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher,
    };
    use std::{
        fs::{remove_dir, remove_dir_all},
        path::{Path, PathBuf},
    };

    use crate::functions::{
        editor::editor::{clean_folder, fix_casing, move_dir, zip_directory},
        get_dirs::get_dirs::get_root,
        transfer::transfer::transfer_file,
        update_log::update_log::{append_bug_report, append_log},
    };

    use crate::types::types::EventStruct;

    fn remove_event(event_object: EventStruct) {
        println!("Remove event triggered");
        debug_sort_fn(&event_object);
    }

    fn access_event(event_object: EventStruct) {
        println!("Access event triggered");
        debug_sort_fn(&event_object);
    }

    fn other_event(event_object: EventStruct) {
        println!("Other event triggered");
        debug_sort_fn(&event_object);
    }

    fn any_event(event_object: EventStruct) {
        println!("Any event triggered");
        debug_sort_fn(&event_object);
    }

    fn modify_event(event_object: EventStruct) {
        println!(" ");
        println!("Modify event triggered");
        debug_sort_fn(&event_object);

        if event_object.event_path.is_dir() {
            // Rename Directory
            let path: PathBuf = fix_casing(event_object.event_path.clone());

            // Copy Directory to new destination
            let destination_res: Option<PathBuf> = move_dir(&path);
            if destination_res.is_none() {
                return;
            }

            let destination: PathBuf = destination_res.unwrap();

            // Delete old version
            if remove_dir(&path).is_err() {
                let bug_report_content: String =
                    format!("Error removing transferred directory at: {:?}", &path);
                _ = append_bug_report(&bug_report_content);
            } else {
                // Zip Directory
                let zip_res: Result<(), std::io::Error> = zip_directory(&destination);
                if zip_res.is_ok() {
                    _ = remove_dir_all(&destination)
                } else {
                    let content: String =
                        format!("Error zipping transferred directory at: {:?}", &path);
                    _ = append_bug_report(&content);
                }
            }
        } else if event_object.event_path.is_file() {
            transfer_file(event_object);
        } else {
            println!("{:?} - Not directory or file.", event_object.event_target);

            _ = append_log(&format!(
                "BUG: {} {:?} at: {}",
                event_object.event_kind_str, event_object.event_target, event_object.event_path_str
            ));
        }
    }

    fn debug_sort_fn(event_object: &EventStruct) {
        println!("Event Path: {:?}", event_object.event_path);
        println!("Event Kind: {:?}", event_object.event_kind_str);
        println!("Event Target: {:?}", event_object.event_target);
        if event_object.event_path.is_dir() {
            println!("Event is Dir");
        } else {
            println!("Event is File");
        }
    }

    fn watch_folder<P: AsRef<Path>>(path: P) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        // Create new watcher
        let mut watcher: ReadDirectoryChangesWatcher =
            RecommendedWatcher::new(tx, Config::default())?;

        // Path to be watched - Non-Recursive Mode
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        for res in rx {
            match res {
                Ok(event) => {
                    let event_path: PathBuf = event.paths.get(0).unwrap().to_owned();
                    println!("Update Type: {:?}", event.kind);

                    // Functions for each event type
                    match event.kind {
                        EventKind::Remove(_) => {
                            let event_path_clone: &'static PathBuf =
                                Box::leak(Box::new(event_path.clone()));

                            let update_event: EventStruct = EventStruct {
                                event_path,
                                event_target: Path::file_name(&event_path_clone).unwrap(),
                                event_path_str: event_path_clone.to_str().unwrap(),
                                event_kind_str: "Remove",
                                modify_kind: None,
                            };
                            remove_event(update_event);
                        }
                        EventKind::Access(_) => {
                            let event_path_clone: &'static PathBuf =
                                Box::leak(Box::new(event_path.clone()));

                            let update_event: EventStruct = EventStruct {
                                event_path,
                                event_target: Path::file_name(&event_path_clone).unwrap(),
                                event_path_str: event_path_clone.to_str().unwrap(),
                                event_kind_str: "Access",
                                modify_kind: None,
                            };
                            access_event(update_event);
                        }
                        EventKind::Any => {
                            let event_path_clone: &'static PathBuf =
                                Box::leak(Box::new(event_path.clone()));

                            let update_event: EventStruct = EventStruct {
                                event_path,
                                event_target: Path::file_name(&event_path_clone).unwrap(),
                                event_path_str: event_path_clone.to_str().unwrap(),
                                event_kind_str: "An",
                                modify_kind: None,
                            };
                            any_event(update_event);
                        }
                        EventKind::Modify(_) | EventKind::Create(_) => {
                            let event_path_clone: &'static PathBuf =
                                Box::leak(Box::new(event_path.clone()));

                            let update_event: EventStruct = EventStruct {
                                event_path,
                                event_target: Path::file_name(&event_path_clone).unwrap(),
                                event_path_str: event_path_clone.to_str().unwrap(),
                                event_kind_str: "Modify",
                                modify_kind: Some("TODO"),
                            };

                            modify_event(update_event);
                        }
                        EventKind::Other => {
                            let event_path_clone: &'static PathBuf =
                                Box::leak(Box::new(event_path.clone()));

                            let update_event: EventStruct = EventStruct {
                                event_path,
                                event_target: Path::file_name(&event_path_clone).unwrap(),
                                event_path_str: event_path_clone.to_str().unwrap(),
                                event_kind_str: "Other",
                                modify_kind: None,
                            };
                            other_event(update_event);
                        }
                    };

                    println!(" ");
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        Ok(())
    }

    pub fn start_watch(dir_name: &'static str) {
        let dir_path: PathBuf = Path::new(get_root().as_str()).join(dir_name);
        println!("Starting {} watcher", dir_name);

        _ = append_log(&format!("Cleaning {}.", dir_name));
        _ = clean_folder(&dir_path);

        _ = append_log(&format!("Starting {} watcher.", dir_name));
        _ = watch_folder(&dir_path);
    }
}
