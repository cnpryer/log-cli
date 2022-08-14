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
}

impl Viewer {
    // TODO: Certain parameters cannot be used together.
    pub fn new(
        keywords: Option<ValuesRef<'_, String>>,
        line_range: Option<ValuesRef<'_, usize>>,
        date_range: Option<ValuesRef<'_, String>>,
    ) -> Viewer {
        match (keywords, line_range, date_range) {
            (Some(kw), Some(lr), None) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: Some(lr.into_iter().copied().collect()),
                date_range: None,
            },
            (Some(kw), None, Some(dr)) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: None,
                date_range: Some(dr.into_iter().cloned().collect()),
            },
            (Some(kw), None, None) => Viewer {
                keywords: Some(kw.into_iter().cloned().collect()),
                line_range: None,
                date_range: None,
            },
            (None, None, None) => Viewer {
                keywords: None,
                line_range: None,
                date_range: None,
            },
            (None, None, Some(dr)) => Viewer {
                keywords: None,
                line_range: None,
                date_range: Some(dr.into_iter().cloned().collect()),
            },
            (None, Some(lr), None) => Viewer {
                keywords: None,
                line_range: Some(lr.into_iter().copied().collect()),
                date_range: None,
            },
            (None, Some(_), Some(_)) => unreachable!(), // Invalid
            (Some(_), Some(_), Some(_)) => unreachable!(), // Invalid
        }
    }

    /// Filter an iterator of String lines for line numbers selected by the line range.
    fn filter_with_line_range<I>(
        &self,
        iter: I,
    ) -> Result<impl Iterator<Item = (usize, String)>, &str>
    where
        I: Iterator<Item = (usize, String)>,
    {
        if self.line_range.is_none() {
            return Err("No line range found.");
        }

        let lr = self.line_range.as_ref().unwrap();

        let res = iter
            .filter(|(i, _)| {
                // If line range is only one value skip ln if it's not the selected ln.
                if lr.len() == 1 && *i != lr[0] {
                    return false;
                }

                // If line range is two values then skip ln if it's outside the range selected.
                if lr.len() == 2 && (*i < lr[0] || *i > lr[1]) {
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
    ) -> Result<impl Iterator<Item = (usize, String)>, &str>
    where
        I: Iterator<Item = (usize, String)>,
    {
        if self.keywords.is_none() {
            return Err("No keywords found.");
        }

        // Filter lines for lines that contain any of the keywords indicated by caller.
        let res = iter
            .filter(|(_, l)| {
                self.keywords
                    .clone()
                    .unwrap()
                    .iter()
                    .any(|kw| l.contains(kw))
            })
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
        ) {
            (Some(_), Some(_), None) => {
                let lines = self.filter_with_line_range(buffer.lines().flatten().enumerate())?;
                let lines = self.filter_with_keywords(lines)?;
                self.display_lines(lines)
            }
            (None, None, None) => self.display_lines(buffer.lines().flatten().enumerate()),
            (None, None, Some(_)) => unimplemented!(),
            (None, Some(_), None) => {
                let lines = self.filter_with_line_range(buffer.lines().flatten().enumerate())?;
                self.display_lines(lines)
            }
            (None, Some(_), Some(_)) => unreachable!(),
            (Some(_), None, None) => {
                let lines = self.filter_with_keywords(buffer.lines().flatten().enumerate())?;
                self.display_lines(lines)
            }
            (Some(_), None, Some(_)) => unimplemented!(),
            (Some(_), Some(_), Some(_)) => unreachable!(),
        }
    }
}

#[test]
// TODO: More robust testing.
fn test_viewer() {
    let viewer_none = Viewer::new(None, None, None);

    assert_eq!(viewer_none.line_range, None);
    assert_eq!(viewer_none.date_range, None);
    assert_eq!(viewer_none.keywords, None);
}
