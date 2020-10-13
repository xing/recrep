use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorGroup {
    //pub id: String,
    #[serde(rename = "operatingSystems")]
    pub operating_systems: Vec<OperatingSystemCount>,
    #[serde(rename = "errorCount")]
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingSystemCount {
    #[serde(rename = "operatingSystemName")]
    pub name: String,
    #[serde(rename = "errorCount")]
    pub count: u64,
}

//impl ErrorGroup {
//    pub fn new(id: String) -> ErrorGroup {
//        ErrorGroup {
//            id: id,
//            operating_systems: HashMap::new(),
//            count: 0,
//        }
//    }
//}
