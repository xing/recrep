use crate::model::version::{Version, DistributionGroup};
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

        let group_version = sorted_versions
            .iter()
            .filter(|version| VersionList::by_distribution_group(&distribution_group,
                                                                 version.distribution_groups.as_ref()))
            .next();

        return group_version.cloned();
    }

    fn by_distribution_group(distribution_group: &String, distribution_groups: Option<&Vec<DistributionGroup>>) -> bool {
        match distribution_groups {
            Some(groups) => groups.iter().any(|group| group.name == distribution_group.to_string()),
            None => false,
        }
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
    
    #[test]
    fn correct_version_when_filtering_by_distribution_group() {

        let distribution_group_name = "Test distribution group";

        let irrelevant_group = DistributionGroup {
            id: String::from("irrelevant_ID"),
            name: String::from("irrelevant_string"),
        };

        let relevant_group = DistributionGroup {
            id: String::from("irrelevant_ID"),
            name: String::from(distribution_group_name),
        };

        let expected_version_string = "1.1";

        let version1 = Version {
            short_version: String::from("1.0"),
            uploaded_at: String::from("2019-11-16T22:29:48.000Z"),
            distribution_groups: Some(vec![irrelevant_group.clone()]),
        };

        let version2 = Version {
            short_version: String::from(expected_version_string),
            uploaded_at: String::from("2019-11-17T22:29:48.000Z"),
            distribution_groups: Some(vec![relevant_group]),
        };
        let version3 = Version {
            short_version: String::from("1.2"),
            uploaded_at: String::from("2019-11-18T22:29:48.000Z"),
            distribution_groups: Some(vec![irrelevant_group.clone()]),
        };

        let vec = vec![version1, version2, version3];

        let found_version = VersionList::latest_version_of_distribution_group(vec, distribution_group_name.to_string());
        match found_version {
            Some(found_version) => {
                assert_eq!(found_version.short_version, String::from(expected_version_string))
            }
            None => panic!("There was no latest version in the returned sorted list"),
        }
    }

    #[test]
    fn dont_find_version_when_filtering_by_distribution_group() {

        let distribution_group_name = "Test distribution group";
        let other_distribution_group_name = "Another distribution group";

        let group = DistributionGroup {
            id: String::from("irrelevant_ID"),
            name: String::from(other_distribution_group_name),
        };

        let version1 = Version {
            short_version: String::from("1.0"),
            uploaded_at: String::from("2019-11-16T22:29:48.000Z"),
            distribution_groups: Some(vec![group.clone()]),
        };

        let version2 = Version {
            short_version: String::from("1.1"),
            uploaded_at: String::from("2019-11-17T22:29:48.000Z"),
            distribution_groups: Some(vec![group.clone()]),
        };
        let version3 = Version {
            short_version: String::from("1.2"),
            uploaded_at: String::from("2019-11-18T22:29:48.000Z"),
            distribution_groups: Some(vec![group.clone()]),
        };

        let vec = vec![version1, version2, version3];

        let found_version = VersionList::latest_version_of_distribution_group(vec, distribution_group_name.to_string());
        match found_version {
            Some(_found_version) => {
                panic!("Should not find any version");
            }
            None => ()
        }
    }
}
