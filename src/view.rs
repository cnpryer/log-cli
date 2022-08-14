use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::parser::ValuesRef;

/// Viewer struct used to perform view operations on file buffers.
pub struct Viewer {
    keywords: Option<Vec<String>>,
    line_range: Option<Vec<usize>>,
    date_range: Option<Vec<String>>,
    head: Option<usize>,
}

impl Viewer {
    // TODO: Certain parameters cannot be used together.
    pub fn new(
        keywords: Option<ValuesRef<'_, String>>,
        line_range: Option<ValuesRef<'_, usize>>,
        date_range: Option<ValuesRef<'_, String>>,
        head: Option<&usize>,
    ) -> Viewer {
        match (keywords, line_range, date_range, head) {
            (Some(kw), Some(lr), None, None) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: Some(lr.into_iter().copied().collect()),
                date_range: None,
                head: None,
            },
            (Some(kw), None, Some(dr), None) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: None,
                date_range: Some(dr.into_iter().cloned().collect()),
                head: None,
            },
            (Some(kw), None, None, None) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: None,
                date_range: None,
                head: None,
            },
            (None, None, None, None) => Viewer {
                keywords: None,
                line_range: None,
                date_range: None,
                head: None,
            },
            (None, None, Some(dr), None) => Viewer {
                keywords: None,
                line_range: None,
                date_range: Some(dr.into_iter().cloned().collect()),
                head: None,
            },
            (None, Some(lr), None, None) => Viewer {
                keywords: None,
                line_range: Some(lr.into_iter().copied().collect()),
                date_range: None,
                head: None,
            },
            (None, None, None, Some(h)) => Viewer {
                keywords: None,
                line_range: None,
                date_range: None,
                head: Some(*h),
            },
            (None, None, Some(dr), Some(h)) => Viewer {
                keywords: None,
                line_range: None,
                date_range: Some(dr.into_iter().cloned().collect()),
                head: Some(*h),
            },
            (None, Some(lr), None, Some(h)) => Viewer {
                keywords: None,
                line_range: Some(lr.into_iter().copied().collect()),
                date_range: None,
                head: Some(*h),
            },
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

    /// Display all lines within date range (inlcusive).
    fn _display_with_dates(&self, _buffer: &mut BufReader<File>) -> Result<(), &str> {
        if self.date_range.is_none() {
            return Err("No date range found.");
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
        for (i, line) in iter {
            println!("ln{} {}", i, line);
        }

        Ok(())
    }

    /// Display with viewer function to display the file via its `BufReader`.
    // TODO:
    //       - Use `Result`
    //       - Validation and error handling.
    pub fn display_with(&self, buffer: &mut BufReader<File>) -> Result<(), &str> {
        match (
            self.keywords.as_ref(),
            self.line_range.as_ref(),
            self.date_range.as_ref(),
            self.head.as_ref(),
        ) {
            (Some(kw), Some(lr), None, None) => {
                let lines =
                    self.filter_with_line_range(buffer.lines().flatten().enumerate(), lr)?;
                let lines = self.filter_with_keywords(lines, kw)?;
                self.display_lines(lines)
            }
            (None, None, None, None) => self.display_lines(buffer.lines().flatten().enumerate()),
            (None, None, Some(_), None) => unimplemented!(),
            (None, Some(lr), None, None) => {
                let lines =
                    self.filter_with_line_range(buffer.lines().flatten().enumerate(), lr)?;
                self.display_lines(lines)
            }
            (Some(kw), None, None, None) => {
                let lines = self.filter_with_keywords(buffer.lines().flatten().enumerate(), kw)?;
                self.display_lines(lines)
            }
            (Some(_), None, Some(_), None) => unimplemented!(),
            (None, None, None, Some(h)) => {
                let lr = vec![0, *h - 1];
                let lines =
                    self.filter_with_line_range(buffer.lines().flatten().enumerate(), &lr)?;
                self.display_lines(lines)
            }
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
// TODO: More robust testing.
fn test_viewer() {
    let viewer_none = Viewer::new(None, None, None, None);

    assert_eq!(viewer_none.line_range, None);
    assert_eq!(viewer_none.date_range, None);
    assert_eq!(viewer_none.keywords, None);
}
