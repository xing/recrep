use crate::model::CrashList;
use serde_json;
pub struct CrashParser {}

impl CrashParser {
    ///
    /// Parses a CrashList from the given json
    ///
    /// ```
    /// # use std::fs::File;
    /// # use std::io::prelude::*;
    /// # use std::path::Path;
    /// use recrep::json_parsing::CrashParser;
    /// let path = Path::new("src/json_parsing/test_fixtures/crashes.json");
    /// let mut file = File::open(&path).expect("Unable to open test fixture");
    /// let mut crashes_json = String::new();
    /// file.read_to_string(&mut crashes_json)
    ///     .expect("Failed to read fixture file into string");
    /// let crash_list = CrashParser::crash_list_from_json(&crashes_json)
    ///     .expect("Failed to parse crash test fixture");
    /// assert_eq!(crash_list.crashes.len(), 30);
    /// ```
    pub fn crash_list_from_json(json: &str) -> Result<CrashList, &'static str> {
        let crash_list: serde_json::Result<CrashList> = serde_json::from_str(json);
        match crash_list {
            Ok(crash_list) => Ok(crash_list),
            Err(e) => {
                println!("Parsing error: {:}", e);
                Err("💥 Failed to parse json into crash list. This happens when there is no app for the given organization, name and version.")
            }
        }
    }
}
