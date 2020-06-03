use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub short_version: String,

    pub uploaded_at: String,

    pub distribution_groups: Option<Vec<DistributionGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionGroup {
    pub id: String,

    pub name: String,
}
