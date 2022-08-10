/// Parse line range array from a string.
// TODO:
// - Use Result.
// - Data validation.
// - Less object redundancy.
pub fn string_to_line_range_array(string: Option<&String>) -> Option<[u32; 2]> {
    string?;

    let parts: Vec<u32> = string?
        .split(':')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    Some([parts[0], parts[1]])
}

/// Parse date range array from a string.
// TODO:
// - Use Result.
// - Data validation.
// - Less object redundancy.
pub fn string_to_date_range_array(string: Option<&String>) -> Option<[String; 2]> {
    string?;

    let parts: Vec<String> = string?.split("::").map(|s| s.to_string()).collect();

    Some([parts[0].clone(), parts[1].clone()])
}

/// Parse function string from string.
// TODO:
// - Use Result.
// - Validation.
// - Less object redundancy.
pub fn string_to_function_string(string: Option<&String>) -> Option<String> {
    string?;

    // TODO
    Some(string?.clone())
}

/// Parse filepath from string.
// TODO:
// - Use Result.
// - Path validation.
// - Use Path.
pub fn string_to_filepath_string(string: &str) -> String {
    // TODO
    string.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_line_range_array() {
        let string = "0:100".to_string();
        let expected: [u32; 2] = [0, 100];
        let res = string_to_line_range_array(Some(&string)).unwrap();

        assert_eq!(res.len(), 2);
        assert_eq!(res, expected);
        assert_eq!(string_to_line_range_array(None), None);
    }

    #[test]
    fn test_string_to_date_range_array() {
        let string = "2022-01-01 08:00::2022-01-02 08:00".to_string();
        let expected: [String; 2] = [
            "2022-01-01 08:00".to_string(),
            "2022-01-02 08:00".to_string(),
        ];
        let res = string_to_date_range_array(Some(&string)).unwrap();

        assert_eq!(res.len(), 2);
        assert_eq!(res, expected);
        assert_eq!(string_to_date_range_array(None), None);
    }

    #[test]
    fn test_string_to_function_string() {
        let string = "my_function()".to_string();
        let expected = "my_function()".to_string();
        let res = string_to_function_string(Some(&string)).unwrap();

        assert_eq!(res, expected);
        assert_eq!(string_to_function_string(None), None);
    }

    #[test]
    fn test_string_to_filepath_string() {
        let string = "sample.log".to_string();
        let expected = "sample.log".to_string();
        let res = string_to_filepath_string(&string);

        assert_eq!(res, expected);
    }
}
