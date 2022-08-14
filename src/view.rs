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

    /// Display all lines within line range (inclusive).
    fn display_with_lines(&self, buffer: &mut BufReader<File>) -> Result<(), &str> {
        if self.line_range.is_none() {
            return Err("No line range found.");
        }

        let lr = self.line_range.as_ref().unwrap();

        for (i, line) in buffer.lines().flatten().enumerate() {
            // If line range is only one value skip ln if it's not the selected ln.
            if lr.len() == 1 && i != lr[0] {
                continue;
            }

            // If line range is two values then skip ln if it's outside the range selected.
            if lr.len() == 2 && (i < lr[0] || i > lr[1]) {
                continue;
            }

            println!("{}", line);
        }

        Ok(())
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

    /// Display all lines that contain any keyword.
    fn display_with_keywords(&self, buffer: &mut BufReader<File>) -> Result<(), &str> {
        if self.keywords.is_none() {
            return Err("No keywords found.");
        }

        // Filter lines for lines that contain any of the keywords indicated by caller.
        let lines = buffer.lines().flatten().filter(|ln| {
            self.keywords
                .clone()
                .unwrap()
                .iter()
                .any(|kw| ln.contains(kw))
        });

        for line in lines {
            println!("{}", line);
        }

        Ok(())
    }

    /// Display the entire file.
    fn display_all(&self, buffer: &mut BufReader<File>) -> Result<(), &str> {
        for line in buffer.lines().flatten() {
            println!("{}", line);
        }

        Ok(())
    }

    /// Display with viewer function to display the file via its `BufReader`.
    // TODO:
    //       - Use `Result`
    //       - Validation and error handling.
    pub fn display_with(&self, buffer: &mut BufReader<File>) -> Result<(), &str> {
        if self.line_range.is_some() {
            return self.display_with_lines(buffer);
        }
        if self.date_range.is_some() {
            return self._display_with_dates(buffer);
        }

        if self.keywords.is_some() {
            return self.display_with_keywords(buffer);
        }

        self.display_all(buffer)
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
