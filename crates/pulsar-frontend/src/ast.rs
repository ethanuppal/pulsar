// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use super::{
    token::{Token, TokenType},
    ty::{StmtTypeCell, Type, TypeCell}
};
use crate::{
    attribute::{Attribute, Attributes},
    ty::StmtType
};
use pulsar_utils::{
    format,
    loc::{Loc, RegionProvider}
};
use std::fmt::{self, Display};

pub type Param = (Token, Type);

pub(crate) trait TokenRegionProvider {
    fn start_token(&self) -> &Token;
    fn end_token(&self) -> &Token;
}

macro_rules! implement_region_provider_for_token_provider {
    ($T:ident) => {
        impl RegionProvider for $T {
            fn start(&self) -> Loc {
                self.start_token().loc.clone()
            }

            fn end(&self) -> Loc {
                let end_token = self.end_token();
                let mut loc = end_token.loc.clone();
                // tokens are always on one line
                if end_token.ty != TokenType::Newline {
                    let length = end_token.value.len() as isize;
                    loc.pos += length;
                    loc.col += length;
                }
                loc
            }
        }
    };
}

#[derive(Clone)]
pub enum ExprValue {
    ConstantInt(i64),
    /// TODO: Support `::`s
    BoundName(Token),

    /// TODO: Call an `expr` or some sort of chaining of `::`
    Call(Token, Vec<Expr>),

    Subscript(Box<Expr>, Box<Expr>),

    /// `ArrayLiteral(elements, should_continue)` is an array literal beginning
    /// with `elements` and filling the remainder of the array with zeros if
    /// `should_continue`.
    ArrayLiteral(Vec<Expr>, bool),

    PrefixOp(Token, Box<Expr>),
    BinOp(Box<Expr>, Token, Box<Expr>),

    /// `HardwareMap(map_token, parallel_factor, f, arr)` is an array produced
    /// by applying `f` elementwise to `arr` using a hardware parallelism
    /// factor of `parallel_factor`.
    HardwareMap(Token, usize, Token, Box<Expr>)
}

#[derive(Clone)]
pub struct Expr {
    pub value: ExprValue,
    pub ty: TypeCell,
    start_token: Token,
    end_token: Token
}

impl Expr {
    /// Constructs a new expression with the given `value` that ranges from
    /// `start_token` to `end_token`.
    pub fn new(value: ExprValue, start_token: Token, end_token: Token) -> Self {
        Self {
            value,
            start_token,
            end_token,
            ty: TypeCell::new(Type::Unknown)
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ExprValue::ConstantInt(i) => {
                write!(f, "{}", i)?;
            }
            ExprValue::BoundName(name) => {
                write!(f, "{}", name.value)?;
            }
            ExprValue::Call(name, args) => {
                write!(
                    f,
                    "{}({})",
                    name.value,
                    args.iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
            ExprValue::Subscript(array, index) => {
                write!(f, "{}[{}]", array, index)?;
            }
            ExprValue::ArrayLiteral(elements, should_continue) => {
                write!(
                    f,
                    "[{}{}]",
                    elements
                        .iter()
                        .map(|ty| ty.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    if *should_continue {
                        format!(
                            "{}...",
                            if elements.is_empty() { "" } else { ", " }
                        )
                    } else {
                        "".into()
                    }
                )?;
            }
            ExprValue::PrefixOp(op, rhs) => {
                write!(f, "({} {})", op.value, rhs)?;
            }
            ExprValue::BinOp(lhs, op, rhs) => {
                write!(f, "({} {} {})", lhs, op.value, rhs)?;
            }
            ExprValue::HardwareMap(_, parallel_factor, fun, arr) => {
                write!(f, "map<{}>({}, {})", parallel_factor, fun.value, arr)?;
            }
        }

        let expr_ty = self.ty.as_ref();
        if expr_ty.clone().is_known() {
            write!(f, ": {}", expr_ty)?;
        }

        Ok(())
    }
}

impl TokenRegionProvider for Expr {
    fn start_token(&self) -> &Token {
        &self.start_token
    }

    fn end_token(&self) -> &Token {
        &self.end_token
    }
}

implement_region_provider_for_token_provider!(Expr);

#[derive(Clone)]
pub enum NodeValue {
    Function {
        name: Token,
        params: Vec<Param>,
        ret: Type,
        pure_token: Option<Token>,
        body: Vec<Node>
    },
    LetBinding {
        name: Token,
        hint: Option<TypeCell>,
        value: Box<Expr>
    },
    Return {
        keyword_token: Token,
        value: Option<Box<Expr>>
    }
}

#[derive(Clone)]
pub struct Node {
    pub value: NodeValue,
    pub ty: StmtTypeCell,
    pub attributes: Attributes,
    start_token: Token,
    end_token: Token
}

impl Node {
    /// Constructs a node node with the given `value` that ranges from
    /// `start_token` to `end_token`.
    pub fn new(value: NodeValue, start_token: Token, end_token: Token) -> Self {
        Self {
            value,
            ty: StmtType::make_unknown(),
            attributes: Attributes::default(),
            start_token,
            end_token
        }
    }

    /// Marks this node as generated (that is, not present syntactically in the
    /// user's source).
    pub fn mark_generated(mut self) -> Self {
        self.attributes.add(Attribute::Generated);
        self
    }

    /// Pretty-prints this node at the given indentation `level`.
    fn pretty(&self, level: usize) -> String {
        let mut result = format::make_indent(level);
        let content = match &self.value {
            NodeValue::Function {
                name,
                params,
                ret,
                pure_token,
                body
            } => {
                let insert_newline = if body.is_empty() { "" } else { "\n" };
                format!(
                    "{}func {}({}) -> {} {{{}{}{}{}}}",
                    if pure_token.is_some() { "pure " } else { "" },
                    name.value,
                    params
                        .iter()
                        .map(|(name, ty)| format!("{}: {}", name.value, ty))
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret,
                    insert_newline,
                    body.iter()
                        .map(|node| { node.pretty(level + 1) })
                        .collect::<Vec<_>>()
                        .join("\n"),
                    insert_newline,
                    format::make_indent(level)
                )
            }
            NodeValue::LetBinding {
                name,
                hint: hint_opt,
                value
            } => {
                let hint_str = if let Some(hint) = hint_opt {
                    format!(": {}", hint)
                } else {
                    "".into()
                };
                format!("let {}{} = {}", name.value, hint_str, value)
            }
            NodeValue::Return {
                keyword_token: _,
                value: value_opt
            } => {
                format!(
                    "return{}",
                    if let Some(value) = value_opt {
                        format!(" {}", value)
                    } else {
                        "".into()
                    }
                )
            }
        };
        result.push_str(&content);
        result
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty(0).fmt(f)
    }
}

impl TokenRegionProvider for Node {
    fn start_token(&self) -> &Token {
        &self.start_token
    }

    fn end_token(&self) -> &Token {
        &self.end_token
    }
}

implement_region_provider_for_token_provider!(Node);
