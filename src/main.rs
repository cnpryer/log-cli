use clap::{arg, value_parser, ArgAction, ArgGroup, Command};
use log_cli::{parse, LogCLI};
use std::path::PathBuf;

const VERSION: &str = "0.0.8";

fn main() {
    // Create main clap command.
    let cmd = Command::new("log-cli")
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
                .value_parser(parse::parse_line_range_value)
                .multiple_values(true)
                .min_values(1)
                .max_values(2)
                .help("Line number range to display. Must be a valid integer range format (ex: 0 10 to display lines 0 through 10)."),
        )
        // .arg(
        //     arg!(--"date-range" <VALUE>)
        //         .required(false)
        //         .value_parser(parse::parse_date_range_value)
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
            arg!(--tail <VALUE>).default_missing_value("5")
            .required(false).value_parser(value_parser!(usize))
            .help("Display the bottom VALUE lines.")
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
        )
        .arg(
            arg!(--latest <VALUE>)
            .required(false).value_parser(value_parser!(usize))
            .default_missing_value("1")
            .help("Set evaluation strategy to 'latest' VALUE lines.")
        )
        .group(ArgGroup::new("ranges")
            .args(&["line-range", "tail", "head"])
            .multiple(false))
        .group(ArgGroup::new("primary-evaluations")
            .args(&["any", "all"])
            .multiple(false)
        );

    // Get arguments.
    let matches = cmd.get_matches();

    // Path to log file to read.
    let filepath = matches
        .get_one::<PathBuf>("LOG_FILE")
        .expect("A valid path to a log file is required.");

    // Initialize the application.
    // TODO: LogCLI::with_clap(&matches);
    let app = LogCLI::from_clap(&matches);

    // Attempt to display the contents otherwise print the error.
    if let Err(e) = app.run_with_filepath(filepath) {
        eprintln!("Error: {:?}", e)
    }
}
