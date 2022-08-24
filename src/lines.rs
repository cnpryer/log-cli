use crate::{buffer, Result};
use std::{fmt, io::BufRead, path::PathBuf};

/// Alias to enumerated lines data structure.
pub(crate) type EnumeratedLines = Vec<(usize, String)>;

/// Collected and enumerated line strings.
pub(crate) struct Lines(EnumeratedLines);

#[allow(dead_code)]
impl Lines {
    /// Create `Lines` from already enumerated lines.
    pub(crate) fn new(lines: EnumeratedLines) -> Lines {
        Lines(lines)
    }

    /// Create `Lines` from a filepath.
    pub(crate) fn read(path: &PathBuf) -> Result<Lines> {
        let buf = buffer::read_file(path)?;

        Ok(Lines(buf.lines().flatten().enumerate().collect()))
    }

    /// Access enumerated lines collected.
    pub(crate) fn enumerated_lines(&self) -> &EnumeratedLines {
        &self.0
    }

    /// Update wrapped enumerated lines.
    pub(crate) fn set_lines(&mut self, lines: EnumeratedLines) {
        self.0 = lines
    }
}

impl fmt::Display for Lines {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lines = self.enumerated_lines();

        // To pad line numbers, get the length of .len as a string.
        let pad_len = lines.len().to_string().len();

        write!(f, "")?;

        for (ln, line) in lines.iter() {
            write!(f, "\nln{:0width$} {}", ln, line, width = pad_len)?;
        }

        Ok(())
    }
}
