pub mod watch_folders {

    extern crate notify;

    use notify::{
        event::{ModifyKind, RenameMode},
        Config, EventKind, ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher,
    };
    use std::{
        ffi::OsStr,
        path::{Path, PathBuf},
    };

    use crate::functions::{
        editor::editor::{fix_casing, move_file},
        transfer::transfer::{transfer_dir, transfer_file},
        update_log::update_log::append_log,
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

    fn modify_event(event_object: EventStruct, logs_path: &Path, bugs_path: &Path) {
        println!("Modify event triggered");
        debug_sort_fn(&event_object);

        if event_object.event_path.is_dir() {
            println!("Transferring: {:?}.", event_object.event_target);
            transfer_dir(event_object);
            // Move entire directory, fix casing and zip it
        } else if event_object.event_path.is_file() {
        } else {
            println!("{:?} - Not directory or file.", event_object.event_target);

            let _ = append_log(
                &format!(
                    "BUG: {} {:?} at: {}",
                    event_object.event_kind_str,
                    event_object.event_target,
                    event_object.event_path_str
                ),
                bugs_path,
            );
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

    fn watch_folder<P: AsRef<Path>>(
        path: P,
        logs_path: &Path,
        bugs_path: &Path,
    ) -> notify::Result<()> {
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

                            println!("event_kind_str: {}", update_event.event_path_str);
                            println!("event_target: {:?}", update_event.event_target);

                            modify_event(update_event, logs_path, bugs_path);
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

    pub fn watch_downloads(root_path: &Path, logs_path: &Path, bugs_path: &Path) {
        let downloads_path: PathBuf = Path::new(root_path).join("Downloads");
        println!("Starting downloads watcher");
        let _ = append_log("Started downloads watcher.", logs_path);
        let _ = watch_folder(&downloads_path, logs_path, bugs_path);
    }

    pub fn watch_documents(root_path: &Path, logs_path: &Path, bugs_path: &Path) {
        let documents_path: PathBuf = Path::new(root_path).join("Documents");
        println!("Starting documents watcher");
        let _ = append_log("Started documents watcher.", logs_path);
        let _ = watch_folder(&documents_path, logs_path, bugs_path);
    }
}
