//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use anyhow::anyhow;
use pulsar_frontend::{
    ast::{
        node::{AsNodePool, NodeInterface},
        ty::{AsTypePool, LiquidTypeValue, Type, TypeValue}
    },
    token::{Token, TokenType}
};
use pulsar_utils::{
    pool::{AsPool, Handle},
    span::Loc
};
use std::fmt::{self, Display, Write};

/// If one exists, the start symbol for a pulsar program is guaranteed to begin
/// with the following.
pub const MAIN_SYMBOL_PREFIX: &str = "pulsar_SF4main";

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
            "pulsar_SF{}{}",
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

    pub fn from<S: AsRef<str>>(value: S) -> Self {
        Self {
            unmangled: value.as_ref().to_string(),
            mangled: value.as_ref().to_string(),
            is_native: false
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

pub struct Demangler<'pool, P: AsTypePool + AsPool<Token, ()>> {
    pos: usize,
    buffer: Vec<char>,
    pool: &'pool mut P,
    fake_token: Handle<Token>
}

impl<'pool, P: AsTypePool + AsPool<Token, ()>> Demangler<'pool, P> {
    pub fn new(label: String, pool: &'pool mut P) -> Self {
        let fake_token = pool.add(Token {
            ty: TokenType::Identifier,
            value: String::new(),
            loc: Loc::make_invalid()
        });
        Self {
            pos: 0,
            buffer: label.chars().collect(),
            pool,
            fake_token
        }
    }

    fn ty<N: NodeInterface<V = V, T = ()>, V>(&mut self, ty: V) -> Handle<N>
    where
        P: AsNodePool<N> {
        self.pool.generate(ty, self.fake_token, self.fake_token)
    }

    fn current(&self) -> Option<char> {
        if self.pos >= self.buffer.len() {
            None
        } else {
            Some(self.buffer[self.pos])
        }
    }

    fn take(&mut self) -> Option<char> {
        self.current().map(|c| {
            self.pos += 1;
            c
        })
    }

    fn unget(&mut self) {
        self.pos -= 1;
    }

    fn take_n(&mut self, mut n: usize) -> Option<String> {
        let mut result = String::new();
        while n > 0 {
            result.push(self.take()?);
            n -= 1;
        }
        Some(result)
    }

    fn take_number(&mut self) -> anyhow::Result<usize> {
        let mut number_str = String::new();

        while let Some(c) = self.take() {
            if c.is_ascii_digit() {
                number_str.push(c);
            } else {
                self.unget();
                break;
            }
        }

        if number_str.is_empty() {
            Err(anyhow!("No number found at the start of the string"))
        } else {
            Ok(number_str.parse()?)
        }
    }

    fn take_prefix<S: AsRef<str>>(&mut self, prefix: S) -> anyhow::Result<()> {
        for char in prefix.as_ref().chars() {
            match self.take() {
                None => {
                    return Err(anyhow!("EOF"));
                }
                Some(next) if next == char => {}
                _ => {
                    return Err(anyhow!("Mismatch"));
                }
            }
        }
        Ok(())
    }

    fn demangle_named_type(&mut self) -> anyhow::Result<Handle<Type>> {
        let length = self.take_number()?;
        let name = self.take_n(length).ok_or(anyhow!("Failed to get name"))?;
        Ok(self.ty(TypeValue::Name(name)))
    }
    fn demangle_array_type(&mut self) -> anyhow::Result<Handle<Type>> {
        let length = self.take_number()?;
        let length = self.ty(LiquidTypeValue::Equal(length));
        if self.take() != Some('E') {
            return Err(anyhow!("Missing element type"))?;
        }
        let element = self.demangle_type()?;
        Ok(self.ty(TypeValue::Array(element, length)))
    }

    fn demangle_type(&mut self) -> anyhow::Result<Handle<Type>> {
        let ty = match self.take() {
            Some('u') => self.ty(TypeValue::Unit),
            Some('q') => self.ty(TypeValue::Int64),
            Some('A') => self.demangle_array_type()?,
            _ => self.demangle_named_type()?
        };
        Ok(ty)
    }

    fn demangle_function(&mut self) -> anyhow::Result<Handle<Type>> {
        let length = self.take_number()?;
        self.take_n(length)
            .ok_or(anyhow!("Failed to get function name"))?;

        let input_count = self.take_number()?;
        let mut inputs = Vec::new();
        for _ in 0..input_count {
            inputs.push(self.demangle_type()?);
        }
        let output_count = self.take_number()?;
        let mut outputs = Vec::new();
        for _ in 0..output_count {
            outputs.push(self.demangle_type()?);
        }
        Ok(self.ty(TypeValue::Function { inputs, outputs }))
    }

    fn demangle_symbol(&mut self) -> anyhow::Result<Handle<Type>> {
        match self.take() {
            Some('F') => self.demangle_function(),
            a => Err(anyhow!("Unknown symbol {:?}", a))
        }
    }

    pub fn demangle(&mut self) -> anyhow::Result<Handle<Type>> {
        self.take_prefix("pulsar_")?;
        match self.take() {
            Some('S') => self.demangle_symbol(),
            _ => Err(anyhow!("Unknown mangled value"))
        }
    }
}
