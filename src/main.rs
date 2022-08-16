use std::path::PathBuf;

use clap::{arg, value_parser, App, ArgAction, Command, ErrorKind};
use log_cli::{read::read_file, validate, view::Viewer};

const VERSION: &str = "0.0.4";

fn main() {
    // Create main clap command.
    let mut app = cli();
    // Get arguments.
    let matches = app.get_matches_mut();

    // Get optional argument values.
    let keywords = matches.get_many::<String>("keywords");
    let line_range = matches.get_many::<usize>("line-range");
    // TODO
    let date_range = None; //matches.get_many::<String>("date-range");
    let head = matches.get_one::<usize>("head");
    let all = matches.get_one::<bool>("all");
    let any = matches.get_one::<bool>("any");

    // Certain arguments cannot be used together. Error if this is the case.
    if line_range.is_some() && date_range.is_some() {
        app.error(
            ErrorKind::ArgumentConflict,
            "Cannot use both line-range and date-range together.",
        )
        .exit();
    }
    if line_range.is_some() && head.is_some() {
        app.error(
            ErrorKind::ArgumentConflict,
            "Cannot use both line-range and head together.",
        )
        .exit();
    }
    if date_range.is_some() && head.is_some() {
        app.error(
            ErrorKind::ArgumentConflict,
            "Cannot use both date-range and head together.",
        )
        .exit();
    }
    if let (Some(_all), Some(_any)) = (all, any) {
        if *_all && *_any {
            app.error(
                ErrorKind::ArgumentConflict,
                "Cannot use all and any flags together.",
            )
            .exit();
        }
    }

    // Path to log file to read.
    let filepath = matches
        .get_one::<PathBuf>("LOG_FILE")
        .expect("A valid path to a log file is required.");

    // Read the file to a buffer and build a viewer for view operations.
    let buffer = &mut read_file(filepath).expect("Unable to read filepath.");
    let viewer = Viewer::new(keywords, line_range, date_range, head, all, any);

    // Attempt to display the contents otherwise print the error.
    if let Err(e) = viewer.display_with(buffer) {
        println!("ERROR: {:?}", e)
    }
}

fn cli() -> App<'static> {
    let app = Command::new("log-cli")
        .version(VERSION)
        .propagate_version(true)
        .author("Chris Pryer <cnpryer@gmail.com>")
        .about("Command line interface for log files.")
        .arg(arg!([LOG_FILE]).value_parser(value_parser!(PathBuf))
        .help("Path to log file to be read. By default if no additional flags are passed the entire file will be displayed."))
        .arg(
            arg!(--keywords <VALUE>)
                .required(false)
                .multiple_values(true)
                .min_values(1)
                .help("Keywords to search for in the log file. Multiple keywords can be passed (ex: these are all keywords)."),
        )
        .arg(
            arg!(--"line-range" <VALUE>)
                .required(false)
                .value_parser(validate::valid_line_range_value)
                .multiple_values(true)
                .min_values(1)
                .max_values(2)
                .help("Line number range to display. Must be a valid integer range format (ex: 0 10 to display the first 10 lines)."),
        )
        // .arg(
        //     arg!(--"date-range" <VALUE>)
        //         .required(false)
        //         .value_parser(validate::valid_date_range_value)
        //         .multiple_values(true)
        //         .min_values(1)
        //         .max_values(2)
        //         .help("Date range to display. Must be a valid date range format (ex:\"2022-01-01\" \"2022-01-02\")."),
        //)
        .arg(
            arg!(--head <VALUE>).default_missing_value("5")
            .required(false).value_parser(value_parser!(usize))
            .help("Display the top VALUE lines.")
        )
        .arg(
            arg!(--all)
            .required(false)
            .takes_value(false)
            .action(ArgAction::SetTrue)
            .help("Set evaluation strategy to 'all'.")
        )
        .arg(
            arg!(--any)
            .required(false)
            .takes_value(false)
            .action(ArgAction::SetTrue)
            .help("Set evaluation strategy to 'any'.")
        );

    app
}
