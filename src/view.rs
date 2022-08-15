use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use clap::parser::ValuesRef;

/// Viewer struct used to perform view operations on file buffers and line iterators.
#[derive(Default)]
pub struct Viewer {
    keywords: Option<Vec<String>>,
    line_range: Option<Vec<usize>>,
    date_range: Option<Vec<String>>,
    head: Option<usize>,
}

impl Viewer {
    pub fn new(
        keywords: Option<ValuesRef<'_, String>>,
        line_range: Option<ValuesRef<'_, usize>>,
        date_range: Option<ValuesRef<'_, String>>,
        head: Option<&usize>,
    ) -> Viewer {
        match (keywords, line_range, date_range, head) {
            // Keywords and line range.
            (Some(kw), Some(lr), None, None) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: Some(lr.into_iter().copied().collect()),
                date_range: None,
                head: None,
            },
            // Keywords and date range.
            (Some(kw), None, Some(dr), None) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: None,
                date_range: Some(dr.into_iter().cloned().collect()),
                head: None,
            },
            // Keywords.
            (Some(kw), None, None, None) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: None,
                date_range: None,
                head: None,
            },
            // Nothing provided.
            (None, None, None, None) => Viewer {
                keywords: None,
                line_range: None,
                date_range: None,
                head: None,
            },
            // Date range.
            (None, None, Some(dr), None) => Viewer {
                keywords: None,
                line_range: None,
                date_range: Some(dr.into_iter().cloned().collect()),
                head: None,
            },
            // Line range.
            (None, Some(lr), None, None) => Viewer {
                keywords: None,
                line_range: Some(lr.into_iter().copied().collect()),
                date_range: None,
                head: None,
            },
            // Head.
            (None, None, None, Some(h)) => Viewer {
                keywords: None,
                line_range: None,
                date_range: None,
                head: Some(*h),
            },
            // Date range and head.
            (None, None, Some(dr), Some(h)) => Viewer {
                keywords: None,
                line_range: None,
                date_range: Some(dr.into_iter().cloned().collect()),
                head: Some(*h),
            },
            // Line range and head.
            (None, Some(lr), None, Some(h)) => Viewer {
                keywords: None,
                line_range: Some(lr.into_iter().copied().collect()),
                date_range: None,
                head: Some(*h),
            },
            // Keywords and head.
            (Some(kw), None, None, Some(h)) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: None,
                date_range: None,
                head: Some(*h),
            },
            _ => unreachable!(),
        }
    }

    /// Filter an iterator of String lines for line numbers selected by the line range.
    fn filter_with_line_range<I>(
        &self,
        iter: I,
        range: &Vec<usize>,
    ) -> Result<impl Iterator<Item = (usize, String)>, &str>
    where
        I: Iterator<Item = (usize, String)>,
    {
        // The range given is invalid if it has more than two values.
        // TODO: Should this be a panic?
        if range.len() > 2 {
            return Err("The range provided has more than two elements.");
        }

        let res = iter
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
            .collect::<Vec<(usize, String)>>()
            .into_iter();

        Ok(res)
    }

    /// Filter an iterator of String lines for dates selected by the date range.
    fn _filter_with_date_range<I>(&self, _iter: I, range: &Vec<usize>) -> Result<(), &str>
    where
        I: Iterator<Item = (usize, String)>,
    {
        // The range given is invalid if it has more than two values.
        // TODO: Should this be a panic?
        if range.len() > 2 {
            return Err("The range provided has more than two elements.");
        }

        // for line in buffer.lines().flatten() {
        //     println!("{}", line);
        // }
        unimplemented!()
    }

    /// Filter iterator of String lines for lines containing any keywords selected.
    fn filter_with_keywords<I>(
        &self,
        iter: I,
        keywords: &[String],
    ) -> Result<impl Iterator<Item = (usize, String)>, &str>
    where
        I: Iterator<Item = (usize, String)>,
    {
        // Filter lines for lines that contain any of the keywords indicated by caller.
        let res = iter
            .filter(|(_, l)| keywords.iter().any(|kw| l.contains(kw)))
            .collect::<Vec<(usize, String)>>()
            .into_iter();

        Ok(res)
    }

    /// Display the entire file.
    fn display_lines<I>(&self, iter: I) -> Result<(), &str>
    where
        I: Iterator<Item = (usize, String)>,
    {
        // TODO: Store either metadata from initial file read or store upfront after intially collecting
        //       or storing the iterator.
        let lines: Vec<(usize, String)> = iter.collect();
        let max_ln = &lines.len() - 1;

        // Display each line numbered with padding based on the number of lines collected.
        // TODO: Could use generic binary search fn to calculate digits without conversion.
        for (i, line) in &lines {
            println!("ln{:0width$} {}", i, line, width = max_ln.to_string().len());
        }

        Ok(())
    }

    /// Display with viewer function to display the file via its `BufReader`.
    // TODO:
    //       - Validation and error handling.
    pub fn display_with(&self, buffer: &mut BufReader<File>) -> Result<(), &str> {
        match (
            self.keywords.as_ref(),
            self.line_range.as_ref(),
            self.date_range.as_ref(),
            self.head.as_ref(),
        ) {
            // Keywords and line range.
            (Some(kw), Some(lr), None, None) => {
                let lines =
                    self.filter_with_line_range(buffer.lines().flatten().enumerate(), lr)?;
                let lines = self.filter_with_keywords(lines, kw)?;
                self.display_lines(lines)
            }
            // Nothing provided.
            (None, None, None, None) => self.display_lines(buffer.lines().flatten().enumerate()),
            // Date range.
            (None, None, Some(_), None) => unimplemented!(),
            // Line range.
            (None, Some(lr), None, None) => {
                let lines =
                    self.filter_with_line_range(buffer.lines().flatten().enumerate(), lr)?;
                self.display_lines(lines)
            }
            // Keywords.
            (Some(kw), None, None, None) => {
                let lines = self.filter_with_keywords(buffer.lines().flatten().enumerate(), kw)?;
                self.display_lines(lines)
            }
            // Keywords and date range.
            (Some(_), None, Some(_), None) => unimplemented!(),
            // Head.
            (None, None, None, Some(h)) => {
                let lr = vec![0, *h - 1];
                let lines =
                    self.filter_with_line_range(buffer.lines().flatten().enumerate(), &lr)?;
                self.display_lines(lines)
            }
            // Keywords and head.
            (Some(kw), None, None, Some(h)) => {
                let lr = vec![0, *h - 1];
                let lines =
                    self.filter_with_line_range(buffer.lines().flatten().enumerate(), &lr)?;
                let lines = self.filter_with_keywords(lines, kw)?;
                self.display_lines(lines)
            }
            _ => unreachable!(),
        }
    }
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
    let iter = vec!["first".to_string(), "second".to_string(), "foo".to_string()]
        .into_iter()
        .enumerate();
    let keywords = vec!["foo".to_string()];

    if let Ok(res) = viewer.filter_with_keywords(iter, &keywords) {
        assert_eq!(
            res.collect::<Vec<(usize, String)>>(),
            vec![(2, "foo".to_string())]
        );
        return;
    } else {
        // Fail if result didn't return Ok.
        assert!(false);
    }
}

#[test]
fn test_viewer_line_range() {
    let viewer = Viewer::default();
    let iter1 = vec![
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    ]
    .into_iter()
    .enumerate();
    let iter2 = vec![
        "fourth".to_string(),
        "fifth".to_string(),
        "sixth".to_string(),
    ]
    .into_iter()
    .enumerate();
    let range1 = vec![0];
    let range2 = vec![1, 2];

    // Test with one value.
    if let Ok(res) = viewer.filter_with_line_range(iter1, &range1) {
        assert_eq!(
            res.collect::<Vec<(usize, String)>>(),
            vec![(0, "first".to_string())]
        );
    } else {
        // Fail if result didn't return Ok.
        assert!(false);
    }

    // Test with two values.
    if let Ok(res) = viewer.filter_with_line_range(iter2, &range2) {
        assert_eq!(
            res.collect::<Vec<(usize, String)>>(),
            vec![(1, "fifth".to_string()), (2, "sixth".to_string())]
        );
    } else {
        // Fail if result didn't return Ok.
        assert!(false);
    }
}
