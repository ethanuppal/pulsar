use crate::frontend::ty::Type;
use std::fmt::{Display, Formatter};

pub enum LabelName {
    Native(String)
}

impl LabelName {
    pub fn mangle(&self, args: &Vec<Type>, ret: &Box<Type>) -> String {
        let mut result = String::new();
        match &self {
            Self::Native(name) => {
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
            Self::Native(name) => {
                write!(f, "@native({})", name)?;
            }
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
