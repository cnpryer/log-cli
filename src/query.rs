use crate::{
    lines::{EnumeratedLines, Lines},
    Result,
};
use std::path::PathBuf;

/// Kinds of ranges that can be executed.
enum RangeKind {
    /// Standard line range to select data between a lower bound line number and an upper bound.
    LineRange((usize, usize)),
    /// Relative range from head of file.
    Head(usize),
    /// Relateive range from tail of file.
    Tail(usize),
}

/// Kinds of evaluation strategies.
#[derive(PartialEq, Eq)]
enum EvalKind {
    /// Strategy to select all data available from query plan.
    All,
    /// Strategy to select any data available from query plan.
    Any,
    /// Strategy to select only the latest N data from query plan.
    Latest(usize),
}

/// Instructions for query to execute.
#[derive(Default)]
pub(crate) struct Instructions {
    paths: Vec<PathBuf>,
    keywords: Vec<String>,
    ranges: Vec<RangeKind>,
    evals: Vec<EvalKind>,
}

impl Instructions {
    /// Create unpopulated `Instructions`.
    pub(crate) fn new() -> Instructions {
        Instructions {
            paths: vec![],
            keywords: vec![],
            ranges: vec![],
            evals: vec![],
        }
    }

    /// Add path to instructions and return new `Instructions`.
    pub(crate) fn add_path(self, path: PathBuf) -> Instructions {
        let mut instructions = self;

        instructions.paths.push(path);

        instructions
    }

    /// Add keyword to instructions and return new `Instructions`.
    pub(crate) fn add_keyword(self, word: String) -> Instructions {
        let mut instructions = self;

        instructions.keywords.push(word);

        instructions
    }

    /// Add range to instructions and return new `Instructions`.
    pub(crate) fn add_range(self, name: &str, lower: usize, upper: usize) -> Instructions {
        let mut instructions = self;

        let kind = match name {
            "line-range" => RangeKind::LineRange((lower, upper)),
            _ => unreachable!(),
        };

        instructions.ranges.push(kind);

        instructions
    }

    /// Add relative range to instructions and return new `Instructions`.
    pub(crate) fn add_relative_range(self, name: &str, val: usize) -> Instructions {
        let mut instructions = self;

        let kind = match name {
            "head" => RangeKind::Head(val),
            "tail" => RangeKind::Tail(val),
            _ => unreachable!(),
        };

        instructions.ranges.push(kind);

        instructions
    }

    /// Add eval to instructions and return new `Instructions`.
    pub(crate) fn add_eval(self, name: &str, val: Option<usize>) -> Instructions {
        let mut instructions = self;

        match name {
            "all" => instructions.evals.push(EvalKind::All),
            "any" => instructions.evals.push(EvalKind::Any),
            "latest" => instructions.evals.push(EvalKind::Latest(val.unwrap_or(1))),
            _ => unreachable!(),
        };

        instructions
    }
}

pub(crate) struct Query(Instructions);

impl Query {
    /// Build query from `Instructions`.
    pub(crate) fn build(instructions: Instructions) -> Result<Query> {
        Ok(Query(instructions))
    }

    /// Access to query `Instructions`.
    fn instructions(&self) -> &Instructions {
        &self.0
    }

    // Filter enumerated lines containing keywords depending on a desired evaulation strategy
    // (eval). An eval can be "all" or "any".
    fn filter_with_keywords(
        &self,
        lines: &[(usize, String)],
        keywords: &[String],
        eval: &EvalKind, // TODO: Use clap-recognizable enum
    ) -> Result<Vec<(usize, String)>> {
        // Filter lines for lines that contain any of the keywords indicated by caller.
        let res = lines
            .iter()
            .filter(|(_, l)| string_contains_vec_elements(l, keywords, eval))
            .into_iter()
            .cloned()
            .collect();

        Ok(res)
    }

    /// Filter enumerated lines for line numbers selected by the line range.
    fn filter_with_line_range(
        &self,
        lines: &EnumeratedLines,
        lower: &usize,
        upper: &usize,
    ) -> Result<EnumeratedLines> {
        // If there aren't any lines to filter correctly return with Ok.
        if lines.is_empty() {
            return Ok(lines.to_vec());
        }

        let res = lines
            .iter()
            .filter(|(i, _)| {
                if i >= lower && i <= upper {
                    return true;
                }

                false
            })
            .cloned()
            .collect();

        Ok(res)
    }

    /// Apply instructions to enumerated line strings.
    fn filter_lines(&self, lines: Lines) -> Result<EnumeratedLines> {
        // Create scoped mutable res of enumerated lines to operate on.
        let mut res = lines.enumerated_lines().to_owned();

        // Set the primary evaluation strategy; defaults to All.
        let primary_eval = if self.instructions().evals.contains(&EvalKind::Any) {
            EvalKind::Any
        } else {
            EvalKind::All
        };

        // Filter lines for the selected relative line ranges.
        let relative_range = self.instructions().ranges.iter().find_map(|e| match e {
            RangeKind::Head(_) => Some(create_relative_usize_range(e, res.len() - 1)),
            RangeKind::Tail(_) => Some(create_relative_usize_range(e, res.len() - 1)),
            _ => None,
        });

        if let Some((lower, upper)) = relative_range {
            res = res[lower..=upper].to_vec()
        }

        // Filter for absolute ranges.
        let range = self.instructions().ranges.iter().find_map(|e| match e {
            RangeKind::LineRange((lower, upper)) => Some((lower, upper)),
            _ => None,
        });

        if let Some((lower, upper)) = range {
            res = self.filter_with_line_range(&res, lower, upper)?;
        }

        // Filter keywords from remaining lines.
        if !self.instructions().keywords.is_empty() {
            res = self.filter_with_keywords(&res, &self.instructions().keywords, &primary_eval)?;
        }

        // Filter for latest N found in remaining lines.
        let latest = self.instructions().evals.iter().find_map(|e| match e {
            EvalKind::Latest(n) => Some(n),
            _ => None,
        });

        if let Some(n) = latest {
            if n < &res.len() {
                res = res[res.len() - n..].to_vec();
            }
        }

        Ok(res.to_vec())
    }

    /// Execute query using `Instructions` and print results to stdout.
    pub(crate) fn execute(&self) -> Result<()> {
        for (i, path) in self.instructions().paths.iter().enumerate() {
            let res = Lines::new(self.filter_lines(Lines::read(path)?)?);
            print!(
                "\nFile ({}/{}): {}",
                i + 1,
                self.instructions().paths.len(),
                path.display()
            );
            println!("{}", res);
        }

        Ok(())
    }
}

/// Check string for "any" or "all" (eval) elements from vector. Return true of eval is met,
/// otherwise return false.
fn string_contains_vec_elements(string: &str, vec: &[String], eval: &EvalKind) -> bool {
    match eval {
        EvalKind::All => vec.iter().all(|e| string.contains(e)),
        EvalKind::Any => vec.iter().any(|e| string.contains(e)),
        _ => false,
    }
}

/// Create an inclusive usize range of lower and upper values relative to max values and kind of
/// range.
fn create_relative_usize_range(kind: &RangeKind, max: usize) -> (usize, usize) {
    match kind {
        RangeKind::Head(n) => {
            if *n - 1 > max {
                (0, max)
            } else {
                (0, *n - 1)
            }
        }
        RangeKind::Tail(n) => {
            if *n > max {
                (0, max)
            } else {
                (max - *n + 1, max)
            }
        }
        _ => (0, max),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_paths_to_instruction() {
        let paths = vec![PathBuf::from("test/path/1"), PathBuf::from("test/path/2")];
        let instructions = Instructions::new()
            .add_path(paths[0].clone())
            .add_path(paths[1].clone());
        assert_eq!(instructions.paths, paths);
    }

    #[test]
    fn add_keywords_to_instructions() {
        let words = vec!["word1", "word2"];
        let instructions = Instructions::new()
            .add_keyword(words[0].to_string())
            .add_keyword(words[1].to_string());
        assert_eq!(instructions.keywords, words);
    }

    #[test]
    fn add_ranges_to_instructions() {
        let ranges: Vec<(usize, usize)> = vec![(0, 0), (1, 2)];
        let instructions = Instructions::new()
            .add_range("line-range", ranges[0].0, ranges[0].1)
            .add_range("line-range", ranges[1].0, ranges[1].1);
        assert_eq!(instructions.ranges.len(), ranges.len());
    }

    #[test]
    fn add_eval_to_instructions() {
        let evals = vec!["all", "any", "latest"];
        let instructions = Instructions::new()
            .add_eval(evals[0].clone(), None)
            .add_eval(evals[1].clone(), None)
            .add_eval(evals[2].clone(), Some(2));
        assert_eq!(instructions.evals.len(), evals.len());
    }

    #[test]
    fn execute_default_query() {
        let instructions = Instructions::default();
        let query = Query::build(instructions);
        let lines = Lines::new(
            vec!["This is a line.".to_string()]
                .into_iter()
                .enumerate()
                .collect(),
        );

        if let Ok(q) = query {
            let res = q.filter_lines(lines).unwrap();

            assert_eq!(res[0].0, 0);
            assert_eq!(res[0].1, "This is a line.");
        } else {
            panic!("test failed");
        }
    }

    #[test]
    fn execute_query_with_keywords() {
        let instructions = Instructions::new().add_keyword("target".to_string());
        let query = Query::build(instructions);
        let lines = Lines::new(
            vec![
                "This is a line.".to_string(),
                "This line has the target.".to_string(),
            ]
            .into_iter()
            .enumerate()
            .collect(),
        );

        if let Ok(q) = query {
            let res = q.filter_lines(lines).unwrap();

            assert_eq!(res[0].0, 1);
            assert_eq!(res[0].1, "This line has the target.");
        } else {
            panic!("test failed");
        }
    }

    #[test]
    fn execute_query_with_any_keywords() {
        let instructions = Instructions::new()
            .add_keyword("target".to_string())
            .add_keyword("line".to_string())
            .add_eval("any", None);
        let query = Query::build(instructions);
        let lines = Lines::new(
            vec![
                "This should not be in res.".to_string(),
                "This is a line.".to_string(),
                "This has the target.".to_string(),
            ]
            .into_iter()
            .enumerate()
            .collect(),
        );

        if let Ok(q) = query {
            let res = q.filter_lines(lines).unwrap();

            assert_eq!(res[0].0, 1);
            assert_eq!(res[0].1, "This is a line.");
            assert_eq!(res[1].0, 2);
            assert_eq!(res[1].1, "This has the target.");
        } else {
            panic!("test failed");
        }
    }

    #[test]
    fn execute_query_with_line_range() {
        let instructions = Instructions::new().add_range("line-range", 1, 2);
        let query = Query::build(instructions);
        let lines = Lines::new(
            vec![
                "This should not be in res.".to_string(),
                "This is a line.".to_string(),
                "This has the target.".to_string(),
            ]
            .into_iter()
            .enumerate()
            .collect(),
        );

        if let Ok(q) = query {
            let res = q.filter_lines(lines).unwrap();

            assert_eq!(res[0].0, 1);
            assert_eq!(res[0].1, "This is a line.");
            assert_eq!(res[1].0, 2);
            assert_eq!(res[1].1, "This has the target.");
        } else {
            panic!("test failed");
        }
    }

    #[test]
    fn execute_query_with_latest_overrided() {
        let instructions = Instructions::new().add_eval("latest", Some(2));
        let query = Query::build(instructions);
        let lines = Lines::new(
            vec![
                "This should not be in res.".to_string(),
                "This is a line.".to_string(),
                "This has the target.".to_string(),
            ]
            .into_iter()
            .enumerate()
            .collect(),
        );

        if let Ok(q) = query {
            let res = q.filter_lines(lines).unwrap();

            assert_eq!(res[0].0, 1);
            assert_eq!(res[0].1, "This is a line.");
            assert_eq!(res[1].0, 2);
            assert_eq!(res[1].1, "This has the target.");
        } else {
            panic!("test failed");
        }
    }

    #[test]
    fn execute_query_with_head() {
        let instructions = Instructions::new().add_relative_range("head", 1);
        let query = Query::build(instructions);
        let lines = Lines::new(
            vec![
                "This should be in res.".to_string(),
                "This is a line.".to_string(),
                "This has the target.".to_string(),
            ]
            .into_iter()
            .enumerate()
            .collect(),
        );

        if let Ok(q) = query {
            let res = q.filter_lines(lines).unwrap();

            assert_eq!(res[0].0, 0);
            assert_eq!(res[0].1, "This should be in res.");
            assert_eq!(res.len(), 1);
        } else {
            panic!("test failed");
        }
    }

    #[test]
    fn execute_query_with_tail() {
        let instructions = Instructions::new().add_relative_range("tail", 1);
        let query = Query::build(instructions);
        let lines = Lines::new(
            vec![
                "This should not be in res.".to_string(),
                "This is a line.".to_string(),
                "This has the target.".to_string(),
            ]
            .into_iter()
            .enumerate()
            .collect(),
        );

        if let Ok(q) = query {
            let res = q.filter_lines(lines).unwrap();

            assert_eq!(res[0].0, 2);
            assert_eq!(res[0].1, "This has the target.");
            assert_eq!(res.len(), 1);
        } else {
            panic!("test failed");
        }
    }
}
