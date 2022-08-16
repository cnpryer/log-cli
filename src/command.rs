use clap::parser::ValuesRef;

// Range selection command group struct.
#[derive(Default)]
pub struct RangeSelectionData {
    pub(crate) line_range: Option<Vec<usize>>,
    pub(crate) date_range: Option<Vec<String>>,
    pub(crate) head: Option<usize>,
}

impl RangeSelectionData {
    pub fn new(
        line_range: Option<ValuesRef<'_, usize>>,
        date_range: Option<ValuesRef<'_, String>>,
        head: Option<&usize>,
    ) -> RangeSelectionData {
        let mut _line_range = None;
        let mut _date_range = None;
        let mut _head = None;

        // Handle invalid combinations and Option parsing.
        match (&line_range, &date_range, &head) {
            // Can't use both line range and date range.
            (Some(_), Some(_), None) => {
                panic!("Cannot use both line range and date range.")
            }
            // Can't use both line range and head.
            (Some(_), None, Some(_)) => {
                panic!("Cannot use both line range and head.")
            }
            // Can't use both date range and head.
            (None, Some(_), Some(_)) => {
                panic!("Cannot use both date range and head.")
            }
            _ => {
                if let Some(v) = line_range {
                    _line_range = Some(v.into_iter().cloned().collect());
                }

                if let Some(v) = date_range {
                    _date_range = Some(v.into_iter().cloned().collect());
                }

                if let Some(v) = head {
                    _head = Some(*v);
                }
            }
        }

        RangeSelectionData {
            line_range: _line_range,
            date_range: _date_range,
            head: _head,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{arg, value_parser};

    #[test]
    #[should_panic]
    fn test_line_range_and_date_range() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--"line-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::validate::valid_line_range_value)
                    .multiple_values(true),
            )
            .arg(
                arg!(--"date-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::validate::valid_date_range_value)
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
        );
    }

    #[test]
    #[should_panic]
    fn test_line_range_and_head() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--"line-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::validate::valid_line_range_value)
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
        );
    }

    #[test]
    #[should_panic]
    fn test_date_range_and_head() {
        let cmd = clap::Command::new("test")
            .arg(
                arg!(--"date-range" <VALUE>)
                    .required(false)
                    .value_parser(crate::validate::valid_date_range_value)
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
        );
    }
}
