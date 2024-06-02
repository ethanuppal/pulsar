// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use std::{cmp::Ordering, fmt::Display, rc::Rc};

/// Different sources of text data.
#[derive(Clone, Debug, Eq)]
pub enum Source {
    /// `Source::File { name, contents }` is a text file with name `name` and
    /// contents `contents`.
    ///
    /// Invariant: if two [`Source::File`]s have the same `name`, they must
    /// represent the same file. For this reason, it is recommended to use
    /// fully-qualified paths for `name`.
    File {
        name: String,
        contents: String
    },
    Unknown
}

impl Source {
    pub fn file(name: String, contents: String) -> Rc<Source> {
        Rc::new(Source::File { name, contents })
    }

    /// `contents(source)` is the string contents of `source`.
    pub fn contents(&self) -> &str {
        match self {
            Self::File { name: _, contents } => contents,
            Self::Unknown => ""
        }
    }

    /// @see [`Loc::lines`]
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
            Self::Unknown => (vec![], 0)
        }
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::File { name, contents: _ } => write!(f, "{}", name),
            Source::Unknown => write!(f, "<unknown>")
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

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unknown, Self::Unknown) => true,
            (
                Self::File { name, contents: _ },
                Self::File {
                    name: other_name,
                    contents: _
                }
            ) => name == other_name,
            _ => false
        }
    }
}

/// `Loc(line, col, pos, source)` is a location referring to line `line` and
/// column `col` of `source`, where the combination of `line` and `col` produces
/// a direct offset `pos`. It is formatted as `"{source}:{line}:{col}"` where
/// `{source}` is the formatted substitution of `source` and likewise for
/// `line`/`col`. It is required that no numeric field is negative, that is,
/// `line`, `col`, and `pos` should be treated as if they were of type `usize`.
#[derive(Debug, Clone, Eq)]
pub struct Loc {
    pub line: isize,
    pub col: isize,
    pub pos: isize,
    pub source: Rc<Source>
}

impl Loc {
    /// `loc.lines(before, after)` is a pair of a vector containing the
    /// line in `loc.source` at position `loc.pos`, preceded by the up to
    /// `before` previous lines and up to `after` subsequent lines, as well
    /// as an index into the vector for the line containing `loc.pos`.
    ///
    /// Requires: `loc.pos` is a valid position in `loc.source`.
    pub fn lines(&self, before: usize, after: usize) -> (Vec<String>, usize) {
        self.source.lines(self.pos as usize, before, after)
    }

    pub fn make_invalid() -> Self {
        Loc {
            line: 0,
            col: 0,
            pos: 0,
            source: Rc::new(Source::Unknown)
        }
    }

    pub fn is_invalid(&self) -> bool {
        let invalid = Loc::make_invalid();
        self.line == invalid.line
            && self.col == invalid.col
            && self.pos == invalid.pos
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.source, self.line, self.col)
    }
}

impl Default for Loc {
    fn default() -> Self {
        Loc {
            line: 1,
            col: 1,
            pos: 0,
            source: Rc::new(Source::Unknown)
        }
    }
}

impl PartialEq for Loc {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line
            && self.col == other.col
            && self.pos == other.pos
            && self.source.as_ref() == other.source.as_ref()
    }
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.source != other.source {
            None
        } else {
            self.pos.partial_cmp(&other.pos)
        }
    }
}

/// The location enclosed by a region begins at `start` and ends exclusively
/// at `end`. It is required that both locations come from the same source and
/// that `end` monotonically proceeds `start` (so `start` and `end` can compare
/// equal). This invariant is enforced when constructing through
/// [`Region::new`].
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Region {
    /// An inclusive lower bound (see [`Loc::partial_cmp`]) on the region
    /// enclosed.
    pub start: Loc,

    /// An exclusive upper bound (see [`Loc::partial_cmp`]) on the region
    /// enclosed.
    pub end: Loc
}

/// The line section with `start` and `end` represents the characters at
/// positions from lower bound `start` to exclusive upper bound `end` on a line.
/// The core invariant that `end >= start` is enforced by [`LineSection::new`].
pub struct LineSection {
    /// The initial position on the line.
    pub start: isize,

    /// One after the final valid position contained by this line section on
    /// the line, that is, an exclusive upper bound on the indices of the range
    /// of characters contained by this line section.
    pub end: isize
}

impl LineSection {
    pub fn new(start: isize, end: isize) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub fn length(&self) -> usize {
        (self.end - self.start) as usize
    }
}

impl Region {
    /// A region from `start` up to (but not including) `end`.
    pub fn new(start: Loc, end: Loc) -> Region {
        assert!(start <= end, "`Region::from`: `start` and `end` must from the same source and `end` must be at least after `start`.");
        Region { start, end }
    }

    /// A region at `start` of length 1.
    ///
    /// Requires: the `start.line` contains at least one more character after
    /// `start.col`.
    pub fn unit(start: Loc) -> Region {
        let mut end = start.clone();
        end.pos += 1;
        end.col += 1;
        Region::new(start, end)
    }

    /// The source where this region occurs.
    pub fn source(&self) -> Rc<Source> {
        self.start.source.clone()
    }

    pub fn start_line(&self) -> isize {
        self.start.line
    }

    pub fn end_line(&self) -> isize {
        self.end.line
    }

    /// Given a set of *complete* `lines` from the same source as `source()`
    /// and the line number of the first line in `lines`, `start_line`, this
    /// function computes the intersection of this region and the given
    /// lines. If the output vector is non-empty, the first entry in the output
    /// vector corresponds to the first line of this region, which is not
    /// necessarily the first line in `lines`. See [`LineSection`].
    pub fn find_intersection(
        &self, lines: &Vec<String>, start_line: isize
    ) -> Vec<LineSection> {
        let mut result = vec![];
        for (i, line) in lines.iter().enumerate() {
            let actual_line = start_line + (i as isize);
            if actual_line >= self.start_line()
                && actual_line <= self.end_line()
            {
                let mut start_pos = 0;
                let mut end_pos = line.len() as isize;

                if actual_line == self.start_line() {
                    start_pos = self.start.col - 1;
                }

                if actual_line == self.end_line() {
                    end_pos = self.end.col - 1;
                }

                result.push(LineSection::new(start_pos, end_pos));
            }
        }
        result
    }
}

pub trait RegionProvider {
    /// The starting location of this region.
    fn start(&self) -> Loc;

    /// Must be in the same source and monotonically after
    /// [`RegionProvider::start`]. See [`Region`] for details.
    fn end(&self) -> Loc;

    /// The region of this object.
    fn region(&self) -> Region {
        Region::new(self.start(), self.end())
    }
}

impl RegionProvider for Region {
    fn start(&self) -> Loc {
        self.start.clone()
    }

    fn end(&self) -> Loc {
        self.end.clone()
    }
}
