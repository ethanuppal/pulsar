//! This crate defines the lexer, parser, and abstract syntax tree for pulsar.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

pub mod ast;
pub mod attribute;
pub mod lexer;
pub mod op;
pub mod parser;
pub mod token;
pub mod type_inferer;
