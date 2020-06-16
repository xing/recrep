use crate::model::Version;
pub struct VersionList {}

impl VersionList {
    pub fn latest_version(versions: Vec<Version>) -> Option<Version> {
        VersionList::sort_versions(versions).first().cloned()
    }

    pub fn latest_version_of_distribution_group(
        versions: Vec<Version>,
        distribution_group: String,
    ) -> Option<Version> {
        let sorted_versions = VersionList::sort_versions(versions);

        for version in sorted_versions {
            match &version.distribution_groups {
                Some(groups) => {
                    if groups
                        .iter()
                        .find(|group| group.name == distribution_group)
                        .is_some()
                    {
                        return Some(version.clone());
                    }
                }
                None => return None,
            }
        }

        None
    }

    fn sort_versions(mut versions: Vec<Version>) -> Vec<Version> {
        versions.sort_by(|a, b| b.uploaded_at.cmp(&a.uploaded_at));
        versions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_sorting_of_version_list() {
        let version1 = Version {
            short_version: String::from("1.0"),
            uploaded_at: String::from("2019-11-16T22:29:48.000Z"),
            distribution_groups: None,
        };
        let version2 = Version {
            short_version: String::from("1.0"),
            uploaded_at: String::from("2019-11-17T22:29:48.000Z"),
            distribution_groups: None,
        };
        let version3 = Version {
            short_version: String::from("1.0"),
            uploaded_at: String::from("2019-11-18T22:29:48.000Z"),
            distribution_groups: None,
        };
        let vec = vec![version1, version2, version3];

        let latest = VersionList::latest_version(vec);
        match latest {
            Some(latest) => {
                assert_eq!(latest.uploaded_at, String::from("2019-11-18T22:29:48.000Z"))
            }
            None => panic!("There was no latest version in the returned sorted list"),
        }
    }
}
