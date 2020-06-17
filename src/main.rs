extern crate recrep;
#[macro_use]
extern crate clap;

use clap::{App, Arg, ArgMatches};
use recrep::CrashReporter;

fn main() {
    let app = app();
    let matches = matches_for_app(app);
    let token = matches.value_of("token").expect("Token is required.");
    let version = matches.value_of("version");
    let outfile = matches.value_of("outfile");
    let organization = matches.value_of("organization").unwrap();
    let application = matches.value_of("application").unwrap();

    let v = version.map(|s| s.to_string());
    let crash_reporter = CrashReporter::with_token(token, organization, application, v);
    crash_reporter.create_report(outfile);
}

fn app<'a, 'b>() -> App<'a, 'b> {
    App::new("recrep")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Recycled Crep: Look at your crashes.")
}

fn matches_for_app<'a>(app: App<'a, '_>) -> ArgMatches<'a> {
    app.args(&[
        Arg::with_name("token")
            .help("The AppCenter API token")
            .takes_value(true)
            .short("t")
            .long("token")
            .env("RECREP_APPCENTER_API_TOKEN")
            .required(true),
        Arg::with_name("version")
            .help("The app version. If none is specified, the latest available version will be picked - be aware that the latest version might not have crashes yet.")
            .takes_value(true)
            .short("v")
            .long("version")
            .required(false),
        Arg::with_name("organization")
            .help("The organization the app belongs to.")
            .takes_value(true)
            .short("c")
            .long("organization")
            .required(true),
        Arg::with_name("application")
            .help("The application identifier as seen in AppCenter urls.")
            .takes_value(true)
            .short("a")
            .long("application")
            .required(true),
        Arg::with_name("outfile")
            .help("An optional filename to write the report to instead of printing it to the console.")
            .takes_value(true)
            .short("o")
            .long("outfile")
            .required(false),
    ])
    .get_matches()
}