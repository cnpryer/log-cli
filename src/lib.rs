//! log-cli: <small>Command line interface for log files.</small>
//!
//! ## Installation
//!
//! To install `log-cli` run:
//!
//! ```console
//! cargo install log-cli
//! ```
//!
//! ## Usage
//!
//! ```console
//! > log-cli sample.log
//!
//! ln00 2022-01-01 07:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln01 2022-01-01 08:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln02 2022-01-01 09:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ln03 2022-01-01 10:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ln04 2022-01-01 11:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ln05 2022-01-01 12:00:00,0 [info] module3  Message Subject: Text for a message.
//! ln06 2022-01-01 13:00:00,0 [info] module3  Message Subject: Text for a message.
//! ln07 2022-01-01 14:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln08 2022-01-01 15:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln09 2022-01-01 16:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln10 2022-01-01 17:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln11 2022-01-01 18:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln12 2022-01-01 19:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln13 2022-01-01 20:00:00,0 [debug] module5  Message Subject: Text for a message.
//! ln14 2022-01-01 21:00:00,0 [info] module2  Message Subject: Text for a message.
//! ln15 2022-01-01 22:00:00,0 [info] module2  Message Subject: Text for a message.
//! ln16 2022-01-01 23:00:00,0 [info] module6  Message Subject: Text for a message.
//! ln17 2022-01-02 00:00:00,0 [warning] module1  Message Subject: Text for a message.
//! ln18 2022-01-02 01:00:00,0 [info] module10  Message Subject: Text for a message.
//! ln19 2022-01-02 02:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln20 2022-01-02 03:00:00,0 [debug] module12  Message Subject: Text for a message.
//! ln21 2022-01-02 04:00:00,0 [warning] module11  Message Subject: Text for a message.
//! ln22 2022-01-02 05:00:00,0 [info] module7  Message Subject: Text for a message.
//! ln23 2022-01-02 06:00:00,0 [info] module6  Message Subject: Text for a message.
//! ```
//!
//! ### View using keywords
//!
//! ```console
//! > log-cli sample.log --keywords "[debug]" module2
//!
//! ln2 2022-01-01 09:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ln3 2022-01-01 10:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ln4 2022-01-01 11:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ```
//!
//! ### View using a line range
//!
//! ```console
//! > log-cli sample.log --line-range 20 30
//!
//! ln20 2022-01-02 03:00:00,0 [debug] module12  Message Subject: Text for a message.
//! ln21 2022-01-02 04:00:00,0 [warning] module11  Message Subject: Text for a message.
//! ln22 2022-01-02 05:00:00,0 [info] module7  Message Subject: Text for a message.
//! ln23 2022-01-02 06:00:00,0 [info] module6  Message Subject: Text for a message.
//! ```
//!
//! ### More usage
//!
//! ```console
//! > log-cli --help
//!
//! Command line interface for log files.
//!
//! USAGE:
//!     log-cli [OPTIONS] [--] [LOG_FILE]
//!
//! ARGS:
//!     <LOG_FILE>    Path to log file to be read. By default if no additional flags are passed the
//!                   entire file will be displayed.
//!
//! OPTIONS:
//!         --all                      Set evaluation strategy to 'all'.
//!         --any                      Set evaluation strategy to 'any'.
//!     -h, --help                     Print help information
//!         --head <VALUE>             Display the top VALUE lines.
//!         --keywords <VALUE>...      Keywords to search for in the log file. Multiple keywords can be
//!                                    passed (ex: these are all keywords).
//!         --line-range <VALUE>...    Line number range to display. Must be a valid integer range
//!                                    format (ex: 0 10 to display the first 10 lines).
//!         --tail <VALUE>             Display the bottom VALUE lines.
//!     -V, --version                  Print version information
//! ```

/// Command implementations.
pub mod command;
/// Parsing implementations.
pub mod parse;
/// `File` reading logic for log files.
pub mod read;
/// Display implementations.
pub mod view;
