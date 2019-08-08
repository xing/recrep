use super::api_trait::API;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// The mock API is only used for tests. It implements the shared API trait so the mock can be used
/// interchangeably with a concrete API like the AppCenter API.
pub struct MockAPI {
    crashes: Option<String>,
}

impl MockAPI {
    pub fn with_crashes_json(json: String) -> impl API {
        MockAPI {
            crashes: Some(json),
        }
    }

    pub fn with_two_crashes() -> impl API {
        let path = Path::new("src/json_parsing/test_fixtures/two_crashes.json");
        let mut file = File::open(&path).expect("Unable to open test fixture");
        let mut crashes_json = String::new();
        file.read_to_string(&mut crashes_json)
            .expect("Failed to read fixture file into string");

        MockAPI::with_crashes_json(crashes_json)
    }
}

impl API for MockAPI {
    fn new(_token: String) -> Self {
        MockAPI { crashes: None }
    }

    fn latest_version(
        &self,
        _organization: String,
        _application: String,
    ) -> Result<String, &'static str> {
        Ok("1.2.3".to_string())
    }

    fn crashes_json(
        &self,
        _organization: String,
        _application: String,
        _version: String,
    ) -> Result<String, &'static str> {
        match self.crashes.clone() {
            Some(json) => Ok(json),
            _ => Err("Mock has no crashes"),
        }
    }
}
