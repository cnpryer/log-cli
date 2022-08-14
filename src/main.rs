use std::path::PathBuf;

use clap::{arg, value_parser, App, Command, ErrorKind};
use log_cli::{read::read_file, validate, view::Viewer};

fn main() {
    // Create main clap command.
    let mut app = cli();
    // Get arguments.
    let matches = app.get_matches_mut();

    // Get optional argument values.
    let keywords = matches.get_many::<String>("keywords");
    let line_range = matches.get_many::<u32>("line-range");
    let date_range = matches.get_many::<String>("date-range");

    // Certain arguments cannot be used together. Error if this is the case.
    if line_range.is_some() && date_range.is_some() {
        app.error(
            ErrorKind::ArgumentConflict,
            "Cannot use both line-range and date-range together.",
        )
        .exit();
    }

    // Path to log file to read.
    let filepath = matches
        .get_one::<PathBuf>("LOG_FILE")
        .expect("A valid path to a log file is required.");

    // Read and display log file.
    let buffer = &mut read_file(filepath).expect("Unable to read filepath.");
    let viewer = Viewer::new(keywords, line_range, date_range);
    viewer.display_with(buffer);
}

fn cli() -> App<'static> {
    let app = Command::new("log-cli")
        .version("0.0.1-alpha.1")
        .propagate_version(true)
        .author("Chris Pryer <cnpryer@gmail.com>")
        .about("Command line interface for log files.")
        // Path to log file to be read. By default if no additional flags are passed the entire file will be displayed.
        .arg(arg!([LOG_FILE]).value_parser(value_parser!(PathBuf)))
        // Keywords to search for in the log file. Multiple keywords can be passed (these are all keywords).
        .arg(
            arg!(--keywords <VALUE>)
                .required(false)
                .multiple_values(true)
                .min_values(1),
        )
        // Line number range to display. Must be a valid integer range format (0 10 to display the first 10 lines).
        .arg(
            arg!(--"line-range" <VALUE>)
                .required(false)
                .value_parser(validate::valid_line_range_value)
                .multiple_values(true)
                .min_values(1)
                .max_values(2),
        )
        // Date range to display. Must be a valid date range format ("2022-01-01" "2022-01-02").
        .arg(
            arg!(--"date-range" <VALUE>)
                .required(false)
                .value_parser(validate::valid_date_range_value)
                .multiple_values(true)
                .min_values(1)
                .max_values(2),
        );

    app
}
