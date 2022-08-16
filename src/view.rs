use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use clap::parser::ValuesRef;

/// Viewer struct used to perform view operations on file buffers.
#[derive(Default)]
pub struct Viewer {
    keywords: Option<Vec<String>>,
    line_range: Option<Vec<usize>>,
    #[allow(dead_code)]
    date_range: Option<Vec<String>>,
    head: Option<usize>,
    #[allow(dead_code)]
    all: Option<bool>,
    any: Option<bool>,
}

impl Viewer {
    pub fn new(
        keywords: Option<ValuesRef<'_, String>>,
        line_range: Option<ValuesRef<'_, usize>>,
        date_range: Option<ValuesRef<'_, String>>,
        head: Option<&usize>,
        all: Option<&bool>,
        any: Option<&bool>,
    ) -> Viewer {
        let mut _keywords = None;
        let mut _line_range = None;
        let mut _date_range = None;
        let mut _head = None;
        let mut _all = None;
        let mut _any = None;

        if let Some(v) = keywords {
            _keywords = Some(v.into_iter().cloned().collect());
        }
        if let Some(v) = line_range {
            _line_range = Some(v.into_iter().cloned().collect());
        }
        if let Some(v) = date_range {
            _date_range = Some(v.into_iter().cloned().collect());
        }
        if let Some(v) = head {
            _head = Some(*v);
        }
        if let Some(v) = all {
            _all = Some(*v);
        }
        if let Some(v) = any {
            _any = Some(*v);
        }

        Viewer {
            keywords: _keywords,
            line_range: _line_range,
            date_range: _date_range,
            head: _head,
            all: _all,
            any: _any,
        }
    }

    /// Filter enumerated lines for line numbers selected by the line range.
    fn filter_with_line_range(
        &self,
        lines: &[(usize, String)],
        range: &Vec<usize>,
    ) -> Result<Vec<(usize, String)>, &str> {
        // The range given is invalid if it has more than two values.
        // TODO: Should this be a panic?
        if range.len() > 2 {
            return Err("The range provided has more than two elements.");
        }

        let res = lines
            .iter()
            .filter(|(i, _)| {
                // If line range is only one value skip ln if it's not the selected ln.
                if range.len() == 1 && *i != range[0] {
                    return false;
                }

                // If line range is two values then skip ln if it's outside the range selected.
                if range.len() == 2 && (*i < range[0] || *i > range[1]) {
                    return false;
                }

                true
            })
            .into_iter()
            .cloned()
            .collect();

        Ok(res)
    }

    /// Filter enumerated lines containing keywords depending on a desired evaulation strategy (eval).
    /// An eval can be "all" or "any".
    fn filter_with_keywords(
        &self,
        lines: &[(usize, String)],
        keywords: &[String],
        eval: &str, // TODO: Use clap-recognizable enum
    ) -> Result<Vec<(usize, String)>, &str> {
        // Filter lines for lines that contain any of the keywords indicated by caller.
        let res = lines
            .iter()
            .filter(|(_, l)| vec_elements_in_string(keywords, l, eval))
            .into_iter()
            .cloned()
            .collect();

        Ok(res)
    }

    /// Display the entire file.
    fn display_lines(&self, lines: &[(usize, String)]) -> Result<(), &str> {
        // If no lines are collected correctly display nothing.
        // TODO: Maybe panic.
        if lines.is_empty() {
            return Ok(());
        }

        // Last line number (assumed sorted ascending) determines line number padding.
        let last_ln = lines[lines.len() - 1].0;

        // Display each line numbered with padding based on the number of lines collected.
        // TODO: Could use generic binary search fn to calculate digits without conversion.
        for (i, line) in lines {
            println!(
                "ln{:0width$} {}",
                i,
                line,
                width = last_ln.to_string().len()
            );
        }

        Ok(())
    }

    /// Display with viewer function to display the file via its `BufReader`.
    // TODO:
    //       - Validation and error handling.
    //       - Use Enum eval.
    pub fn display_with(&self, buffer: &mut BufReader<File>) -> Result<(), &str> {
        // Collect enumerated lines.
        let mut lines: Vec<(usize, String)> = buffer.lines().flatten().enumerate().collect();
        // TODO: Enum.
        let mut eval = "all";

        // Update eval if 'any' provided.
        if let Some(any) = self.any {
            if any {
                eval = "any"
            }
        }

        // Return head view if one is provided.
        if let Some(head) = &self.head {
            let range = vec![0, head - 1];
            lines = self.filter_with_line_range(&lines, &range)?;
        }

        // Filter using ranges if provided.
        if let Some(range) = &self.line_range {
            lines = self.filter_with_line_range(&lines, range)?;
        }

        if let Some(keywords) = &self.keywords {
            lines = self.filter_with_keywords(&lines, keywords, eval)?;
        }

        self.display_lines(&lines)
    }
}

/// Check string for "any" or "all" (eval) elements from vector. Return true of eval is met, otherwise return false.
// TODO: Use Result and eval Enum.
fn vec_elements_in_string(vec: &[String], string: &str, eval: &str) -> bool {
    if eval == "all" {
        return vec.iter().all(|e| string.contains(e));
    }

    if eval == "any" {
        return vec.iter().any(|e| string.contains(e));
    }

    false
}

#[test]
fn test_viewer_none() {
    let viewer = Viewer::default();

    assert_eq!(viewer.line_range, None);
    assert_eq!(viewer.date_range, None);
    assert_eq!(viewer.keywords, None);
}

#[test]
fn test_viewer_keywords() {
    let viewer = Viewer::default();
    let lines = vec![
        (0, "first".to_string()),
        (1, "second".to_string()),
        (2, "foo".to_string()),
    ];
    let keywords = vec!["foo".to_string()];

    if let Ok(res) = viewer.filter_with_keywords(&lines, &keywords, "all") {
        assert_eq!(res, vec![(2, "foo".to_string())]);
        return;
    } else {
        // Fail if result didn't return Ok.
        assert!(false);
    }
}

#[test]
fn test_viewer_line_range() {
    let viewer = Viewer::default();
    let lines = vec![
        (0, "first".to_string()),
        (1, "second".to_string()),
        (2, "third".to_string()),
    ];
    let range1 = vec![0];
    let range2 = vec![1, 2];

    // Test with one value.
    if let Ok(res) = viewer.filter_with_line_range(&lines, &range1) {
        assert_eq!(res, vec![(0, "first".to_string())]);
    } else {
        // Fail if result didn't return Ok.
        assert!(false);
    }

    // Test with two values.
    if let Ok(res) = viewer.filter_with_line_range(&lines, &range2) {
        assert_eq!(
            res,
            vec![(1, "second".to_string()), (2, "third".to_string())]
        );
    } else {
        // Fail if result didn't return Ok.
        assert!(false);
    }
}
