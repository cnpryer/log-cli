use chrono::NaiveDate;

/// Required date format.
const DATE_FORMAT: &str = "%Y-%m-%d";

/// Parse line range argument value(s). Return vector of at most integers or error.
/// A valid line range can be either one line number (1 for line one) or two line numbers to indicate
/// multiple lines (0 20 to get first 20 lines).
pub fn parse_line_range_value(value: &str) -> Result<usize, String> {
    // Attempt to parse u32 values from string values.
    let res: usize = value
        .parse()
        .map_err(|_| format!("{} must be a valid usize.", value))?;

    Ok(res)
}

/// Validate that a string is date-like.
// NOTE: Currently strings are utilized internally. Evenaully `chrono` datetimes could be used, but
//       for now we only use `chrono` for validation purposes.
// TODO: Operate with `chrono` dates.
fn is_date_like(value: &str) -> Result<bool, String> {
    // Attempt to parse string as date.
    let res = NaiveDate::parse_from_str(value, DATE_FORMAT);

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
pub fn parse_date_range_value(value: &str) -> Result<String, String> {
    // Check if value passed is a valid date formatted string.
    if let Ok(false) = is_date_like(value) {
        return Err(format!("Date format must be {}.", DATE_FORMAT));
    }

    Ok(value.to_string())
}

#[test]
fn test_parse_line_range() {
    let valid_value = "0";
    let invalid_value = "foo";

    assert_eq!(parse_line_range_value(valid_value), Ok(0));
    assert_eq!(
        parse_line_range_value(invalid_value),
        Err(format!("{} must be a valid usize.", invalid_value))
    );
}

#[test]
fn test_parse_date_range() {
    let valid_value = "2022-01-01";
    let invalid_value = "foo";

    assert_eq!(
        parse_date_range_value(valid_value),
        Ok("2022-01-01".to_string())
    );
    assert_eq!(
        parse_date_range_value(invalid_value),
        Err(format!("Date format must be {}.", DATE_FORMAT))
    );
}
