// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use core::fmt;
use inform::fmt::IndentFormatter;
use pulsar_utils::format::INDENT_WIDTH;

/// An AST node that can be pretty-printed.
///
/// # `Display``
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
    /// Formats the current node using the given formatter `f` and the AST
    /// context `ast`.
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result;

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = IndentFormatter::new(f, INDENT_WIDTH);
        self.pretty_print(&mut f)
    }
}
