use crate::frontend::ty::Type;
use std::fmt::{Display, Formatter};

pub enum LabelName {
    Native(String, Vec<Type>, Box<Type>)
}

impl LabelName {
    fn mangle(&self) -> String {
        let mut result = String::new();
        match &self {
            Self::Native(name, args, ret) => {
                result.push_str("_pulsar");
                result.push_str(&format!("_S{}", name));
                for arg in args {
                    result.push_str(&format!("_{}", arg.mangle()));
                }
                result.push_str(&format!("_{}", ret.mangle()));
            }
        }
        result
    }
}

impl Display for LabelName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Native(name, _, _) => {
                write!(f, "@native({})", name)?;
            }
        }
        Ok(())
    }
}

pub struct Label {
    pub name: LabelName,
    pub is_external: bool,
    pub is_global: bool
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_external {
            write!(f, "extern ")?;
        } else if self.is_global {
            write!(f, "public ")?;
        } else {
            write!(f, "private ")?;
        }
        write!(f, "{}", self.name)
    }
}
