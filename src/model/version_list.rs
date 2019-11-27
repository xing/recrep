use crate::model::Version;
pub struct VersionList {}

impl VersionList {
    pub fn latest_version(versions: Vec<Version>) -> Option<Version> {
        let mut sorted_versions = versions.to_vec();
        sorted_versions.sort_by(|a, b| b.uploaded_at.cmp(&a.uploaded_at));
        sorted_versions.first().map(|v| v.clone())
    }
}
