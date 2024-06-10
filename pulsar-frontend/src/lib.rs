//! This crate defines the lexer, parser, and abstract syntax tree for pulsar.
//!
//! Copyright (C) 2024 Ethan Uppal. All rights reserved.

pub mod ast;
pub mod attribute;
pub mod lexer;
pub mod op;
pub mod parser;
pub mod static_analysis;
pub mod token;
pub mod ty;
