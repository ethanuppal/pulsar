//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{operand::Operand, variable::Variable, Ir};
use inform::fmt::IndentFormatter;
use pulsar_frontend::ast::pretty_print::PrettyPrint;
use pulsar_utils::{
    id::{Gen, Id},
    pool::{AsPool, Handle}
};
use std::{
    fmt::{self, Display, Write},
    mem, vec
};

pub struct For {
    pub id: Id,
    variant: Variable,
    lower: Operand,
    upper: Operand,
    pub(crate) body: Handle<Control>
}

impl For {
    pub fn new(
        id: Id, variant: Variable, lower: Operand, upper: Operand,
        body: Handle<Control>
    ) -> Self {
        Self {
            id,
            variant,
            lower,
            upper,
            body
        }
    }
}

impl PrettyPrint for For {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        writeln!(
            f,
            "for {} in {} ..< {} {{",
            self.variant, self.lower, self.upper
        )?;
        f.increase_indent();
        self.body.pretty_print(f)?;
        f.decrease_indent();
        write!(f, "\n}}")
    }
}

impl Display for For {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}

pub struct Seq {
    pub id: Id,
    pub(crate) children: Vec<Handle<Control>>
}

impl Seq {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            children: Vec::new()
        }
    }

    pub fn push(&mut self, child: Handle<Control>) {
        self.children.push(child);
    }
}

impl PrettyPrint for Seq {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        writeln!(f, "seq {{")?;
        f.increase_indent();
        for child in &self.children {
            child.pretty_print(f)?;
            writeln!(f)?;
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

pub struct Par {
    pub id: Id,
    pub(crate) children: Vec<Handle<Control>>
}

impl Par {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            children: Vec::new()
        }
    }

    pub fn singleton(id: Id, child: Handle<Control>) -> Self {
        let mut new_self = Self::new(id);
        new_self.push(child);
        new_self
    }

    pub fn push(&mut self, child: Handle<Control>) {
        self.children.push(child);
    }
}

impl PrettyPrint for Par {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        writeln!(f, "par {{")?;
        f.increase_indent();
        for child in &self.children {
            child.pretty_print(f)?;
            writeln!(f)?;
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
    pub id: Id,
    cond: Operand,
    pub(crate) true_branch: Handle<Control>,
    pub(crate) false_branch: Handle<Control>
}

impl IfElse {
    pub fn new(
        id: Id, cond: Operand, true_branch: Handle<Control>,
        false_branch: Handle<Control>
    ) -> Self {
        Self {
            id,
            cond,
            true_branch,
            false_branch
        }
    }
}

impl PrettyPrint for IfElse {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        writeln!(f, "if {} {{", self.cond)?;
        f.increase_indent();
        self.true_branch.pretty_print(f)?;
        f.decrease_indent();
        writeln!(f, "\n}} else {{")?;
        f.increase_indent();
        self.false_branch.pretty_print(f)?;
        f.decrease_indent();
        write!(f, "\n}}")
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

pub const DEFAULT_CONTROL_ID: Id = 0;

impl Control {
    pub fn id(&self) -> Id {
        match self {
            Control::Empty | Control::Enable(_) => DEFAULT_CONTROL_ID,
            Control::For(for_) => for_.id,
            Control::Seq(seq) => seq.id,
            Control::Par(par) => par.id,
            Control::IfElse(if_else) => if_else.id
        }
    }
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

pub struct SeqParBuilder<'gen, 'pool, P: AsControlPool> {
    gen: &'gen mut Gen,
    pool: &'pool mut P,
    /// inv: `pars.len() >= 1`.
    pars: Vec<Par>
}

impl<'gen, 'pool, P: AsControlPool> SeqParBuilder<'gen, 'pool, P> {
    pub fn new(gen: &'gen mut Gen, pool: &'pool mut P) -> Self {
        let par = Par::new(gen.next());
        Self {
            gen,
            pool,
            pars: vec![par]
        }
    }

    pub fn push<C: Into<Control>>(&mut self, control: C) {
        self.pars
            .last_mut()
            .unwrap()
            .push(self.pool.add(control.into()));
    }

    pub fn split(&mut self) {
        self.pars.push(Par::new(self.gen.next()));
    }

    pub fn with_inner<R, F: FnOnce(&mut Gen, &mut P) -> R>(
        &mut self, f: F
    ) -> R {
        f(self.gen, self.pool)
    }

    pub fn next_id(&mut self) -> Id {
        self.gen.next()
    }
}

impl<'gen, 'pool, P: AsControlPool> From<SeqParBuilder<'gen, 'pool, P>>
    for Control
{
    fn from(mut value: SeqParBuilder<P>) -> Self {
        if value.pars.len() == 1 {
            let mut result = Par::new(0);
            mem::swap(&mut value.pars[0], &mut result);
            Self::Par(result)
        } else {
            let mut seq = Seq::new(value.gen.next());
            for par in value.pars {
                seq.push(value.pool.add(Self::Par(par)));
            }
            Self::Seq(seq)
        }
    }
}
