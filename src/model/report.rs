use crate::model::CrashList;

pub struct Report {
    pub version: String,
    pub crash_list: CrashList,
}

impl Report {
    pub fn new(version: String, crash_list: CrashList) -> Report {
        Report {
            version,
            crash_list,
        }
    }
}
