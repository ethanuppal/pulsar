//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of the
//! License, or (at your option) any later version.

use crate::{operand::Operand, variable::Variable, Ir};
use inform::fmt::IndentFormatter;
use pulsar_frontend::ast::pretty_print::PrettyPrint;
use pulsar_utils::pool::{AsPool, Handle};
use std::{
    default,
    fmt::{self, Display, Write},
    mem, vec
};

pub struct For {
    variant: Variable,
    lower: usize,
    upper: usize,
    body: Handle<Control>
}

impl PrettyPrint for For {
    fn pretty_print(
        &self, f: &mut IndentFormatter<'_, '_>
    ) -> fmt::Result {
        writeln!(
            f,
            "for {} in {} ..< {} {{",
            self.variant, self.lower, self.upper
        )?;
        f.increase_indent();
        self.body.pretty_print(f)?;
        f.decrease_indent();
        write!(f, "}}")
    }
}

impl Display for For {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}

#[derive(Default)]
pub struct Seq {
    children: Vec<Handle<Control>>
}

impl Seq {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<C: Into<Control>, Metadata, P: AsPool<Control, Metadata>>(
        &mut self, child: C, pool: &mut P
    ) {
        self.children.push(pool.add(child.into()));
    }
}

impl PrettyPrint for Seq {
    fn pretty_print(
        &self, f: &mut IndentFormatter<'_, '_>
    ) -> fmt::Result {
        writeln!(f, "seq {{")?;
        f.increase_indent();
        for child in &self.children {
            child.pretty_print(f)?;
        }
        f.decrease_indent();
        write!(f, "}}")
    }
}

impl Display for Seq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}

#[derive(Default)]
pub struct Par {
    children: Vec<Handle<Control>>
}

impl Par {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<C: Into<Control>, Metadata, P: AsPool<Control, Metadata>>(
        &mut self, child: C, pool: &mut P
    ) {
        self.children.push(pool.add(child.into()));
    }
}

impl PrettyPrint for Par {
    fn pretty_print(
        &self, f: &mut IndentFormatter<'_, '_>
    ) -> fmt::Result {
        writeln!(f, "par {{")?;
        f.increase_indent();
        for child in &self.children {
            child.pretty_print(f)?;
        }
        f.decrease_indent();
        write!(f, "}}")
    }
}

impl Display for Par {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}

pub struct IfElse {
    cond: Operand,
    true_branch: Handle<Control>,
    false_branch: Handle<Control>
}

impl IfElse {
    pub fn new(
        cond: Operand, true_branch: Handle<Control>,
        false_branch: Handle<Control>
    ) {
    }
}

impl PrettyPrint for IfElse {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        writeln!(f, "if {} {{", self.cond)?;
        f.increase_indent();
        self.true_branch.pretty_print(f)?;
        f.decrease_indent();
        writeln!(f, "}} else {{")?;
        f.increase_indent();
        self.false_branch.pretty_print(f)?;
        f.decrease_indent();
        write!(f, "}}")
    }
}

impl Display for IfElse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}

#[derive(Default)]
pub enum Control {
    #[default]
    Empty,
    For(For),
    Seq(Seq),
    Par(Par),
    IfElse(IfElse),
    Enable(Ir)
}

impl PrettyPrint for Control {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        match self {
            Control::Empty => Ok(()),
            Control::For(for_) => for_.pretty_print(f),
            Control::Seq(seq) => seq.pretty_print(f),
            Control::Par(par) => par.pretty_print(f),
            Control::IfElse(if_else) => if_else.pretty_print(f),
            Control::Enable(ir) => write!(f, "{}", ir)
        }
    }
}

impl Display for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}

impl From<Ir> for Control {
    fn from(value: Ir) -> Self {
        Self::Enable(value)
    }
}

pub trait AsControlPool: AsPool<Control, usize> {}

pub struct SeqParBuilder<'pool, P: AsControlPool> {
    pool: &'pool mut P,
    /// inv: `pars.len() >= 1`.
    pars: Vec<Par>
}

impl<'pool, P: AsControlPool> SeqParBuilder<'pool, P> {
    pub fn new(pool: &'pool mut P) -> Self {
        Self {
            pool,
            pars: vec![Par::new()]
        }
    }

    pub fn push<C: Into<Control>>(&mut self, control: C) {
        self.pars.last_mut().unwrap().push(control, self.pool);
    }

    pub fn split(&mut self) {
        self.pars.push(Par::new());
    }
}

impl<'pool, P: AsControlPool> From<SeqParBuilder<'pool, P>> for Control {
    fn from(mut value: SeqParBuilder<P>) -> Self {
        if value.pars.len() == 1 {
            Self::Par(mem::take(&mut value.pars[0]))
        } else {
            let mut seq = Seq::new();
            for par in value.pars {
                seq.push(Self::Par(par), value.pool);
            }
            Self::Seq(seq)
        }
    }
}
