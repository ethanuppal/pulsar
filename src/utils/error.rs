use colored::*;
use std::fmt::Display;

use super::loc::Loc;

type ErrorCode = i32;

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

pub enum Style {
    Primary,
    Secondary
}

/// An error at a given location with various information and diagnostics. Use
/// [`Error::to_string`] to obtain a printable version of the error.
pub struct Error<'a> {
    style: Style,
    level: Level,
    code: ErrorCode,
    loc: Loc<'a>,
    length: usize,
    message: String,
    explain: Option<String>,
    fix: Option<String>
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}\n",
            self.level.form_header(self.code).bold(),
            self.message.bold()
        )?;
        let (lines, current_line_pos) = self.loc.lines(2, 2);
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
                let split_second = split_first + self.length - 1;
                let (part1, rest) = line.split_at(split_first);
                let (part2, part3) =
                    rest.split_at(split_second - split_first + 1);
                write!(
                    f,
                    "{}{}{}",
                    part1,
                    part2.color(self.level.color()),
                    part3
                )?;
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
                        create_error_pointer(part2.len())
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

impl<'a> Default for Error<'a> {
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

pub struct ErrorBuilder<'a> {
    error: Error<'a>
}

impl<'a> ErrorBuilder<'a> {
    pub fn new() -> Self {
        ErrorBuilder {
            error: Error::default()
        }
    }

    pub fn of_style(mut self, style: Style) -> Self {
        self.error.style = style;
        self
    }

    pub fn at_level(mut self, level: Level) -> Self {
        self.error.level = level;
        self
    }

    pub fn at_region(mut self, loc: Loc<'a>, length: usize) -> Self {
        self.error.loc = loc;
        self.error.length = length;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.error.message = message;
        self
    }

    pub fn explain(mut self, explain: String) -> Self {
        self.error.explain = Some(explain);
        self
    }

    pub fn fix(mut self, fix: String) -> Self {
        self.error.fix = Some(fix);
        self
    }

    pub fn build(self) -> Error<'a> {
        self.error
    }
}
