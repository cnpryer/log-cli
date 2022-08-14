use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use clap::parser::ValuesRef;

pub struct Viewer {
    keywords: Option<Vec<String>>,
    line_range: Option<Vec<u32>>,
    date_range: Option<Vec<String>>,
}

impl Viewer {
    // TODO: Certain parameters cannot be used together.
    pub fn new(
        keywords: Option<ValuesRef<'_, String>>,
        line_range: Option<ValuesRef<'_, u32>>,
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
    fn _display_with_lines(&self, _buffer: &mut BufReader<File>) {
        if self.line_range.is_none() {
            return;
        }
        // for line in buffer.lines().flatten() {
        //     println!("{}", line);
        // }
        unimplemented!()
    }

    /// Display all lines within date range (inlcusive).
    fn _display_with_dates(&self, _buffer: &mut BufReader<File>) {
        if self.date_range.is_none() {
            return;
        }

        // for line in buffer.lines().flatten() {
        //     println!("{}", line);
        // }
        unimplemented!()
    }

    /// Display all lines that contain any keyword.
    fn display_with_keywords(&self, buffer: &mut BufReader<File>) {
        if self.keywords.is_none() {
            return;
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
    }

    /// Display the entire file.
    fn display_all(&self, buffer: &mut BufReader<File>) {
        for line in buffer.lines().flatten() {
            println!("{}", line);
        }
    }

    /// Display with viewer function to display the file via its `BufReader`.
    // TODO:
    //       - Use `Result`
    //       - Validation and error handling.
    pub fn display_with(&self, buffer: &mut BufReader<File>) {
        if self.line_range.is_some() {
            return self._display_with_lines(buffer);
        }
        if self.date_range.is_some() {
            return self._display_with_dates(buffer);
        }

        if self.keywords.is_some() {
            return self.display_with_keywords(buffer);
        }

        self.display_all(buffer);
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
