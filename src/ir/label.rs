use crate::frontend::ty::Type;
use std::fmt::Display;

pub enum LabelName {
    Native(String, Vec<Type>, Box<Type>)
}

impl Display for LabelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Native(name, args, ret) => {
                write!(f, "_pulsar")?;
                write!(f, "_S{}", name)?;
                for arg in args {
                    write!(f, "_{}", arg.mangle())?;
                }
                write!(f, "_{}", ret.mangle())?;
            }
        }
        Ok(())
    }
}

pub struct Label {
    name: LabelName,
    is_external: bool,
    is_global: bool
}
