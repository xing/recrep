pub mod api;
pub mod crashes;
pub mod json_parsing;
pub mod model;
pub mod utils;

#[macro_use]
extern crate serde_json;
extern crate handlebars;
extern crate serde;

use api::{AppCenter, API};
use handlebars::Handlebars;
use model::Report;
use utils::{FileWriter, Printing, StdOutPrinter, Writing};

/// The `CrashReporter` is the heart of `recrep`.
pub struct CrashReporter {
    pub token: String,
    organization: String,
    application: String,
    version: Option<String>,
    distribution_group: Option<String>,
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
    /// Some("My-Distribution-Group".to_string()));
    ///
    /// assert_eq!("abc", reporter.token);
    /// ```
    pub fn with_token(
        token: &str,
        organization: &str,
        application: &str,
        version: Option<String>,
        distribution_group: Option<String>,
    ) -> CrashReporter {
        CrashReporter {
            token: token.to_string(),
            organization: organization.to_string(),
            application: application.to_string(),
            version: version.map(|s| s.to_string()),
            file_writer: &FileWriter {},
            printer: &StdOutPrinter {},
            distribution_group: distribution_group,
        }
    }

    pub fn create_report(&self, outfile: Option<&str>) {
        match self.crashes() {
            Ok(crash_report) => self.report(crash_report, outfile),
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
    /// let reporter = CrashReporter::with_token("abc", "org name", "app id", None, None);
    /// let report = Report::new("version".to_string(), crash_list);
    /// reporter.report(report, None)
    /// ```
    pub fn report(&self, report: Report, path: Option<&str>) {
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
    /// let reporter = CrashReporter::with_token("abc", "org name", "app id", None, None);
    /// let report = TestHelper::report_from_json("src/json_parsing/test_fixtures/two_crashes.json");
    /// let formatted_report = reporter.format_report(report);
    /// assert_eq!(formatted_report.chars().count(), 1100)
    /// ```
    pub fn format_report(&self, report: Report) -> String {
        let mut template = Handlebars::new();
        template
            .register_template_string("crashes_template", self.crashes_template())
            .expect("Failed to register the crashes template.");

        let mut crash_list_json = json!(report.crash_list);
        let data = crash_list_json.as_object_mut().unwrap();
        data.insert(
            "organization".to_string(),
            json!(self.organization.to_string()),
        );
        data.insert(
            "application".to_string(),
            json!(self.application.to_string()),
        );
        data.insert("version".to_string(), json!(report.version));
        template.render("crashes_template", &json!(data)).unwrap()
    }

    fn crashes_template<'a>(&self) -> &'a str {
        r#"
Hello everyone!

This is the crash newsletter of v{{version}}.

{{#each errorGroups}}
First appeared on {{ firstOccurrence }} and occurred {{ count }} times in {{ appVersion }}/{{appBuild}} and affected {{deviceCount}} devices.
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

    fn crashes(&self) -> Result<Report, &'static str> {
        let api = AppCenter::new(self.token.clone());
        self.crashes_from_api(api)
    }

    fn crashes_from_api(&self, api: impl API) -> Result<Report, &'static str> {
        let crash_manager = crashes::CrashManager {};
        crash_manager.crash_list(
            &api,
            self.organization.as_str(),
            self.application.as_str(),
            self.version.clone(),
            self.distribution_group.clone(),
        )
    }
}
