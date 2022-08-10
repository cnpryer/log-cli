use clap::{arg, Command};
use log_cli::parser::{utils, Parser, ParserConfig, ParserOptions};

fn main() {
    // Create clap app.
    let app = Command::new("log-cli")
        .version("0.0.1-alpha.0")
        .author("Chris Pryer <cnpryer@gmail.com>")
        .about("Command line interface for log files.")
        .arg(arg!([filepath]))
        .arg(
            arg!(--line_range <VALUE>)
                .alias("line-range")
                .required(false),
        )
        .arg(
            arg!(--date_range <VALUE>)
                .alias("date-range")
                .required(false),
        )
        .arg(arg!(--keyword <VALUE>).required(false));
    let matches = app.get_matches();

    // Parse options from values passed.
    let line_range = utils::string_to_line_range_array(matches.get_one::<String>("line_range"));
    let date_range = utils::string_to_date_range_array(matches.get_one::<String>("date_range"));
    let keyword = utils::string_to_keyword_string(matches.get_one::<String>("keyword"));
    let parser_options = ParserOptions::new(line_range, date_range, keyword);

    // Set target file and parsing configuration.
    let filepath = utils::string_to_filepath_string(
        matches
            .get_one::<String>("filepath")
            .expect("No filepath was given."),
    );
    let parser_config = ParserConfig::new(filepath, parser_options);

    // Create parser and parse log file.
    let parser = Parser::new(parser_config);
    parser.parse();
}
