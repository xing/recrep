use clap::{crate_authors, crate_version};
use clap::{App, Arg, ArgMatches};
use recrep::CrashReporter;

fn main() {
    let app = app();
    let matches = matches_for_app(app);
    let token = matches.value_of("token").expect("Token is required.");
    let version = matches.value_of("version");
    let outfile = matches.value_of("outfile");
    let organization = matches
        .value_of("organization")
        .expect("Organization is required");
    let application = matches
        .value_of("application")
        .expect("Application is required");
    let distribution_group = matches.value_of("distribution-group");
    let crash_threshold = matches.value_of("threshold");
    let use_arithmetic_mean = matches.is_present("arithmetic-mean");
    let filter_out_errors = matches.is_present("omit-errors");
    let version = version.map(String::from);
    let group = distribution_group.map(String::from);
    let crash_threshold =
        crash_threshold.map(|x| x.parse::<u64>().expect("Invalid number provided"));
    let show_os_information = matches.is_present("show-operating-systems");
    let crash_reporter = CrashReporter::with_token(
        token,
        organization,
        application,
        version,
        group,
        crash_threshold,
        use_arithmetic_mean,
        show_os_information,
        filter_out_errors,
    );
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
        Arg::with_name("distribution-group")
            .help("Distribution group used to search for the latest version released into this distribution group.")
            .takes_value(true)
            .short("g")
            .long("group")
            .required(false),
        Arg::with_name("threshold")
            .help("Set a threshold value to determine a baseline amount of crashes. This max represents 100%. A percentage is shown of how many percent of this maximum is reached for each crash in the crash list. Crashes exceeding the threshold are marked as such.")
            .takes_value(true)
            .short("m")
            .long("threshold")
            .required(false),
        Arg::with_name("arithmetic-mean")
            .help("Use the arithmetic mean as threshold value, and only show crashes exceeding this value. This flag is incompatible with `--threshold`.")
            .takes_value(false)
            .long("arithmetic-mean")
            .required(false)
            .conflicts_with("threshold"),
        Arg::with_name("show-operating-systems")
            .help("Show the operating systems affected for each crash.")
            .takes_value(false)
            .long("show-operating-systems")
            .required(false),
        Arg::with_name("omit-errors")
            .help("Filters out AppCenter \"Crashes\" that are classified as `Error`.")
            .takes_value(false)
            .long("omit-errors")
            .required(false),
    ])
    .get_matches()
}
