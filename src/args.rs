use std::path::PathBuf;

use crate::app;
use crate::query;
use crate::Result;

/// Possible root arguments available to the user.
pub(crate) enum Root {
    /// Query a log file using query instructions.
    Query,
}

/// Args wrapper around clap data.
pub(crate) struct Args(clap::ArgMatches);

impl Args {
    /// Get matches from clap and wrap in `Args` struct.
    pub(crate) fn parse() -> Result<Args> {
        let matches = app::app().get_matches();

        if !matches.contains_id("paths") {
            return Err(From::from("no paths were found"));
        }

        Ok(Args(matches))
    }

    /// Get the root argument from `Args`.
    pub(crate) fn root(&self) -> Result<Root> {
        Ok(Root::Query)
    }

    // Access to clap matches.
    fn matches(&self) -> &clap::ArgMatches {
        &self.0
    }

    /// Create query instructions from `Args`.
    pub(crate) fn to_instructions(&self) -> Result<query::Instructions> {
        let mut instructions = query::Instructions::new();

        // Add paths found to instructions.
        if let Some(paths) = self.matches().get_many::<PathBuf>("paths") {
            for path in paths {
                instructions = instructions.add_path(path.to_owned());
            }
        }

        // Add keywords found to instructions.
        if let Some(keywords) = self.matches().get_many::<String>("keywords") {
            for word in keywords {
                instructions = instructions.add_keyword(word.to_owned());
            }
        }

        // Add head range to instructions if found.
        if let Some(n) = self.matches().get_one::<usize>("head") {
            instructions = instructions.add_relative_range("head", *n);
        }

        // Add tail range to instructions if found.
        if let Some(n) = self.matches().get_one::<usize>("tail") {
            instructions = instructions.add_relative_range("tail", *n);
        }

        // Add line range to instructions if found.
        if let Some(range) = self.matches().get_many::<usize>("line-range") {
            let mut vals = range;
            let lower = vals.next();
            if lower.is_none() {
                return Err(From::from("at least one value is required for line-range"));
            }
            let upper = vals.next().unwrap_or_else(|| lower.unwrap());
            instructions = instructions.add_range("line-range", *lower.unwrap(), *upper);
        }

        // Add all eval to instructions if found.
        if let Some(eval) = self.matches().get_one::<bool>("all") {
            if *eval {
                instructions = instructions.add_eval("all", None);
            }
        }

        // Add any eval to instructions if found.
        if let Some(eval) = self.matches().get_one::<bool>("any") {
            if *eval {
                instructions = instructions.add_eval("any", None);
            }
        }

        // Add latest eval to instructions if found.
        if let Some(eval) = self.matches().get_one::<usize>("latest") {
            instructions = instructions.add_eval("latest", Some(*eval));
        }

        Ok(instructions)
    }
}
