// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use pulsar_frontend::ty::Type;
use std::fmt::{Display, Formatter};

pub struct LabelName {
    unmangled: String,
    mangled: String,
    is_native: bool
}

impl LabelName {
    pub fn from_native(
        value: String, args: &Vec<Type>, ret: &Box<Type>
    ) -> Self {
        let mut mangled = String::new();
        mangled.push_str("_pulsar");
        mangled.push_str(&format!("_S{}", value));
        for arg in args {
            mangled.push_str(&format!("_{}", arg.mangle()));
        }
        mangled.push_str(&format!("_{}", ret.mangle()));
        Self {
            unmangled: value,
            mangled,
            is_native: true
        }
    }

    pub fn mangle(&self) -> &String {
        &self.mangled
    }
}

impl Display for LabelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_native {
            write!(f, "@native({})", self.unmangled)?;
        } else {
            self.mangled.fmt(f)?;
        }

        Ok(())
    }
}

pub enum LabelVisibility {
    Public,
    Private,
    External
}

impl Display for LabelVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            LabelVisibility::Public => "public",
            LabelVisibility::Private => "private",
            LabelVisibility::External => "external"
        }
        .fmt(f)
    }
}

pub struct Label {
    pub name: LabelName,
    pub visibility: LabelVisibility
}

impl Label {
    pub fn from(name: LabelName, visibility: LabelVisibility) -> Label {
        Label { name, visibility }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.visibility, self.name)
    }
}
