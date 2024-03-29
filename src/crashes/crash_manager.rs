use crate::api::API;
use crate::json_parsing::crash_parsing;
use crate::json_parsing::version_parsing::VersionListParser;
use crate::model::{ErrorGroup, Report, VersionList};

/// The `CrashManager` is responsible to get crash data from its API.
/// It transforms crash data into structs using a the `CrashParser`.
pub struct CrashManager {}

impl CrashManager {
    /// Fetches crash groups from the API. If no version is provided, the latest version of the
    /// given app will be used
    ///
    /// ```
    /// # use recrep::api::MockAPI;
    /// # use recrep::crashes::CrashManager;
    /// # let api = MockAPI::with_two_crashes();
    /// #
    /// // api is a mock that returns 2 crashes
    /// let manager = CrashManager{};
    /// let report = manager.crash_list(&api, "org", "app", Some("1.2.3".to_string()), None).unwrap();
    ///
    /// assert_eq!(report.crash_list.crashes.len(), 2);
    /// assert_eq!(report.version, "1.2.3");
    /// ```
    pub fn crash_list(
        &self,
        api: &impl API,
        organization: &str,
        application: &str,
        version: Option<String>,
        distribution_group: Option<String>,
    ) -> Result<Report, &'static str> {
        match version {
            Some(version) => self.crash_list_for_version(
                api,
                organization.to_string(),
                application.to_string(),
                version,
            ),
            None => self.crash_list_for_latest_version(
                api,
                organization.to_string(),
                application.to_string(),
                distribution_group,
            ),
        }
    }

    /// Returns a Report after loading and parsing crashes json from the API
    fn crash_list_for_version(
        &self,
        api: &impl API,
        organization: String,
        application: String,
        version: String,
    ) -> Result<Report, &'static str> {
        match api.crashes_json(organization, application, version.clone()) {
            Ok(json) => Ok(Report::new(
                version,
                crash_parsing::crash_list_from_json(json.as_str()).unwrap(),
            )),
            Err(_e) => Err("Failed to get crashes json from API."),
        }
    }

    fn crash_list_for_latest_version(
        &self,
        api: &impl API,
        organization: String,
        application: String,
        distribution_group: Option<String>,
    ) -> Result<Report, &'static str> {
        let latest_version_json = api
            .latest_version(organization.to_string(), application.to_string())
            .expect("Missing version json.");
        let versions = VersionListParser::versions(&latest_version_json).unwrap();
        let latest_version = match distribution_group {
            Some(group) => VersionList::latest_version_of_distribution_group(versions, group),
            None => VersionList::latest_version(versions),
        };

        match latest_version {
            Some(latest_version) => self.crash_list_for_version(
                api,
                organization.to_string(),
                application.to_string(),
                latest_version.short_version.clone(),
            ),
            None => {
                Err("💥 Failed to get the latest version. Cannot get crashes without a version.")
            }
        }
    }

    pub fn error_group_details(
        &self,
        api: &impl API,
        error_group_id: &str,
        application: &str,
        organization: &str,
    ) -> Result<ErrorGroup, String> {
        let response = api.os_versions(organization, application, error_group_id);
        // json string
        match response {
            Ok(response) => {
                // parsed json structs
                let error_group =
                    crash_parsing::error_group_details_from_json(response.as_str()).unwrap();
                Ok(error_group)
            }

            Err(e) => Err(format!("Download failure {} for errorGroup", e)),
        }
    }
}
