use crate::frontend::token::Token;

use super::loc::Loc;
use colored::*;
use std::{cell::RefCell, fmt::Display, io, rc::Rc};

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum ErrorCode {
    Unknown,
    UnrecognizedCharacter,
    UnexpectedEOF,
    UnexpectedToken,
    InvalidTopLevelConstruct,
    ConstructShouldBeTopLevel,
    InvalidTokenForStatement,
    InvalidOperatorSyntax,
    MalformedType,
    UnboundName
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (*self as i32).fmt(f)
    }
}

impl Default for ErrorCode {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(PartialEq, Eq)]
pub enum Level {
    Info,
    Note,
    Warning,
    Error
}

impl Level {
    fn color(&self) -> Color {
        match self {
            Self::Info => Color::BrightWhite,
            Self::Note => Color::BrightYellow,
            Self::Warning => Color::Yellow,
            Self::Error => Color::BrightRed
        }
    }

    fn form_header(&self, code: ErrorCode) -> ColoredString {
        let string = self.to_string();
        format!(
            "{}[{}{:04}]",
            string,
            string.chars().next().unwrap().to_ascii_uppercase(),
            code
        )
        .color(self.color())
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Info => "info",
                Self::Note => "note",
                Self::Warning => "warning",
                Self::Error => "error"
            }
        )
    }
}

#[derive(PartialEq, Eq)]
pub enum Style {
    Primary,
    Secondary
}

/// An error at a given location with various information and diagnostics. Use
/// [`Error::fmt`] to obtain a printable version of the error.
pub struct Error {
    style: Style,
    level: Level,
    code: ErrorCode,
    loc: Loc,
    length: usize,
    message: String,
    explain: Option<String>,
    fix: Option<String>
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.style == Style::Primary {
            write!(
                f,
                "{}: {}: ",
                self.level.form_header(self.code).bold(),
                self.loc.to_string().underline()
            )?;
        }
        write!(f, "{}\n", self.message.bold())?;
        let (lines, current_line_pos) = self.loc.lines(1, 1);
        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(
                f,
                "{}",
                format!("{: >4} │  ", i + self.loc.line - current_line_pos)
                    .dimmed()
            )?;
            if i == current_line_pos {
                let split_first = self.loc.col - 1;
                let (part1, rest) = line.split_at(split_first);
                if !line.is_empty() {
                    let split_second = split_first + self.length - 1;
                    let (part2, part3) = if split_second == line.len() {
                        (rest, "")
                    } else {
                        rest.split_at(split_second - split_first + 1)
                    };
                    write!(
                        f,
                        "{}{}{}",
                        part1,
                        part2.color(self.level.color()),
                        if part3.is_empty() {
                            " ".on_color(self.level.color())
                        } else {
                            part3.into()
                        }
                    )?;
                } else {
                    write!(f, "{}{}", part1, " ".on_color(self.level.color()))?;
                }
                if let Some(explain) = &self.explain {
                    fn create_error_pointer(length: usize) -> String {
                        match length {
                            0 => "".into(),
                            1 => "│".into(),
                            n => format!("└{}", "─".repeat(n - 1))
                        }
                    }
                    writeln!(f)?;
                    write!(
                        f,
                        "        {}{} {}",
                        " ".repeat(part1.len()),
                        create_error_pointer(self.length)
                            .color(self.level.color()),
                        explain.bold().italic()
                    )?;
                }
            } else {
                write!(f, "{}", line)?;
            }
        }
        if let Some(fix) = &self.fix {
            write!(f, "\n{}", fix.bold())?;
        }
        Ok(())
    }
}

impl Default for Error {
    fn default() -> Self {
        Error {
            style: Style::Primary,
            level: Level::Error,
            code: ErrorCode::default(),
            loc: Loc::default(),
            length: 0,
            message: String::default(),
            explain: None,
            fix: None
        }
    }
}

/// An interface for fluently constructing errors.
pub struct ErrorBuilder {
    error: Error
}

impl ErrorBuilder {
    /// Constructs a new error builder.
    pub fn new() -> Self {
        ErrorBuilder {
            error: Error::default()
        }
    }

    /// Uses the display style `style`.
    pub fn of_style(mut self, style: Style) -> Self {
        self.error.style = style;
        self
    }

    /// Uses the severity level `level`.
    pub fn at_level(mut self, level: Level) -> Self {
        self.error.level = level;
        self
    }

    /// Uses the error code `code`.
    pub fn with_code(mut self, code: ErrorCode) -> Self {
        self.error.code = code;
        self
    }

    /// Locates the error as extending `length` characters starting from `loc`.
    pub fn at_region(mut self, loc: Loc, length: usize) -> Self {
        self.error.loc = loc;
        self.error.length = length;
        self
    }

    /// Locates the error at the given token `token`.
    pub fn at_token(self, token: &Token) -> Self {
        self.at_region(token.loc.clone(), token.length())
    }

    /// Uses the main error description `message`.
    pub fn message(mut self, message: String) -> Self {
        self.error.message = message;
        self
    }

    /// Uses an explanatory note `explain`.
    pub fn explain(mut self, explain: String) -> Self {
        self.error.explain = Some(explain);
        self
    }

    /// Uses a recommendation `fix`.
    pub fn fix(mut self, fix: String) -> Self {
        self.error.fix = Some(fix);
        self
    }

    /// Produces the error.
    pub fn build(self) -> Error {
        self.error
    }
}

/// A shared error manager with an error recording limit.
pub struct ErrorManager {
    max_count: usize,
    primary_count: usize,
    errors: Vec<Error>
}

impl ErrorManager {
    /// Constructs a shared error manager that can record up to `max_count`
    /// primary errors.
    pub fn with_max_count(max_count: usize) -> Rc<RefCell<ErrorManager>> {
        Rc::new(RefCell::new(ErrorManager {
            max_count,
            primary_count: 0,
            errors: vec![]
        }))
    }

    /// Whether the error manager has recorded an error.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Whether the error manager cannot take any further primary errors.
    pub fn is_full(&self) -> bool {
        self.primary_count == self.max_count
    }

    /// Records `error` and returns `true` unless `is_full()`.
    pub fn record(&mut self, error: Error) -> bool {
        if self.is_full() {
            false
        } else {
            if error.style == Style::Primary && error.level == Level::Error {
                self.primary_count += 1
            }
            self.errors.push(error);
            true
        }
    }

    /// Prints and clears all recorded errors to `output`.
    pub fn consume_and_write<W: io::Write>(
        &mut self, output: &mut W
    ) -> io::Result<()> {
        let mut i = 0;
        for error in &self.errors {
            if error.style == Style::Primary && i > 0 {
                output.write(&[b'\n'])?;
            }
            output.write_all(error.to_string().as_bytes())?;
            output.write(&[b'\n'])?;
            output.flush()?;
            i += 1;
        }
        self.errors.clear();
        self.primary_count = 0;
        Ok(())
    }
}
