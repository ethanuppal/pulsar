//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{cell::Cell, control::Control, label::Label, variable::Variable};
use inform::fmt::IndentFormatter;
use pulsar_frontend::ast::pretty_print::PrettyPrint;
use pulsar_utils::pool::Handle;
use std::{
    collections::HashMap,
    fmt::{self, Display, Write}
};

pub struct Component {
    label: Label,
    inputs: Vec<(Variable, Handle<Cell>)>,
    outputs: Vec<(Variable, Handle<Cell>)>,
    internal_cells: Vec<Handle<Cell>>,
    /// Like reg-alloc but for cells. need better way to represent
    cell_alloc: HashMap<Variable, Handle<Cell>>,
    pub(crate) cfg: Control
}

impl Component {
    pub fn new(
        label: Label, inputs: Vec<(Variable, Handle<Cell>)>,
        outputs: Vec<(Variable, Handle<Cell>)>, cfg: Control
    ) -> Self {
        let initial_cell_alloc =
            inputs.iter().chain(&outputs).cloned().collect();
        Self {
            label,
            inputs,
            outputs,
            internal_cells: Vec::new(),
            cell_alloc: initial_cell_alloc,
            cfg
        }
    }

    pub fn inputs(&self) -> &[(Variable, Handle<Cell>)] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[(Variable, Handle<Cell>)] {
        &self.outputs
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
                .map(|(var, _)| var.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.outputs
                .iter()
                .map(|(var, _)| var.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )?;
        f.increase_indent();

        writeln!(f, "cells {{")?;
        {
            f.increase_indent();
            for (var, cell) in &self.cell_alloc {
                writeln!(f, "{} = {}", var, cell)?;
            }
            f.decrease_indent();
        }
        writeln!(f, "}}\ncontrol {{")?;
        {
            f.increase_indent();
            self.cfg.pretty_print(f)?;
            f.decrease_indent();
        }
        write!(f, "\n}}")?;

        f.decrease_indent();
        write!(f, "\n}}")
    }
}

impl Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}
