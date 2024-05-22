use std::{fmt::Display, rc::Rc};

/// Different sources of text data.
#[derive(Clone, Debug)]
pub enum Source {
    /// `Source::File { name, contents }` is a text file with name `name` and
    /// contents `contents`.
    File { name: String, contents: String }
}

impl Source {
    /// `source.lines(pos, before, after)` is a pair of a vector containing the
    /// line in `source` at position `pos`, preceded by the up to `before`
    /// previous lines and up to `after` subsequent lines, as well as an index
    /// into the vector for the line containing `pos`.
    ///
    /// Requires: `pos` is a valid position in `source`.
    fn lines(
        &self, pos: usize, before: usize, after: usize
    ) -> (Vec<String>, usize) {
        match self {
            Self::File { name: _, contents } => {
                assert!(pos < contents.len());
                let bytes = contents.as_bytes();

                // Find the bounds of the current line
                let mut start_pos = pos;
                while start_pos > 0 && bytes[start_pos - 1] != b'\n' {
                    start_pos -= 1;
                }
                let mut end_pos = start_pos;
                while end_pos < contents.len() && bytes[end_pos] != b'\n' {
                    end_pos += 1;
                }
                end_pos += 1;

                // Slice the contents to get the current line
                let line = contents
                    .get(start_pos..end_pos - 1)
                    .unwrap_or_default()
                    .to_string();

                // Make iterators for the before/after lines
                let before_lines: Vec<_> = {
                    let (before_contents, _) = contents.split_at(start_pos);
                    let mut result: Vec<_> = before_contents
                        .lines()
                        .rev()
                        .take(before)
                        .map(String::from)
                        .collect();
                    result.reverse();
                    result
                };
                let after_lines: Vec<_> = if end_pos < contents.len() {
                    let (_, after_contents) = contents.split_at(end_pos);
                    after_contents
                        .lines()
                        .take(after)
                        .map(String::from)
                        .collect()
                } else {
                    std::iter::empty().collect()
                };

                // Construct the final result
                let mut result = vec![];
                result.extend(before_lines);
                let pos_current_line = result.len();
                result.push(line);
                result.extend(after_lines);

                (result, pos_current_line)
            }
        }
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::File { name, contents: _ } => write!(f, "{}", name)
        }
    }
}

impl Default for Source {
    fn default() -> Self {
        Source::File {
            name: String::new(),
            contents: String::new()
        }
    }
}

/// `Loc(line, col, pos, source)` is a location referring to line `line` and
/// column `col` of `source`, where the combination of `line` and `col` produces
/// a direct offset `pos`. It is formatted as `"{source}:{line}:{col}"` where
/// `{source}` is the formatted substitution of `source` and likewise for
/// `line`/`col`.
#[derive(Debug)]
pub struct Loc {
    pub line: usize,
    pub col: usize,
    pub pos: usize,
    pub source: Rc<Source>
}

impl Loc {
    /// See [`Source::lines`].
    pub fn lines(&self, before: usize, after: usize) -> (Vec<String>, usize) {
        self.source.lines(self.pos, before, after)
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.source.as_ref(), self.line, self.col)
    }
}

impl Default for Loc {
    fn default() -> Self {
        Loc {
            line: 0,
            col: 0,
            pos: 0,
            source: Rc::new(Source::default())
        }
    }
}
