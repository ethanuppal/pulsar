//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{port::Port, variable::Variable, Ir};
use inform::fmt::IndentFormatter;
use pulsar_frontend::ast::pretty_print::PrettyPrint;
use pulsar_utils::pool::{AsPool, Handle};
use std::{
    fmt::{self, Display, Write},
    mem, vec
};

pub struct For {
    variant: Variable,
    lower: Port,
    exclusive_upper: Port,
    pub(crate) body: Handle<Control>
}

impl For {
    /// A for control takes exclusive ownership of its upper and lower bound
    /// ports, so no handles need to be created for them.
    pub fn new(
        variant: Variable, lower: Port, exclusive_upper: Port,
        body: Handle<Control>
    ) -> Self {
        Self {
            variant,
            lower,
            exclusive_upper,
            body
        }
    }

    pub fn variant(&self) -> Variable {
        self.variant
    }

    pub fn lower_bound(&self) -> &Port {
        &self.lower
    }

    pub fn exclusive_upper_bound(&self) -> &Port {
        &self.exclusive_upper
    }

    pub fn body(&self) -> Handle<Control> {
        self.body
    }

    /// The number of cycles needed to initialize the loop variant to the lower
    /// bound.
    pub fn init_latency(&self) -> usize {
        1
    }
}

impl PrettyPrint for For {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        writeln!(
            f,
            "for {} in {} ..< {} {{",
            self.variant, self.lower, self.exclusive_upper
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

#[derive(Default)]
pub struct Seq {
    pub(crate) children: Vec<Handle<Control>>
}

impl Seq {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, child: Handle<Control>) {
        self.children.push(child);
    }

    pub fn children(&self) -> &[Handle<Control>] {
        &self.children
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

#[derive(Default)]
pub struct Par {
    pub(crate) children: Vec<Handle<Control>>
}

impl Par {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn singleton(child: Handle<Control>) -> Self {
        let mut new_self = Self::new();
        new_self.push(child);
        new_self
    }

    pub fn push(&mut self, child: Handle<Control>) {
        self.children.push(child);
    }

    pub fn children(&self) -> &[Handle<Control>] {
        &self.children
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
    cond: Port,
    pub(crate) true_branch: Handle<Control>,
    pub(crate) false_branch: Handle<Control>
}

impl IfElse {
    pub fn new(
        cond: Port, true_branch: Handle<Control>, false_branch: Handle<Control>
    ) -> Self {
        Self {
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

/// Only [`Seq`]s and [`Par`]s can hold [`Ir`] operations.
#[derive(Default)]
pub enum Control {
    #[default]
    Empty,
    Delay(usize),
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
            Control::Delay(delay) => write!(f, "delay {}", delay),
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

pub struct ControlBuilder<'pool, P: AsControlPool + AsPool<Port, ()>> {
    pool: &'pool mut P,
    /// inv: `pars.len() >= 1`.
    pars: Vec<Par>
}

impl<'pool, P: AsControlPool + AsPool<Port, ()>> ControlBuilder<'pool, P> {
    pub fn new(pool: &'pool mut P) -> Self {
        Self {
            pool,
            pars: vec![Par::new()]
        }
    }

    /// Enables `control` in the current logical time step since initialization
    /// or [`ControlBuilder::split`].
    pub fn push<C: Into<Control>>(&mut self, control: C) {
        self.pars
            .last_mut()
            .unwrap()
            .push(self.pool.add(control.into()));
    }

    /// Marks all later [`ControlBuilder::push`]es as occuring in a subsequent
    /// logical time step.
    pub fn split(&mut self) {
        self.pars.push(Par::new());
    }

    pub fn with_pool<R, F: FnOnce(&mut P) -> R>(&mut self, f: F) -> R {
        f(self.pool)
    }

    /// [`Ir::Add`] followed by [`ControlBuilder::push`].
    pub fn push_add<R: Into<Port>>(
        &mut self, result: R, port: Port, port2: Port
    ) {
        let ir = Ir::Add(
            self.pool.add(result.into()),
            self.pool.add(port),
            self.pool.add(port2)
        );
        self.push(ir);
    }

    /// [`Ir::Mul`] followed by [`ControlBuilder::push`].
    pub fn push_mul<R: Into<Port>>(
        &mut self, result: R, port: Port, port2: Port
    ) {
        let ir = Ir::Mul(
            self.pool.add(result.into()),
            self.pool.add(port),
            self.pool.add(port2)
        );
        self.push(ir);
    }

    /// [`Ir::Assign`] followed by [`ControlBuilder::push`].
    pub fn push_assign<R: Into<Port>>(&mut self, result: R, port: Port) {
        let ir = Ir::Assign(self.pool.add(result.into()), self.pool.add(port));
        self.push(ir);
    }

    pub fn add_port<PortLike: Into<Port>>(
        &mut self, port: PortLike
    ) -> Handle<Port> {
        self.pool.add(port.into())
    }

    pub fn new_const(&mut self, value: i64) -> Handle<Port> {
        self.pool.add(Port::Constant(value))
    }
}

impl<'pool, P: AsControlPool + AsPool<Port, ()>> From<ControlBuilder<'pool, P>>
    for Control
{
    fn from(mut value: ControlBuilder<P>) -> Self {
        if value.pars.len() == 1 {
            let mut result = Par::new();
            mem::swap(&mut value.pars[0], &mut result);
            Self::Par(result)
        } else {
            let mut seq = Seq::new();
            for par in value.pars {
                seq.push(value.pool.add(Self::Par(par)));
            }
            Self::Seq(seq)
        }
    }
}
