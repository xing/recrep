use crate::json_parsing::crash_parsing;
use crate::model::crash_list::CrashList;
use crate::model::Report;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct TestHelper {}

impl TestHelper {
    pub fn report_from_json(path: &str) -> Report {
        let crash_list = TestHelper::crash_list_from_json(path);
        let report = Report::new("version".to_string(), crash_list);
        report
    }

    pub fn crash_list_from_json(path: &str) -> CrashList {
        let path = Path::new(path);
        let mut file = File::open(&path).expect("Unable to open test fixture");
        let mut crashes_json = String::new();
        file.read_to_string(&mut crashes_json)
            .expect("Failed to read fixture file into string");
        let crash_list = crash_parsing::crash_list_from_json(crashes_json.as_str()).unwrap();
        crash_list
    }
}
