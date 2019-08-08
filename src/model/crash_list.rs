use crate::model::Crash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrashList {
    #[serde(rename = "errorGroups")]
    pub crashes: Vec<Crash>,
}

// pub fn from_json() -> impl API {
//         let path = Path::new("src/json_parsing/test_fixtures/two_crashes.json");
//         let mut file = File::open(&path).expect("Unable to open test fixture");
//         let mut crashes_json = String::new();
//         file.read_to_string(&mut crashes_json)
//             .expect("Failed to read fixture file into string");

//         MockAPI::with_crashes_json(crashes_json)
//     }
