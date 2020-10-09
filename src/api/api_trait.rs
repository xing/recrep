pub trait API {
    /// Create a new API
    ///
    /// ```
    /// use recrep::api::API;
    /// use recrep::api::AppCenter;
    /// #
    /// let api = AppCenter::new("abc".to_string());
    /// assert_eq!(api.token, "abc");
    /// ```
    fn new(token: String) -> Self;

    /// Get the latest available version
    fn latest_version(
        &self,
        organization: String,
        application: String,
    ) -> Result<String, &'static str>;

    /// Get the json for crashes
    fn crashes_json(
        &self,
        organization: String,
        application: String,
        version: String,
    ) -> Result<String, &'static str>;

    fn os_versions(
        &self,
        organization: &str,
        application: &str,
        error_group_id: &str,
    ) -> Result<String, &'static str>;
}
