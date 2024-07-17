//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use inform::fmt::IndentFormatter;
use pulsar_utils::format::INDENT_WIDTH;
use std::fmt::{self, Display};

/// Implements [`Display`] for a [`PrettyPrint`] type `P`.
struct PrettyBox<'a, P: PrettyPrint> {
    value: &'a P
}

impl<'a, P: PrettyPrint> From<&'a P> for PrettyBox<'a, P> {
    fn from(value: &'a P) -> Self {
        Self { value }
    }
}

impl<'a, P: PrettyPrint> Display for PrettyBox<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self.value, f)
    }
}

/// An AST node that can be pretty-printed.
///
/// # Display
///
/// You can implement [`Display`] for a [`PrettyPrint`] type `T` with:
/// ```
/// impl Display for T {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         PrettyPrint::fmt(self, f)
///     }
/// }
/// ```
pub trait PrettyPrint {
    /// Formats the current node using the given indent formatter `f`.
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result;

    /// Formats the current node using the given formatter `f`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = IndentFormatter::new(f, INDENT_WIDTH);
        self.pretty_print(&mut f)
    }

    /// Yields the string representation of this node.
    fn to_string(&self) -> String
    where
        Self: Sized {
        PrettyBox::from(self).to_string()
    }
}
