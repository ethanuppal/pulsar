use inform::fmt::IndentFormatter;
use pulsar_utils::id::Id;
use std::fmt::Write;

use super::{
    node::{AsNodePool, Handle, Node},
    pretty_print::{PrettyPrint, PrettyPrinter},
    AsASTPool
};

pub const ARRAY_TYPE_UNKNOWN_SIZE: isize = -1;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum TypeValue {
    Unit,
    Var(Id),
    Name(String),
    Int64,

    /// A negative size indicates that the size is not yet known.
    Array(Handle<Type>, isize),

    Function {
        is_pure: bool,
        args: Vec<Handle<Type>>,
        ret: Handle<Type>
    }
}

impl Type {
    /// The number of bytes to store one instance of a value of the current
    /// type.
    pub fn size<Pool: AsNodePool<Type>>(&self, pool: &Pool) -> usize {
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
                pool.get(element_type).size(pool) * (element_count as usize)
            }
            TypeValue::Function {
                is_pure: _,
                args: _,
                ret: _
            } => 8
        }
    }

    // pub fn as_array_type(&self) -> (TypeCell, isize) {
    //     match &self {
    //         Self::Array(element_type, size) => (element_type.clone(), *size),
    //         _ => panic!(
    //             "{}",
    //             format!(
    //                 "Type::as_array_type called on non-array type `{}`",
    //                 &self
    //             )
    //         )
    //     }
    // }

    pub fn mangle<Pool: AsNodePool<Type> + PrettyPrinter<Type>>(
        &self, pool: &Pool
    ) -> String {
        match &self.value {
            TypeValue::Var(_) => panic!("cannot mangle type var"),
            TypeValue::Unit => "u".into(),
            TypeValue::Name(name) => format!("{}{}", name.len(), name),
            TypeValue::Int64 => "q".into(),
            TypeValue::Array(element_type, count) => {
                format!("A{}{}", count, pool.fmtr(*element_type))
            }
            TypeValue::Function {
                is_pure: _,
                args: _,
                ret: _
            } => todo!()
        }
    }
}

pub type Type = Node<TypeValue, ()>;

impl PrettyPrint for Type {
    fn fmt<P: AsASTPool>(
        &self, f: &mut IndentFormatter<'_, '_>, ast_pool: &P
    ) -> core::fmt::Result {
        match &self.value {
            TypeValue::Unit => write!(f, "Unit"),
            TypeValue::Var(var) => write!(f, "'t{}", var),
            TypeValue::Name(name) => write!(f, "{}", name),
            TypeValue::Int64 => write!(f, "Int64"),
            TypeValue::Array(element_ty, size) => write!(
                f,
                "{}[{}]",
                ast_pool.fmtr(*element_ty),
                if *size == ARRAY_TYPE_UNKNOWN_SIZE {
                    "?".into()
                } else {
                    size.to_string()
                }
            ),
            TypeValue::Function { is_pure, args, ret } => write!(
                f,
                "{}({}) -> {}",
                if *is_pure { "pure " } else { "" },
                args.iter()
                    .map(|ty| ast_pool.to_string(*ty))
                    .collect::<Vec<_>>()
                    .join(", "),
                ast_pool.fmtr(*ret),
            )
        }
    }
}
