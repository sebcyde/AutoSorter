pub mod transfer {

    use std::path::PathBuf;

    use crate::{
        functions::{
            editor::editor::{classify_file, fix_casing},
            update_log::update_log::{append_bug_report, append_log},
        },
        types::types::EventStruct,
    };

    pub async fn transfer_dir(event_object: EventStruct) {
        let destination: PathBuf = fix_casing(event_object.event_path);
        _ = append_log(&format!("{:?} has been transferred.", &destination));

        println!("From Transfer before Move {:?}", &destination);

        // let final_destination: PathBuf = move_dir(&destination);
        // println!("From Transfer after Move {:?}", &final_destination);
    }

    pub fn transfer_file(event_object: EventStruct) {
        fix_casing(event_object.event_path.clone());

        let destination: Option<PathBuf> = classify_file(event_object.event_path);

        if destination.is_some() {
            _ = append_log(&format!(
                "{:?} has been transferred.",
                event_object.event_target
            ));
        } else {
            _ = append_bug_report(&format!(
                "{:?} has NOT been transferred.",
                event_object.event_target
            ));
        }
    }
}
