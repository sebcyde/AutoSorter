pub mod types {
    use std::{ffi::OsStr, path::PathBuf};

    pub struct EventStruct {
        pub event_path: PathBuf,
        pub event_target: &'static OsStr,
        pub event_path_str: &'static str,
        pub event_kind_str: &'static str,
        pub modify_kind: Option<&'static str>,
    }
}
