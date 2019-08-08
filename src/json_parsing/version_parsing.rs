use crate::model::Version;
use serde_json;

pub struct VersionListParser {}

impl VersionListParser {
    pub fn versions(json: &str) -> Result<Vec<Version>, &'static str> {
        let result: serde_json::Result<Vec<Version>> = serde_json::from_str(json);
        match result {
            Ok(version) => Ok(version),
            Err(e) => {
                println!("Version parsing error: {:}", e);
                Err("💥 Failed to parse json into a version list")
            }
        }
    }
}
