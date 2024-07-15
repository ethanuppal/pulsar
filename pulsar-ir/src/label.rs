// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use std::fmt::{Display, Formatter};

use pulsar_frontend::ast::ty::Type;
use pulsar_utils::pool::Handle;

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
        value: S, args: &[Handle<Type>], ret: Handle<Type>
    ) -> Self {
        let mut mangled = String::new();
        mangled.push_str("_pulsar");
        mangled.push_str(&format!("_S{}", value.as_ref()));
        for arg in args {
            mangled.push_str(&format!("_{}", arg.mangle()));
        }
        mangled.push_str(&format!("_{}", ret.mangle()));
        Self {
            unmangled: value.as_ref().to_string(),
            mangled,
            is_native: true
        }
    }

    pub fn mangle(&self) -> &String {
        &self.mangled
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.visibility, self.name)
    }
}
