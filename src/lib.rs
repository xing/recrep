pub mod api;
pub mod crashes;
pub mod json_parsing;
pub mod model;
pub mod utils;

#[macro_use]
extern crate serde_json;
extern crate handlebars;
extern crate serde;

use crate::model::{OperatingSystemCount, Report};
use api::{AppCenter, API};
use handlebars::Handlebars;
use std::collections::HashMap;
use utils::{FileWriter, Printing, StdOutPrinter, Writing};

/// The `CrashReporter` is the heart of `recrep`.
pub struct CrashReporter {
    pub token: String,
    organization: String,
    application: String,
    version: Option<String>,
    distribution_group: Option<String>,
    threshold: Option<u64>,
    use_arithmetic_mean: bool,
    show_os_information: bool,
    filter_out_errors: bool,
    file_writer: &'static dyn Writing,
    printer: &'static dyn Printing,
}

impl CrashReporter {
    ///
    /// Create a crash reporter
    ///
    ///```
    /// use recrep::CrashReporter;
    ///
    /// let reporter = CrashReporter::with_token("abc", "org", "app", Some("1.2.3".to_string()),
    /// Some("My-Distribution-Group".to_string()), None, false, false, false);
    ///
    /// assert_eq!("abc", reporter.token);
    /// ```
    pub fn with_token(
        token: &str,
        organization: &str,
        application: &str,
        version: Option<String>,
        distribution_group: Option<String>,
        threshold: Option<u64>,
        use_arithmetic_mean: bool,
        show_os_information: bool,
        filter_out_errors: bool,
    ) -> CrashReporter {
        CrashReporter {
            token: token.to_string(),
            organization: organization.to_string(),
            application: application.to_string(),
            version: version.map(|s| s.to_string()),
            file_writer: &FileWriter {},
            printer: &StdOutPrinter {},
            distribution_group: distribution_group,
            threshold: threshold,
            use_arithmetic_mean: use_arithmetic_mean,
            show_os_information: show_os_information,
            filter_out_errors: filter_out_errors,
        }
    }

    pub fn create_report(&self, outfile: Option<&str>) {
        match self.crashes_from_app_center() {
            Ok(mut crash_report) => {
                if self.show_os_information {
                    let error_groups = self.download_group_details_for_crashes(&crash_report);
                    crash_report.assign_operating_system_details(error_groups);
                }

                self.write_report(crash_report, outfile)
            }
            Err(x) => println!("Failed to get list of crashes with error: {:}", x),
        }
    }

    /// Reports to standard out. Optionally writes into a file at the given path
    ///
    /// ```
    /// # use recrep::utils::test_helper::TestHelper;
    /// # use recrep::CrashReporter;
    /// # use recrep::model::Report;
    /// #
    /// # let crash_list = TestHelper::crash_list_from_json("src/json_parsing/test_fixtures/two_crashes.json");
    /// let reporter = CrashReporter::with_token("abc", "org name", "app id", None, None, None,
    /// false, false, false);
    /// let report = Report::new("version".to_string(), crash_list);
    /// reporter.write_report(report, None)
    /// ```
    pub fn write_report(&self, report: Report, path: Option<&str>) {
        let formatted_report = self.format_report(report);
        match path {
            Some(file_path) => self
                .file_writer
                .write(formatted_report, file_path.to_string()),
            None => self.printer.print(formatted_report),
        }
    }

    /// Formats a crash report using the default template
    ///
    /// ```
    /// # use recrep::utils::test_helper::TestHelper;
    /// # use recrep::CrashReporter;
    /// #
    /// let reporter = CrashReporter::with_token("abc", "org name", "app id", None, None, None,
    /// false, false, false);
    /// let report = TestHelper::report_from_json("src/json_parsing/test_fixtures/two_crashes.json");
    /// let formatted_report = reporter.format_report(report);
    /// assert_eq!(formatted_report.chars().count(), 1352)
    /// ```
    pub fn format_report(&self, report: Report) -> String {
        let mut crash_list_json: serde_json::Value = json!(report.crash_list);
        let data = crash_list_json.as_object_mut().unwrap();

        if self.filter_out_errors {
            self.filter_out_errors(data);
        }

        data.insert(
            "organization".to_string(),
            json!(self.organization.to_string()),
        );
        data.insert(
            "application".to_string(),
            json!(self.application.to_string()),
        );
        data.insert("version".to_string(), json!(report.version));

        if data
            .get("errorGroups")
            .unwrap()
            .as_array()
            .unwrap()
            .is_empty()
        {
            let mut template = Handlebars::new();
            template
                .register_template_string(
                    "no_crashes_found_template",
                    self.no_crashes_found_template(),
                )
                .expect("Failed to register the no crashes found template.");
            return template
                .render("no_crashes_found_template", &json!(data))
                .unwrap();
        }

        if let Some(threshold) = self.threshold {
            self.add_threshold_values_to_individual_crashes(data, threshold);
            data.insert("threshold".to_string(), json!(threshold));
        }

        if self.use_arithmetic_mean {
            self.add_arithmetic_mean(data);
        }

        if self.show_os_information {
            self.add_operating_system_information(data);
        }

        data.insert(
            "show_oses_affected".to_string(),
            json!(self.show_os_information),
        );
        let mut template = Handlebars::new();
        template
            .register_template_string("crashes_template", self.crashes_template())
            .expect("Failed to register the crashes template.");
        template.render("crashes_template", &json!(data)).unwrap()
    }

    fn crashes_template<'a>(&self) -> &'a str {
        r#"
Hello everyone!

This is the crash newsletter of v{{version}}

If your team is assigned to take a look at a certain crash but decides to not to be responsible for handling the crash, please forward it 
to another team or the community yourself.
Please be proactive to raise awareness of crashes among the community - especially if your team can't address a crash at hand on its own.

{{#if arithmetic_mean }}
This Crash Report uses a threshold based on the arithmetic mean of all crashes ({{ arithmetic_mean }}). Crashes that occurred less than (<) {{ arithmetic_mean }} times are excluded.
{{~/if}}
{{#each errorGroups}}
{{~#if threshold_exceeded}}
!! THRESHOLD EXCEEDED !! 
{{/if}}
{{~#if threshold }}
{{ percentage }} ({{ count }}/{{threshold}}) of threshold reached. (crashes/threshold)
{{~else}}
{{ count }} times in {{ appVersion }} ({{appBuild}})
{{~/if}}
{{~#if operatingSystemName}}
Affected OSes: {{operatingSystemName}} on {{ deviceCount }} overall affected devices
{{~/if}}

First appeared on {{ firstOccurrence }}
{{~#if exceptionFile}}
File:    {{exceptionFile}}
{{~ /if ~}}
{{~#if exceptionClassName}}
Class:   {{exceptionClassName}}
{{~/if ~}}
{{~#if exceptionMethod}}
Method:  {{exceptionMethod}}
{{~/if~}}
{{~#if errorGroupId}}
More on AppCenter: https://appcenter.ms/orgs/{{../organization}}/apps/{{../application}}/crashes/errors/{{errorGroupId}}/overview.
{{~/if}}

-------------------------------
{{/each}}


Haven't found your section above? Please checkout the rest of the crashes in the diagnostics overview: https://appcenter.ms/orgs/{{organization}}/apps/{{application}}/crashes/errors?version={{version}}&period=last30Days&status=&errorType=unhandlederror&sortCol=count&sortDir=desc.

Cheers,
The Mobile Releases Team


This report was created using `recrep` for {{organization}}/{{application}}/{{version}}."#
    }

    fn no_crashes_found_template<'a>(&self) -> &'a str {
        r#"
Hello everyone!

This is the crash newsletter of v{{version}}

Luckily this version does not have any crashes AppCenter knows about. Congratulations 🎉!

This report was created using `recrep` for {{organization}}/{{application}}/{{version}}.
"#
    }

    fn add_threshold_values_to_individual_crashes(
        &self,
        crash_data: &mut serde_json::Map<String, serde_json::Value>,
        threshold: u64,
    ) {
        let value = crash_data.get_mut("errorGroups").unwrap();
        let all_crashes: &mut Vec<serde_json::Value> = value.as_array_mut().unwrap();

        for crash_obj in all_crashes.iter_mut() {
            let crash = crash_obj.as_object_mut().unwrap();
            let percentage = (crash["count"].as_u64().unwrap() as f32 / threshold as f32) * 100f32;
            if percentage >= 100.0 {
                crash.insert(
                    "percentage".to_string(),
                    json!(format!("{:.2}%", percentage)),
                );
                crash.insert("threshold_exceeded".to_string(), json!(threshold));
            } else {
                // remove this to get rid of displaying crashes below threshold
                crash.insert(
                    "percentage".to_string(),
                    json!(format!("{:.2}%", percentage)),
                );
            }
            crash.insert("threshold".to_string(), json!(threshold));
        }
    }

    fn filter_out_errors(&self, crash_data: &mut serde_json::Map<String, serde_json::Value>) {
        let value = &mut crash_data["errorGroups"];
        let all_crashes: &mut Vec<serde_json::Value> = value.as_array_mut().unwrap();

        // exceptionAppCode is set to true on crashes and false on errors
        all_crashes.retain(|crash| crash["exceptionAppCode"] == true);
    }

    fn add_operating_system_information(
        &self,
        crash_data: &mut serde_json::Map<String, serde_json::Value>,
    ) {
        let value = &mut crash_data["errorGroups"];
        let all_crashes: &mut Vec<serde_json::Value> = value.as_array_mut().unwrap();

        // object: Object<Map<String, Value>>
        for object in all_crashes.iter_mut() {
            let crash = object.as_object_mut().unwrap();
            let oses = crash["operating_systems"].as_array().unwrap();
            let crash_amount_devices_overall = crash["deviceCount"].as_f64().unwrap();
            let mut formatted = oses
                .iter()
                .filter_map(|os| {
                    let crash_amount_os = os["errorCount"].as_f64().unwrap();
                    let percentage =
                        (crash_amount_os / crash_amount_devices_overall) as f64 * 100.0;
                    if percentage > 5.0 {
                        let os_string = os["operatingSystemName"].as_str().unwrap();
                        Some(format!(
                            "{}: {} crashes ({:.2}%) ",
                            os_string, crash_amount_os, percentage
                        ))
                    } else {
                        None
                    }
                })
                .fold(String::new(), |mut formatted, sub| {
                    formatted += sub.as_str();
                    formatted += "| ";
                    formatted
                });

            // remove trailing "| ", as the previous 'fold' section can't know when
            // to not put the suffix.
            formatted.truncate(formatted.len() - 3);
            let amount_of_affected_oses_shown = formatted.matches("|").count();
            if amount_of_affected_oses_shown < crash["deviceCount"].as_u64().unwrap() as usize {
                formatted += " and more"
            }
            crash.insert("operatingSystemName".to_string(), json!(formatted));
        }
    }
    fn add_arithmetic_mean(&self, crash_data: &mut serde_json::Map<String, serde_json::Value>) {
        // sum of all crashes / amount of crashes
        // {"errorGroups": Array([…])}
        let value = crash_data.get_mut("errorGroups").unwrap();
        let all_crashes: &mut Vec<serde_json::Value> = value.as_array_mut().unwrap();
        let mut sum_of_all_crash_occurrences = 0;

        // object: Object<Map<String, Value>>
        for object in all_crashes.iter_mut() {
            let crash = object.as_object_mut().unwrap();
            let occurrences_of_crash = crash["count"].as_u64().unwrap();
            sum_of_all_crash_occurrences += occurrences_of_crash;
        }
        if sum_of_all_crash_occurrences == 0 {
            return;
        }
        let arithmetic_mean = sum_of_all_crash_occurrences / all_crashes.len() as u64;

        // remove crashes below arithmetic mean threshold
        all_crashes.retain(|x| {
            let crash = x.as_object().unwrap();
            let occurrences_of_crash = crash["count"].as_u64().unwrap();

            // true keep, false remove
            occurrences_of_crash >= arithmetic_mean
        });
        crash_data.insert("arithmetic_mean".to_string(), json!(arithmetic_mean));
    }

    fn crashes_from_app_center(&self) -> Result<Report, &'static str> {
        let api = AppCenter::new(self.token.clone());
        self.crashes_from_api(api)
    }

    fn crashes_from_api(&self, api: impl API) -> Result<Report, &'static str> {
        let crash_downloader = crashes::CrashManager {};

        crash_downloader.crash_list(
            &api,
            self.organization.as_str(),
            self.application.as_str(),
            self.version.clone(),
            self.distribution_group.clone(),
        )
    }

    fn download_group_details_for_crashes(
        &self,
        crash_report: &Report,
    ) -> HashMap<String, Vec<OperatingSystemCount>> {
        let api = AppCenter::new(self.token.clone());
        let crash_downloader = crashes::CrashManager {};

        let mut error_groups: HashMap<String, Vec<OperatingSystemCount>> = HashMap::new();
        for crash in crash_report.crash_list.crashes.iter() {
            if let Some(error_group_id) = &crash.error_group_id {
                match crash_downloader.error_group_details(
                    &api,
                    error_group_id.as_str(),
                    self.application.as_str(),
                    self.organization.as_str(),
                ) {
                    Ok(error_group) => {
                        error_groups.insert(
                            crash.error_group_id.as_ref().unwrap().to_string(),
                            error_group.operating_systems,
                        );
                    }
                    Err(e) => {
                        println!(
                            "No error group found, for ID: {}. Error: {}",
                            error_group_id, e
                        );
                        continue;
                    }
                }
            }
        }
        return error_groups;
    }
}

#[test]
//Formats a crash report including a threshold value
fn test_report_formatting_supports_threshold() {
    let reporter = CrashReporter::with_token(
        "abc",
        "org name",
        "app id",
        None,
        None,
        Some(300),
        false,
        false,
        false,
    );
    let report = utils::test_helper::TestHelper::report_from_json(
        "src/json_parsing/test_fixtures/two_crashes.json",
    );
    let formatted_report = reporter.format_report(report);
    assert_eq!(formatted_report.chars().count(), 1412)
}

#[test]
fn test_filtering_out_errors() {
    let reporter = CrashReporter::with_token(
        "abc",
        "org name",
        "app id",
        None,
        None,
        Some(300),
        false,
        false,
        true,
    );
    let report = utils::test_helper::TestHelper::report_from_json(
        "src/json_parsing/test_fixtures/crashes.json",
    );

    let mut crash_list_json: serde_json::Value = json!(report.crash_list);
    let data = crash_list_json.as_object_mut().unwrap();
    reporter.filter_out_errors(data);

    let value = &mut data["errorGroups"];
    let all_crashes: &mut Vec<serde_json::Value> = value.as_array_mut().unwrap();
    assert_eq!(all_crashes.len(), 26);
}

#[test]
fn test_report_formatting_supports_filtering_out_errors() {
    let reporter = CrashReporter::with_token(
        "abc",
        "org name",
        "app id",
        None,
        None,
        Some(300),
        false,
        false,
        true, // <- switch to omit-errors from report
    );
    let report = utils::test_helper::TestHelper::report_from_json(
        "src/json_parsing/test_fixtures/crashes.json",
    );

    let mut crash_list_json: serde_json::Value = json!(report.crash_list);
    let data = crash_list_json.as_object_mut().unwrap();
    let amount_of_crashes_in_fixture = data.get("errorGroups")
                                           .unwrap()
                                           .as_array()
                                           .unwrap().len();

    let formatted_report = reporter.format_report(report);
    let amount_of_formatted_crashes = formatted_report.matches("More on AppCenter").count();
    assert!(amount_of_crashes_in_fixture != amount_of_formatted_crashes);
}


#[test]
fn test_report_formatting_does_not_filter_out_errors() {
    let reporter = CrashReporter::with_token(
        "abc",
        "org name",
        "app id",
        None,
        None,
        Some(300),
        false,
        false,
        false, // <- switch to not omit errors from report
    );
    let report = utils::test_helper::TestHelper::report_from_json(
        "src/json_parsing/test_fixtures/crashes.json",
    );

    let mut crash_list_json: serde_json::Value = json!(report.crash_list);
    let data = crash_list_json.as_object_mut().unwrap();
    let amount_of_crashes_in_fixture = data.get("errorGroups")
                                           .unwrap()
                                           .as_array()
                                           .unwrap().len();

    let formatted_report = reporter.format_report(report);
    let amount_of_formatted_crashes = formatted_report.matches("More on AppCenter").count();
    assert_eq!(amount_of_crashes_in_fixture, amount_of_formatted_crashes);
}

#[test]
fn test_report_template_if_no_crash_exists() {
    let reporter = CrashReporter::with_token(
        "abc",
        "org name",
        "app id",
        None,
        None,
        Some(300),
        false,
        false,
        false,
    );
    let report = utils::test_helper::TestHelper::report_from_json(
        "src/json_parsing/test_fixtures/no_crashes.json",
    );
    let formatted_report = reporter.format_report(report);
    assert_eq!(formatted_report.chars().count(), 218)
}
