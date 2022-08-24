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
