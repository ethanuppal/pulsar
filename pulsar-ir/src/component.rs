//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    cell::Cell, control::Control, from_ast::AsGeneratorPool, label::Label,
    variable::Variable
};
use inform::fmt::IndentFormatter;
use pulsar_frontend::{
    ast::pretty_print::PrettyPrint,
    attribute::{AttributeProvider, Attributes}
};
use pulsar_utils::{id::Id, pool::Handle};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Write}
};

// average research-grade software moment
pub struct Component {
    label: Label,
    attributes: Attributes,
    inputs: Vec<(Variable, Handle<Cell>)>,
    outputs: Vec<(Variable, Handle<Cell>)>,
    // internal_cells: Vec<Handle<Cell>>,
    /// Like reg-alloc but for cells. need better way to represent
    cell_alloc: HashMap<Variable, Handle<Cell>>,
    pub(crate) cfg: Handle<Control>
}

impl Component {
    pub fn new(
        label: Label, inputs: Vec<(Variable, Handle<Cell>)>,
        outputs: Vec<(Variable, Handle<Cell>)>, cfg: Handle<Control>
    ) -> Self {
        let initial_cell_alloc =
            inputs.iter().chain(&outputs).cloned().collect();
        Self {
            label,
            attributes: Attributes::default(),
            inputs,
            outputs,
            // internal_cells: Vec::new(),
            cell_alloc: initial_cell_alloc,
            cfg
        }
    }

    pub fn label(&self) -> &Label {
        &self.label
    }

    pub fn inputs(&self) -> &[(Variable, Handle<Cell>)] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[(Variable, Handle<Cell>)] {
        &self.outputs
    }

    pub fn cells(&self) -> &HashMap<Variable, Handle<Cell>> {
        &self.cell_alloc
    }

    pub fn internal_cells(&self) -> Vec<(Variable, Handle<Cell>)> {
        let mut interface = HashSet::new();
        for (var, _) in self.inputs.iter().chain(&self.outputs) {
            interface.insert(var);
        }
        self.cell_alloc
            .iter()
            .filter(|(var, _)| !interface.contains(var))
            .map(|(var, cell)| (*var, *cell))
            .collect()
    }

    pub fn cfg(&self) -> &Control {
        &self.cfg
    }

    pub fn cfg_id<P: AsGeneratorPool>(&self, pool: &P) -> Id {
        self.cfg.id_in(pool)
    }
}

impl AttributeProvider for Component {
    fn attributes_ref(&self) -> &Attributes {
        &self.attributes
    }

    fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
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
            let mut cell_alloc_list: Vec<_> = self.cell_alloc.iter().collect();
            cell_alloc_list.sort_by(|a, b| a.0.cmp(b.0));
            for (var, cell) in &cell_alloc_list {
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

/// A mutable view into a [`Component`] while its control is being mutably
/// borrowed.
pub struct ComponentViewMut<'comp> {
    pub label: &'comp mut Label,
    pub inputs: &'comp mut Vec<(Variable, Handle<Cell>)>,
    pub outputs: &'comp mut Vec<(Variable, Handle<Cell>)>,
    pub cell_alloc: &'comp mut HashMap<Variable, Handle<Cell>>
}

impl Component {
    pub fn as_view_mut(&mut self) -> (Handle<Control>, ComponentViewMut) {
        (
            self.cfg,
            ComponentViewMut {
                label: &mut self.label,
                inputs: &mut self.inputs,
                outputs: &mut self.outputs,
                cell_alloc: &mut self.cell_alloc
            }
        )
    }
}
