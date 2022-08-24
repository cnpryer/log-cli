//! log-cli: <small>Command line interface for log files.</small>
//!
//! ## Contents
//! - [Installation](#installation)
//! - [Basic Usage](#basic-usage)
//!   - [Keywords](#view-using-keywords)
//!   - [Line Range](#view-using-a-line-range)
//!   - [Head and Tail](#view-head-and-tail)
//! - [Advanced Usage](#more-advanced-usage)
//!   - [Keywords and Evaluation Strategies](#keywords-and-evaluation-strategies)
//!
//! ## Installation
//!
//! To install `log-cli` run:
//!
//! ```console
//! $ cargo install log-cli
//! ```
//!
//! ## Basic usage
//!
//! View the entire log file.
//!
//! ```console
//! $ log-cli sample.log
//!
//! File (1/1): sample.log
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
//! Pass keywords to filter for. By default `--keywords` will filter for lines where **all** keywords are found.
//!
//! ```console
//! $ log-cli sample.log --keywords "[debug]" module2
//!
//! File (1/1): sample.log
//! ln2 2022-01-01 09:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ln3 2022-01-01 10:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ln4 2022-01-01 11:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ```
//!
//! ### View using a line range
//!
//! Pass a line range. Ranges can be one to many lines. To view just the first line pass 0. To view a range of many lines pass
//! two values.
//!
//! ```console
//! $ log-cli sample.log --line-range 20 30
//!
//! File (1/1): sample.log
//! ln20 2022-01-02 03:00:00,0 [debug] module12  Message Subject: Text for a message.
//! ln21 2022-01-02 04:00:00,0 [warning] module11  Message Subject: Text for a message.
//! ln22 2022-01-02 05:00:00,0 [info] module7  Message Subject: Text for a message.
//! ln23 2022-01-02 06:00:00,0 [info] module6  Message Subject: Text for a message.
//! ```
//!
//! ### View head and tail
//!
//! Pass `--head` to view the top 5 lines.
//!
//! ```console
//! $ log-cli sample.log --head
//!
//! File (1/1): sample.log
//! ln0 2022-01-01 07:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln1 2022-01-01 08:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln2 2022-01-01 09:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ln3 2022-01-01 10:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ln4 2022-01-01 11:00:00,0 [debug] module2  Message Subject: Text for a message.
//! ```
//!
//! By default `--head` and `--tail` will return 5 lines. Pass a number to override this value.
//!
//! ```console
//! $ log-cli sample.log --tail 3
//!
//! File (1/1): sample.log
//! ln21 2022-01-02 04:00:00,0 [warning] module11  Message Subject: Text for a message.
//! ln22 2022-01-02 05:00:00,0 [info] module7  Message Subject: Text for a message.
//! ln23 2022-01-02 06:00:00,0 [info] module6  Message Subject: Text for a message.
//! ```
//!
//! ## More advanced usage
//!
//! Arguments can be combined for more complex use cases.
//!
//! ### Keywords and evaluation strategies
//!
//! Evaluation strategies can be used to configure viewing and filtering behaviors.
//!
//! ```console
//! $ log-cli sample.log --keywords "[debug]" "[info]" --any
//!
//! File (1/1): sample.log
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
//! ln18 2022-01-02 01:00:00,0 [info] module10  Message Subject: Text for a message.
//! ln19 2022-01-02 02:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln20 2022-01-02 03:00:00,0 [debug] module12  Message Subject: Text for a message.
//! ln22 2022-01-02 05:00:00,0 [info] module7  Message Subject: Text for a message.
//! ln23 2022-01-02 06:00:00,0 [info] module6  Message Subject: Text for a message.
//! ```
//!
//! In addition to the `--any` and `--all` evaluation strategies, pass `--latest` to filter for the latest results.
//!
//! ```console
//! $ log-cli sample.log --keywords "[debug]" "[info]" --any --latest 3
//!
//! File (1/1): sample.log
//! ln18 2022-01-02 01:00:00,0 [info] module10  Message Subject: Text for a message.
//! ln19 2022-01-02 02:00:00,0 [info] module1  Message Subject: Text for a message.
//! ln20 2022-01-02 03:00:00,0 [debug] module12  Message Subject: Text for a message.
//! ```
//!
//! By default `--latest` will return the latest filtered line found.
//!
//! ## Help
//!
//! ```console
//! $ log-cli --help
//!
//! Command line interface for log files.
//!
//! USAGE:
//!     log-cli [OPTIONS] <LOG_FILE>...
//!
//! ARGS:
//!     <LOG_FILE>...
//!             Path to log file to be read. By default if no additional flags are passed the entire
//!             file will be
//!             displayed.
//!
//! OPTIONS:
//!         --all
//!             Set evaluation strategy to 'all'.
//!
//!         --any
//!             Set evaluation strategy to 'any'.
//!
//!     -h, --help
//!             Prints help information. Use --help for more details.
//!
//!         --head <VALUE>
//!             Display the top VALUE lines.
//!
//!         --keywords <VALUE>...
//!             Keywords to search for in the log file. Multiple keywords can be passed (ex: these are
//!             all
//!             keywords).
//!
//!         --latest <VALUE>
//!             Set evaluation strategy to 'latest' VALUE lines.
//!
//!         --line-range <VALUE>...
//!             Line number range to display. Must be a valid integer range format (ex: 0 10 to display
//!             lines 0
//!             through 10).
//!
//!         --tail <VALUE>
//!             Display the bottom VALUE lines.
//!
//!     -V, --version
//!          Print version information
//!    ```
use args::Args;
use std::{
    error,
    process::{ExitCode, Termination},
};

/// `log-cli` application code. This is a command line application, so the application is defined
/// by its exposed arguments. These arguments under the hood are clap arguments. The application
/// code composes clap logic inside log cli control flow.
///
/// This module implements the clap arguments available to the user, some light validation, and
/// value parsing.
mod app;
/// Argument implementations bridging clap inputs to `log-cli` components.
mod args;
/// Buffer operations for IO.
mod buffer;
/// Data structures for file data.
mod lines;
/// Query implementations for data operations.
mod query;

// Generic result type for all errors.
type Result<T> = ::std::result::Result<T, Box<dyn error::Error>>;

#[repr(u8)]
enum MainResult {
    /// Process exit without issue.
    Success = 0,
    /// Process exit due to invalid behavior.
    Invalid = 2,
}

impl Termination for MainResult {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

fn main() -> MainResult {
    // Exit with invalid behavior code if args fail to parse.
    if let Err(e) = Args::parse().and_then(run) {
        eprintln!("{}", e);
        return MainResult::Invalid;
    }

    MainResult::Success
}

/// Determine root behavior selected.
fn run(args: Args) -> Result<()> {
    use args::Root::*;

    match args.root()? {
        Query => run_query(args),
    }
}

/// Execute the main query behavior.
fn run_query(args: Args) -> Result<()> {
    use query::Query;

    Query::build(args.to_instructions()?)?.execute()
}
