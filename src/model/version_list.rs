use crate::model::Version;
pub struct VersionList {}

impl VersionList {
    pub fn latest_version(versions: Vec<Version>) -> Option<Version> {
        let mut sorted_versions = versions.to_vec();
        sorted_versions.sort_by(|a, b| b.uploaded_at.cmp(&a.uploaded_at));
        sorted_versions.first().map(|v| v.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_sorting_of_version_list() {
        let version1 = Version { short_version: String::from("1.0"), uploaded_at: String::from("2019-11-16T22:29:48.000Z") };
        let version2 = Version { short_version: String::from("1.0"), uploaded_at: String::from("2019-11-17T22:29:48.000Z") };
        let version3 = Version { short_version: String::from("1.0"), uploaded_at: String::from("2019-11-18T22:29:48.000Z") };
        let mut vec = Vec::new();

        vec.push(version1);
        vec.push(version2);
        vec.push(version3);

        let latest = VersionList::latest_version(vec);
        match latest {
            Some(latest) => assert_eq!(latest.uploaded_at, String::from("2019-11-18T22:29:48.000Z")),
            None => panic!("There was no latest version in the returned sorted list")
        }
    }
}
