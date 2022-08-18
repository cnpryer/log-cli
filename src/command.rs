use clap::parser::ValuesRef;

/// Range selection command group struct.
#[derive(Default)]
pub struct RangeSelectionData {
    pub(crate) line_range: Option<Vec<usize>>,
    pub(crate) date_range: Option<Vec<String>>,
    pub(crate) head: Option<usize>,
    pub(crate) tail: Option<usize>,
}

impl RangeSelectionData {
    pub fn new(
        line_range: Option<ValuesRef<'_, usize>>,
        date_range: Option<ValuesRef<'_, String>>,
        head: Option<&usize>,
        tail: Option<&usize>,
    ) -> RangeSelectionData {
        let mut _line_range = None;
        let mut _date_range = None;
        let mut _head = None;
        let mut _tail = None;

        if let Some(v) = line_range {
            _line_range = Some(v.into_iter().cloned().collect());
        }

        if let Some(v) = date_range {
            _date_range = Some(v.into_iter().cloned().collect());
        }

        if let Some(v) = head {
            _head = Some(*v);
        }

        if let Some(v) = tail {
            _tail = Some(*v);
        }

        let ranges = RangeSelectionData {
            line_range: _line_range,
            date_range: _date_range,
            head: _head,
            tail: _tail,
        };

        if let Err(msg) = validate_range_selection_combinations(&ranges) {
            panic!("{:?}", msg);
        }

        ranges
    }
}

/// Validate that range selection combinations are compatible, otherwise return Err.
pub(crate) fn validate_range_selection_combinations(
    ranges: &RangeSelectionData,
) -> Result<(), &str> {
    match (
        &ranges.line_range,
        &ranges.date_range,
        &ranges.head,
        &ranges.tail,
    ) {
        // Can't use both line range and date range.
        (Some(_), Some(_), None, None) => Err("Cannot use both line range and date range."),
        // Can't use both line range and head.
        (Some(_), None, Some(_), None) => Err("Cannot use both line range and head."),
        // Can't use both date range and head.
        (None, Some(_), Some(_), None) => Err("Cannot use both date range and head."),
        // Can't use both line range and tail.
        (Some(_), None, None, Some(_)) => Err("Cannot use both line range and tail."),
        // Can't use both date range and tail
        (None, Some(_), None, Some(_)) => Err("Cannot use both date range and tail."),
        // Can't use both head and tail.
        (None, None, Some(_), Some(_)) => Err("Cannot use both head and tail."),
        _ => Ok(()),
    }
}

/// Evaluation data group struct.
#[derive(Default)]
pub struct EvaluationStrategyData {
    pub(crate) all: Option<bool>,
    pub(crate) any: Option<bool>,
    #[allow(dead_code)]
    pub(crate) latest: Option<usize>,
}

impl EvaluationStrategyData {
    pub fn new(
        all: Option<&bool>,
        any: Option<&bool>,
        latest: Option<&usize>,
    ) -> EvaluationStrategyData {
        let mut _all = None;
        let mut _any = None;
        let mut _latest = None;

        if let Some(v) = all {
            _all = Some(*v);
        }

        if let Some(v) = any {
            _any = Some(*v);
        }

        if let Some(v) = latest {
            _latest = Some(*v);
        }

        let evals = EvaluationStrategyData {
            all: _all,
            any: _any,
            latest: _latest,
        };

        if let Err(msg) = validate_evaluation_strategy_combinations(&evals) {
            panic!("{:?}", msg);
        }

        evals
    }
}

/// Validate that evaluation strategy combinations are compatible, otherwise return Err.
pub(crate) fn validate_evaluation_strategy_combinations(
    evals: &EvaluationStrategyData,
) -> Result<(), &str> {
    match (&evals.all, &evals.any) {
        // Can't use both all and any.
        (Some(_all), Some(_any)) => {
            if *_all && *_any {
                Err("Cannot use both all and any.")
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

// TODO: For tests expected to panic, maybe there's a more elegant way to catch the message to ensure it's the expected
//       message. The failure shouldn't be recoverable since it's the implementation that should handle combinations.
#[cfg(test)]
mod tests {
    use super::*;
    use clap::{arg, value_parser, ArgAction};

    // TODO: Remove this restriction since it's okay to use both.
    #[test]
    #[should_panic]
    fn test_line_range_and_date_range() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--"line-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::parse::parse_line_range_value)
                    .multiple_values(true),
            )
            .arg(
                arg!(--"date-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::parse::parse_date_range_value)
                    .multiple_values(true),
            );

        let matches = cmd
            .try_get_matches_from([
                "test",
                "--line-range",
                "0",
                "1",
                "--date-range",
                "2022-01-01",
            ])
            .unwrap();

        // Can't use both line and date range.
        let _ = RangeSelectionData::new(
            matches.get_many::<usize>("line-range"),
            matches.get_many::<String>("date-range"),
            None,
            None,
        );
    }

    #[test]
    #[should_panic]
    fn test_line_range_and_head() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--"line-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::parse::parse_line_range_value)
                    .multiple_values(true),
            )
            .arg(
                arg!(--head <VALUE>)
                    .default_missing_value("5")
                    .required(false)
                    .value_parser(value_parser!(usize)),
            );

        let matches = cmd
            .try_get_matches_from(["test", "--line-range", "0", "1", "--head", "5"])
            .unwrap();

        // Can't use both line range and head.
        let _ = RangeSelectionData::new(
            matches.get_many::<usize>("line-range"),
            None,
            matches.get_one::<usize>("head"),
            None,
        );
    }

    #[test]
    #[should_panic]
    fn test_date_range_and_head() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--"date-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::parse::parse_date_range_value)
                    .multiple_values(true),
            )
            .arg(
                arg!(--head <VALUE>)
                    .default_missing_value("5")
                    .required(false)
                    .value_parser(value_parser!(usize)),
            );

        let matches = cmd
            .try_get_matches_from(["test", "--date-range", "2022-01-01", "--head", "5"])
            .unwrap();

        // Can't use both date range and head.
        let _ = RangeSelectionData::new(
            None,
            matches.get_many::<String>("date-range"),
            matches.get_one::<usize>("head"),
            None,
        );
    }

    #[test]
    #[should_panic]
    fn test_line_range_and_tail() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--"line-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::parse::parse_line_range_value)
                    .multiple_values(true),
            )
            .arg(
                arg!(--tail <VALUE>)
                    .default_missing_value("5")
                    .required(false)
                    .value_parser(value_parser!(usize)),
            );

        let matches = cmd
            .try_get_matches_from(["test", "--line-range", "0", "1", "--tail", "5"])
            .unwrap();

        // Can't use both date range and head.
        let _ = RangeSelectionData::new(
            matches.get_many::<usize>("line-range"),
            None,
            None,
            matches.get_one::<usize>("tail"),
        );
    }
    #[test]
    #[should_panic]
    fn test_date_range_and_tail() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--"date-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::parse::parse_date_range_value)
                    .multiple_values(true),
            )
            .arg(
                arg!(--tail <VALUE>)
                    .default_missing_value("5")
                    .required(false)
                    .value_parser(value_parser!(usize)),
            );

        let matches = cmd
            .try_get_matches_from(["test", "--date-range", "2022-01-01", "--tail", "5"])
            .unwrap();

        // Can't use both date range and head.
        let _ = RangeSelectionData::new(
            None,
            matches.get_many::<String>("date-range"),
            None,
            matches.get_one::<usize>("tail"),
        );
    }

    #[test]
    #[should_panic]
    fn test_head_and_tail() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--head <VALUE>)
                    .default_missing_value("5")
                    .required(false)
                    .value_parser(value_parser!(usize)),
            )
            .arg(
                arg!(--tail <VALUE>)
                    .default_missing_value("5")
                    .required(false)
                    .value_parser(value_parser!(usize)),
            );

        let matches = cmd
            .try_get_matches_from(["test", "--head", "5", "--tail", "5"])
            .unwrap();

        // Can't use both date range and head.
        let _ = RangeSelectionData::new(
            None,
            None,
            matches.get_one::<usize>("head"),
            matches.get_one::<usize>("tail"),
        );
    }

    #[test]
    #[should_panic]
    fn test_all_and_any() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--all)
                    .takes_value(false)
                    .required(false)
                    .action(ArgAction::SetTrue),
            )
            .arg(
                arg!(--any)
                    .takes_value(false)
                    .required(false)
                    .action(ArgAction::SetTrue),
            );

        let matches = cmd
            .try_get_matches_from(["test", "--all", "--any"])
            .unwrap();

        // Can't use both date range and head.
        let _ = EvaluationStrategyData::new(
            matches.get_one::<bool>("all"),
            matches.get_one::<bool>("any"),
            None,
        );
    }
}
