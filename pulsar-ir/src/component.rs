//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{control::Control, label::Label, memory::Memory};
use inform::fmt::IndentFormatter;
use pulsar_frontend::ast::{pretty_print::PrettyPrint, ty::Type};
use pulsar_utils::pool::Handle;
use std::fmt::{self, Display, Write};

pub struct Component {
    label: Label,
    inputs: Vec<Handle<Type>>,
    outputs: Vec<Handle<Type>>,
    memories: Vec<Memory>,
    cfg: Control
}

impl Component {
    pub fn new(
        label: Label, inputs: Vec<Handle<Type>>, outputs: Vec<Handle<Type>>,
        memories: Vec<Memory>, cfg: Control
    ) -> Self {
        Self {
            label,
            inputs,
            outputs,
            memories,
            cfg
        }
    }
}

impl PrettyPrint for Component {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        writeln!(
            f,
            "comp {}({}) -> ({}) {{",
            self.label,
            self.inputs
                .iter()
                .map(|ty| ty.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.outputs
                .iter()
                .map(|ty| ty.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        f.increase_indent();
        self.cfg.pretty_print(f)?;
        f.decrease_indent();
        write!(f, "}}")
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}
