pub mod utils;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Parsing options to instruct the parser.
#[allow(dead_code)]
pub struct ParserOptions {
    /// Line range to parse. Given as a colon-delimited string ("0:100").
    line_range: Option<[u32; 2]>,
    /// Date range to parse. Given as a double-colon-delimited string ("2022-01-01 08:00::2022-01-02 08:00").
    date_range: Option<[String; 2]>,
    /// Function to parse for. Given as a string ("my_function()").
    function: Option<String>,
}

impl ParserOptions {
    pub fn new(
        line_range: Option<[u32; 2]>,
        date_range: Option<[String; 2]>,
        function: Option<String>,
    ) -> ParserOptions {
        ParserOptions {
            line_range,
            date_range,
            function,
        }
    }
}

/// Configuration for the log file parser.
#[allow(dead_code)]
pub struct ParserConfig {
    /// Relative or absolute path to the target log file.
    filepath: String,
    /// Options to configure the parser with.
    options: ParserOptions,
}

impl ParserConfig {
    pub fn new(filepath: String, options: ParserOptions) -> ParserConfig {
        ParserConfig { filepath, options }
    }
}

/// Parser used to parse log files.
pub struct Parser {
    /// Parser configuration.
    config: ParserConfig,
}

impl Parser {
    pub fn new(config: ParserConfig) -> Parser {
        Parser { config }
    }

    /// Parse and display log file using parser configuration.
    pub fn parse(&self) {
        if let Ok(lines) = read_lines(&self.config.filepath) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.flatten() {
                println!("{}", line);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// TODO
#[cfg(test)]
mod tests {}
