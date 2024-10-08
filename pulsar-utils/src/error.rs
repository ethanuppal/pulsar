//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::span::{Span, SpanProvider};
use colored::*;
use std::{
    fmt::{self, Display},
    io
};

/// Also applies the following procedural macros:
/// ```
/// #[repr(i32)]
/// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// ```
macro_rules! string_associated_pub_enum {
    {
        @$func_name:ident pub enum $enum_name:ident {
            $(
                #[str = $variant_str:literal]
                $variant_name:ident
            ),*
        }
    } => {
        #[repr(i32)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $enum_name {
            $(
                $variant_name
            ),*,
            COUNT
        }

        impl $enum_name {
            pub fn $func_name(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant_name => $variant_str
                    ),*,
                    _ => panic!()
                }
            }

            pub fn from(value: i32) -> Option<Self> {
                if value >= 0 && value < (Self::COUNT as i32) {
                    Some(unsafe { std::mem::transmute::<i32, $enum_name>(value) })
                } else {
                    None
                }
            }
        }
    }
}

string_associated_pub_enum! {
    @description pub enum ErrorCode {
        #[str = "Quite unfortunate indeed."]
        WompWomp,
        #[str = "The lexer encountered a character that isn't a valid part of any language construct. Currently, UTF-8 and certain symbols are not handled."]
        UnrecognizedCharacter,
        #[str = "The end of file was encountered in the middle of parsing a language construct. For example, missing the ending brace to a function causes this error."]
        UnexpectedEOF,
        #[str = "A different token was encountered than expected."]
        UnexpectedToken,
        #[str = "The only valid top-level construct is a function."]
        InvalidTopLevelConstruct,
        #[str = "Functions should be placed at the top level."]
        ConstructShouldBeTopLevel,
        #[str = "The parser encountered an incorrect token when expecting the start of a statement. Valid statements begin with `let`, `for`, or an lvalue expression."]
        InvalidTokenToStartStatement,
        #[str = "An operator was misued."]
        InvalidOperatorSyntax,
        #[str = "A type was syntactically incorrect. For example, an array type was declared with negative size."]
        MalformedType,
        #[str = "The oarser encountered an identifier not bound to any variable or function in scope."]
        UnboundName,
        #[str = "Types were not fully resolved at compile time."]
        AmbiguousType,
        #[str = "Hindley-Milner constraints were not satisfiable."]
        UnificationFailure,
        #[str = "An affine resource was used twice"]
        AffineResource
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (*self as i32).fmt(f)
    }
}

impl Default for ErrorCode {
    fn default() -> Self {
        Self::WompWomp
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(PartialEq, Eq, Debug)]
pub enum Style {
    Primary,
    Secondary
}

/// An error at a given location with various information and diagnostics. Use
/// [`Error::fmt`] to obtain a printable version of the error.
#[derive(Debug)]
pub struct Error {
    style: Style,
    level: Level,
    code: ErrorCode,
    span: Option<Span>,
    message: String,
    explain: Option<String>,
    fix: Option<String>
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only primary-style messages have a header indicating they are the
        // root of an error message and not auxillary information
        if self.style == Style::Primary {
            write!(f, "{}: ", self.level.form_header(self.code).bold(),)?;
            if self.span.is_some() {
                write!(
                    f,
                    "{}: ",
                    self.span.as_ref().unwrap().start.to_string().underline()
                )?;
            }
        }
        writeln!(f, "{}", self.message.bold())?;

        // If there is no span associated with this error, then we have
        // nothing more to print
        if self.span.is_none() {
            return Ok(());
        }

        // Otherwise, we print the span via a sequence of lines from the
        // source.
        let span = self.span.as_ref().unwrap();
        let span_extra_lines = (span.end.line - span.start.line) as usize;
        let show_lines_before = 1;
        let show_lines_after = 1;
        let mut already_explained = false;
        writeln!(f, "{}", "     │  ".dimmed())?;
        let (lines, current_line_pos) = span
            .start
            .lines(show_lines_before, span_extra_lines + show_lines_after);
        let show_start_line = span.start_line() - (show_lines_before as isize);
        let span_line_sections =
            span.find_intersection(&lines, show_start_line);
        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(
                f,
                "{}",
                format!(
                    "{: >4} │  ",
                    i + (span.start.line as usize) - current_line_pos
                )
                .dimmed()
            )?;
            if i >= current_line_pos && i <= current_line_pos + span_extra_lines
            {
                let line_section = &span_line_sections[i - current_line_pos];
                let split_first = line_section.start;
                let (part1, rest) = line.split_at(split_first as usize);
                if !line.is_empty() {
                    let split_second =
                        split_first + (line_section.length() as isize) - 1;
                    let (part2, part3) = if (split_second as usize)
                        == line.len()
                    {
                        (rest, "")
                    } else {
                        rest.split_at((split_second - split_first + 1) as usize)
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
                if let Some(explain) =
                    &self.explain.as_ref().filter(|_| !already_explained)
                {
                    already_explained = true;
                    fn create_error_pointer(length: usize) -> String {
                        match length {
                            0 => "".into(),
                            n => format!("└{}", "─".repeat(n - 1))
                        }
                    }
                    writeln!(f)?;
                    write!(
                        f,
                        "{}  {}{} {}",
                        "     │".dimmed(),
                        " ".repeat(part1.len()),
                        create_error_pointer(line_section.length())
                            .color(self.level.color()),
                        explain.bold().italic()
                    )?;
                }
            } else {
                write!(f, "{}", line)?;
            }
        }
        write!(f, "\n{}", "     │  ".dimmed())?;
        if let Some(fix) = &self.fix {
            write!(f, "\nSuggestion: {}", fix.bold())?;
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
            span: None,
            message: String::default(),
            explain: None,
            fix: None
        }
    }
}

/// An interface for fluently constructing errors.
#[derive(Default)]
pub struct ErrorBuilder {
    error: Error
}

impl ErrorBuilder {
    /// Constructs a new error builder.
    pub fn new() -> Self {
        Self::default()
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

    /// Locates the error at the span given by `span_provider`.
    pub fn span<P: SpanProvider, PRef: AsRef<P>>(
        mut self, span_provider: PRef
    ) -> Self {
        self.error.span = Some(span_provider.as_ref().span());
        self
    }

    /// Identifies the error as having no location.
    pub fn without_loc(mut self) -> Self {
        self.error.span = None;
        self
    }

    /// Uses the main error description `message`.
    pub fn message<S: AsRef<str>>(mut self, message: S) -> Self {
        self.error.message = message.as_ref().to_string();
        self
    }

    /// Marks this error as without a message and instead continuing a previous
    /// error.
    ///
    /// Requires: the error is of secondary style, which must be set before this
    /// function is called (see [`ErrorBuilder::of_style`]).
    pub fn continues(self) -> Self {
        assert!(self.error.style == Style::Secondary);
        self.message("   ...")
    }

    /// Uses an explanatory note `explain`.
    pub fn explain<S: AsRef<str>>(mut self, explain: S) -> Self {
        self.error.explain = Some(explain.as_ref().to_string());
        self
    }

    /// Uses a recommendation `fix`.
    pub fn fix<S: AsRef<str>>(mut self, fix: S) -> Self {
        self.error.fix = Some(fix.as_ref().to_string());
        self
    }

    pub fn maybe_fix<S: AsRef<str>>(mut self, fix: Option<S>) -> Self {
        self.error.fix = fix.map(|str| str.as_ref().to_string());
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
    /// Constructs an error manager that can record up to `max_count` primary
    /// errors.
    pub fn with_max_count(max_count: usize) -> ErrorManager {
        ErrorManager {
            max_count,
            primary_count: 0,
            errors: vec![]
        }
    }

    /// Whether the error manager contains any recorded items, not just primary
    /// errors.
    pub fn has_items(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Whether the error manager has recorded a primary error.
    pub fn has_errors(&self) -> bool {
        self.primary_count > 0
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
        let mut primary_level = Level::Error;
        let mut primary_code = ErrorCode::WompWomp;
        for (i, error) in self.errors.iter().enumerate() {
            if error.style == Style::Primary {
                primary_level = error.level;
                primary_code = error.code;
            }
            if error.style == Style::Primary && i > 0 {
                writeln!(output)?;
            }
            writeln!(output, "{}", error)?;
            if primary_level == Level::Error
                && (i + 1 == self.errors.len()
                    || self.errors[i + 1].style == Style::Primary)
                && primary_code != ErrorCode::WompWomp
            {
                writeln!(
                    output,
                    "For more information, pass `--explain {}`",
                    primary_code
                )?;
            }
        }
        self.errors.clear();
        self.primary_count = 0;
        Ok(())
    }
}
