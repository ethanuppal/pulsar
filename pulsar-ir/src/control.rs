// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use crate::{operand::Operand, variable::Variable, Ir};
use inform::fmt::IndentFormatter;
use pulsar_frontend::ast::pretty_print::PrettyPrint;
use pulsar_utils::pool::{AsPool, Handle};
use std::fmt::{self, Display, Write};

pub struct For {
    variant: Variable,
    lower: usize,
    upper: usize,
    body: Handle<Control>
}

impl PrettyPrint for For {
    fn pretty_print(
        &self, f: &mut IndentFormatter<'_, '_>
    ) -> core::fmt::Result {
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

pub struct Seq {
    contents: Vec<Handle<Control>>
}

impl Seq {
    pub fn push<C: Into<Control>, Metadata, P: AsPool<Control, Metadata>>(
        &mut self, child: C, pool: &mut P
    ) {
        self.contents.push(pool.add(child.into()));
    }
}

impl PrettyPrint for Seq {
    fn pretty_print(
        &self, f: &mut IndentFormatter<'_, '_>
    ) -> core::fmt::Result {
        writeln!(f, "seq {{")?;
        f.increase_indent();
        for child in &self.contents {
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

pub struct IfElse {
    cond: Operand,
    true_branch: Handle<Control>,
    false_branch: Handle<Control>
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

pub enum Control {
    Empty,
    For(For),
    Seq(Seq),
    IfElse(IfElse),
    Enable(Ir)
}

impl PrettyPrint for Control {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        match self {
            Control::Empty => Ok(()),
            Control::For(for_) => for_.pretty_print(f),
            Control::Seq(seq) => seq.pretty_print(f),
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
