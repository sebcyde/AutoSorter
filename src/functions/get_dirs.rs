pub mod get_dirs {
    use std::path::Path;

    fn is_at_work() -> bool {
        return Path::new("C:/Users/sebastian.cyde").exists();
    }

    pub fn get_base() -> String {
        if is_at_work() {
            return String::from("C:/Users/sebastian.cyde/Documents/AutoSorter");
        } else {
            return String::from("C:/Users/SebCy/Documents/AutoSorter");
        }
    }

    pub fn get_root() -> String {
        if is_at_work() {
            return String::from("C:/Users/sebastian.cyde");
        } else {
            return String::from("C:/Users/SebCy");
        }
    }

    pub fn get_logs() -> String {
        if is_at_work() {
            return String::from("C:/Users/sebastian.cyde/Documents/AutoSorter/Logs");
        } else {
            return String::from("C:/Users/SebCy/Documents/AutoSorter/Logs");
        }
    }

    pub fn get_bugs() -> String {
        if is_at_work() {
            return String::from("C:/Users/sebastian.cyde/Documents/AutoSorter/Bugs");
        } else {
            return String::from("C:/Users/SebCy/Documents/AutoSorter/Bugs");
        }
    }
}
