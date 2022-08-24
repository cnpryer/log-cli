use std::path::PathBuf;

use chrono::NaiveDate;
use clap::{
    self, arg, crate_authors, crate_version, value_parser, Arg, ArgAction, ArgGroup, Command,
};

const ABOUT: &str = "Command line interface for log files.";

pub fn app() -> Command<'static> {
    let mut app = Command::new("log-cli")
        .author(crate_authors!())
        .version(crate_version!())
        .about(ABOUT)
        .help_message("Prints help information. Use --help for more details.");

    for arg in all_args() {
        app = app.arg(arg);
    }

    for group in all_groups() {
        app = app.group(group);
    }

    app
}

fn all_args() -> Vec<Arg<'static>> {
    vec![
        paths_arg(),
        all_flag(),
        any_flag(),
        head_flag(),
        keywords_flag(),
        latest_flag(),
        line_range_flag(),
        tail_flag(),
    ]
}

fn all_groups() -> Vec<ArgGroup<'static>> {
    vec![evaluation_group(), range_group()]
}

fn paths_arg() -> Arg<'static> {
    arg!([LOG_FILE])
        .id("paths")
        .required(true)
        .multiple_values(true)
        .value_parser(value_parser!(PathBuf))
        .help("Path to log file to be read.")
        .long_help(
            "\
Path to log file to be read. By default if no additional flags are passed the entire file will be 
displayed.",
        )
}

fn keywords_flag() -> Arg<'static> {
    arg!(--keywords <VALUE>)
        .required(false)
        .multiple_values(true)
        .min_values(1)
        .help("Keywords to search for in the log file.")
        .long_help(
            "\
Keywords to search for in the log file. Multiple keywords can be passed (ex: these are all 
keywords).",
        )
}

fn line_range_flag() -> Arg<'static> {
    arg!(--"line-range" <VALUE>)
        .required(false)
        .value_parser(parse_line_range_value)
        .multiple_values(true)
        .min_values(1)
        .max_values(2)
        .help("Line number range to display.")
        .long_help(
            "\
Line number range to display. Must be a valid integer range format (ex: 0 10 to display lines 0 
through 10).",
        )
}

fn head_flag() -> Arg<'static> {
    arg!(--head <VALUE>)
        .default_missing_value("5")
        .required(false)
        .value_parser(value_parser!(usize))
        .help("Display the top VALUE lines.")
}

fn tail_flag() -> Arg<'static> {
    arg!(--tail <VALUE>)
        .default_missing_value("5")
        .required(false)
        .value_parser(value_parser!(usize))
        .help("Display the bottom VALUE lines.")
}

fn all_flag() -> Arg<'static> {
    arg!(--all)
        .required(false)
        .takes_value(false)
        .action(ArgAction::SetTrue)
        .help("Set evaluation strategy to 'all'.")
}

fn any_flag() -> Arg<'static> {
    arg!(--any)
        .required(false)
        .takes_value(false)
        .action(ArgAction::SetTrue)
        .help("Set evaluation strategy to 'any'.")
}

fn latest_flag() -> Arg<'static> {
    arg!(--latest <VALUE>)
        .required(false)
        .value_parser(value_parser!(usize))
        .default_missing_value("1")
        .help("Set evaluation strategy to 'latest' VALUE lines.")
}

fn range_group() -> ArgGroup<'static> {
    ArgGroup::new("ranges")
        .args(&["line-range", "tail", "head"])
        .multiple(false)
}

fn evaluation_group() -> ArgGroup<'static> {
    ArgGroup::new("primary-evaluations")
        .args(&["any", "all"])
        .multiple(false)
}

/// Parse line range argument value(s). Return vector of at most integers or error.
/// A valid line range can be either one line number (1 for line one) or two line numbers to
/// indicate multiple lines (0 20 to get first 20 lines).
pub fn parse_line_range_value(value: &str) -> Result<usize, String> {
    // Attempt to parse u32 values from string values.
    let res: usize = value
        .parse()
        .map_err(|_| format!("{} must be a valid usize.", value))?;

    Ok(res)
}

/// Validate that a string is date-like.
// NOTE: Currently strings are utilized internally. Evenaully `chrono` datetimes could be used,
//       but for now we only use `chrono` for validation purposes.
// TODO: Operate with `chrono` dates.
fn _is_date_like(value: &str) -> Result<bool, String> {
    // Attempt to parse string as date.
    let res = NaiveDate::parse_from_str(value, "%Y-%m-%d");

    // If we fail to parse `NaiveDate` correctly return false.
    if res.is_err() {
        return Ok(false);
    }

    // Otherwise correctly return true.
    Ok(true)
}

/// Parse date range argument value(s). Return vector of at most 2 strings or error.
/// A valid date range can be either one date string ("2022-01-01" for just January 1st) or two
/// strings to indicate an inclusive range of dates ("2022-01-01" "2022-01-02").
// TODO: Accept datetimes.
pub fn _parse_date_range_value(value: &str) -> Result<String, String> {
    // Check if value passed is a valid date formatted string.
    if let Ok(false) = _is_date_like(value) {
        return Err(format!("date format must be {}.", "%Y-%m-%d"));
    }

    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_range() {
        let valid_value = "0";
        let invalid_value = "foo";

        assert_eq!(parse_line_range_value(valid_value), Ok(0));
        assert_eq!(
            parse_line_range_value(invalid_value),
            Err(format!("{} must be a valid usize.", invalid_value))
        );
    }
}
