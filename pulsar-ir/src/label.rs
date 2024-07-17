//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_frontend::ast::ty::Type;
use pulsar_utils::pool::Handle;
use std::fmt::{self, Display, Formatter, Write};

/// If one exists, the start symbol for a pulsar program is guaranteed to begin
/// with the following.
pub const MAIN_SYMBOL_PREFIX: &str = "_pulsar_Smain";

pub struct Name {
    unmangled: String,
    mangled: String,
    is_native: bool
}

impl Name {
    pub fn from_native<S: AsRef<str>>(
        value: S, inputs: &[Handle<Type>], outputs: &[Handle<Type>]
    ) -> Self {
        let mut mangled = String::new();
        write!(
            &mut mangled,
            "_pulsar_SF{}{}",
            value.as_ref().len(),
            value.as_ref()
        )
        .unwrap();
        write!(&mut mangled, "{}", inputs.len()).unwrap();
        for input in inputs {
            write!(&mut mangled, "{}", input.mangle()).unwrap();
        }
        write!(&mut mangled, "{}", outputs.len()).unwrap();
        for output in outputs {
            write!(&mut mangled, "{}", output.mangle()).unwrap();
        }
        Self {
            unmangled: value.as_ref().to_string(),
            mangled,
            is_native: true
        }
    }

    pub fn unmangled(&self) -> &str {
        &self.unmangled
    }

    pub fn mangled(&self) -> &str {
        &self.mangled
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_native {
            write!(f, "native {}", self.unmangled)?;
        } else {
            self.mangled.fmt(f)?;
        }

        Ok(())
    }
}

pub enum Visibility {
    Public,
    Private,
    External
}

impl Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Visibility::Public => "public",
            Visibility::Private => "private",
            Visibility::External => "external"
        }
        .fmt(f)
    }
}

pub struct Label {
    pub name: Name,
    pub visibility: Visibility
}

impl Label {
    pub fn from(name: Name, visibility: Visibility) -> Label {
        Label { name, visibility }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.visibility, self.name)
    }
}
