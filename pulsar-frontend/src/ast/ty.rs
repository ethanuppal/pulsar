//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of the
//! License, or (at your option) any later version.

use super::{
    node::{AsNodePool, Node},
    pretty_print::PrettyPrint
};
use inform::fmt::IndentFormatter;
use pulsar_utils::{id::Id, pool::Handle};
use std::{
    cmp,
    fmt::{self, Display, Write},
    mem
};

/// This isn't a real liquid type. Notably, the only constraint it can
/// express is equality to a given number.
#[derive(PartialEq, Eq)]
pub enum LiquidTypeValue {
    Equal(usize),
    All
}

pub type LiquidType = Node<LiquidTypeValue, ()>;

impl PartialEq for LiquidType {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl Eq for LiquidType {}

impl PartialOrd for LiquidType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(match (&self.value, &other.value) {
            (LiquidTypeValue::Equal(a), LiquidTypeValue::Equal(b)) => {
                a.partial_cmp(b)?
            }
            (LiquidTypeValue::Equal(_), LiquidTypeValue::All) => {
                cmp::Ordering::Less
            }
            (LiquidTypeValue::All, LiquidTypeValue::Equal(_)) => {
                cmp::Ordering::Greater
            }
            (LiquidTypeValue::All, LiquidTypeValue::All) => cmp::Ordering::Equal
        })
    }
}

impl PrettyPrint for LiquidType {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        match self.value {
            LiquidTypeValue::Equal(value) => write!(f, "{}", value),
            LiquidTypeValue::All => write!(f, "?")
        }
    }
}

impl Display for LiquidType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TypeValue {
    Unit,
    Var(Id),
    Name(String),
    Int64,

    /// A negative size indicates that the size is not yet known.
    Array(Handle<Type>, Handle<LiquidType>),

    Function {
        inputs: Vec<Handle<Type>>,
        outputs: Vec<Handle<Type>>
    }
}

impl Type {
    /// The number of bytes to store one instance of a value of the current
    /// type.
    pub fn size(&self) -> usize {
        match self.value {
            TypeValue::Unit => 0,
            TypeValue::Var(_) => {
                panic!("Type::Var should have been resolved by type inference")
            }
            TypeValue::Name(_) => {
                todo!("Need to figure out user-defined types")
            }
            TypeValue::Int64 => 8,
            TypeValue::Array(element_type, element_count) => {
                let LiquidTypeValue::Equal(element_count) = element_count.value
                else {
                    panic!("Size not resolved to single value")
                };
                element_type.size() * element_count
            }
            TypeValue::Function {
                inputs: _,
                outputs: _
            } => 8
        }
    }

    pub fn mangle(&self) -> String {
        match &self.value {
            TypeValue::Var(_) => panic!("cannot mangle type var"),
            TypeValue::Unit => "u".into(),
            TypeValue::Name(name) => format!("{}{}", name.len(), name),
            TypeValue::Int64 => "q".into(),
            TypeValue::Array(element_type, element_count) => {
                format!("A{}{}", element_count, element_type)
            }
            TypeValue::Function { inputs, outputs } => format!(
                "F{}{}{}{}",
                inputs.len(),
                inputs
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(""),
                outputs.len(),
                outputs
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(""),
            )
        }
    }

    pub fn can_unify_with(&self, other: &Self) -> bool {
        mem::discriminant(&self.value) == mem::discriminant(&other.value)
    }

    pub fn subterms(&self) -> Vec<Handle<Type>> {
        match &self.value {
            TypeValue::Unit
            | TypeValue::Var(_)
            | TypeValue::Name(_)
            | TypeValue::Int64 => Vec::new(),
            TypeValue::Array(element_type, _) => vec![*element_type],
            TypeValue::Function { inputs, outputs } => {
                let mut result = inputs.clone();
                result.extend(outputs);
                result
            }
        }
    }

    pub fn liquid_subterms(&self) -> Vec<Handle<LiquidType>> {
        match self.value {
            TypeValue::Unit
            | TypeValue::Var(_)
            | TypeValue::Name(_)
            | TypeValue::Int64
            | TypeValue::Function { .. } => Vec::new(),
            TypeValue::Array(_, element_count) => vec![element_count]
        }
    }
}

pub type Type = Node<TypeValue, ()>;

impl PrettyPrint for Type {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        match &self.value {
            TypeValue::Unit => write!(f, "Unit"),
            TypeValue::Var(var) => write!(f, "'t{}", var),
            TypeValue::Name(name) => write!(f, "{}", name),
            TypeValue::Int64 => write!(f, "Int64"),
            TypeValue::Array(element_type, element_count) => {
                write!(f, "{}[{}]", element_type, element_count)
            }
            TypeValue::Function { inputs, outputs } => write!(
                f,
                "({}) -> ({})",
                inputs
                    .iter()
                    .map(|input| input.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                outputs
                    .iter()
                    .map(|outputs| outputs.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Type {}

pub trait AsTypePool: AsNodePool<Type> + AsNodePool<LiquidType> {}