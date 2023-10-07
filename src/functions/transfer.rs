pub mod transfer {

    use crate::types::types::EventStruct;

    pub fn transfer_dir(event_object: EventStruct) {}

    pub fn transfer_file(event_object: EventStruct) {
        // fix_casing(event_object.event_path.clone(), logs_path);
        // let destination: Option<PathBuf> = move_file(event_object.event_path, logs_path);

        // if destination.is_some() {
        // println!("{:?} has been transferred.", event_object.event_target);
        // } else {
        // println!("{:?} has NOT been transferred.", event_object.event_target);
        // }
    }
}
