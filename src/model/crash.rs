use crate::model::OperatingSystemCount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Crash {
    #[serde(rename = "exceptionMethod")]
    pub exception_method: Option<String>,

    #[serde(rename = "appVersion")]
    pub app_version: Option<String>,

    #[serde(rename = "appBuild")]
    pub app_build: Option<String>,

    #[serde(rename = "firstOccurrence")]
    pub first_occurrence: Option<String>,

    #[serde(rename = "exceptionFile")]
    pub exception_file: Option<String>,

    #[serde(rename = "exceptionClassName")]
    pub exception_classname: Option<String>,

    #[serde(rename = "errorGroupId")]
    pub error_group_id: Option<String>,

    #[serde(rename = "deviceCount")]
    pub device_count: Option<u64>,

    #[serde(rename = "exceptionAppCode")]
    pub exception_app_code: Option<bool>,

    pub count: Option<u64>,

    pub operating_systems: Option<Vec<OperatingSystemCount>>,
}
