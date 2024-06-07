// Copyright (C) 2024 Ethan Uppal. All rights reserved.

use calyx_ir::RRC;
use pulsar_utils::environment::Environment;
use std::{collections::HashMap, fmt::Display, path::PathBuf};

pub mod builder_macros;

/// Describes the semantics of a cell.
#[derive(Clone, PartialEq, Eq)]
pub enum CalyxCellKind {
    /// `Register { size }` is a `"std_reg"` of bit-width `size`.
    Register { size: usize },

    /// `CombMemoryD1 { size, length, address_bits }` is a `"comb_mem_d1"` with
    /// cell bit-width `size`, cell count `length`, and address bit-width
    /// `address_bits`.
    CombMemoryD1 {
        size: usize,
        length: usize,
        address_bits: usize
    },

    /// A calyx primitive other than a register, memory, or constant.
    Primitive { name: String, params: Vec<u64> },

    /// `GoDoneComponent { component }` is a cell for a
    /// component named `component`.
    GoDoneComponent { component: String },

    /// `Constant { width }` is a `"std_const"` with bit-width `width`.
    Constant { width: usize }
}

impl CalyxCellKind {
    /// Whether the cell represents a primitive, in which case
    /// [`CalyxCellKind::to_string`] retrieves the name of the primitive in the
    /// standard library.
    pub fn is_primitive(&self) -> bool {
        match &self {
            Self::Register { size: _ }
            | Self::CombMemoryD1 {
                size: _,
                length: _,
                address_bits: _
            } => true,
            _ => false
        }
    }

    /// The parameters associated with the primitive.
    ///
    /// Requires: `self.is_primitive()`.
    pub(crate) fn primitive_params(&self) -> Vec<u64> {
        match &self {
            CalyxCellKind::Register { size } => vec![*size as u64],
            CalyxCellKind::CombMemoryD1 {
                size,
                length,
                address_bits
            } => vec![*size as u64, *length as u64, *address_bits as u64],
            CalyxCellKind::Primitive { name: _, params } => params.clone(),
            CalyxCellKind::GoDoneComponent { component } => {
                panic!("Cell not a primitive")
            }
            CalyxCellKind::Constant { width } => vec![*width as u64]
        }
    }
}

impl Display for CalyxCellKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Register { size: _ } => "std_reg",
            Self::CombMemoryD1 {
                size: _,
                length: _,
                address_bits: _
            } => "comb_mem_d1",
            Self::Primitive { name, params: _ } => name,
            Self::GoDoneComponent { component } => &component,
            Self::Constant { width: _ } => "std_const"
        }
        .fmt(f)
    }
}

/// A wrapper around [`calyx_ir::Cell`]s containing additional semantic
/// information (see [`CalyxCellKind`]).
#[derive(Clone)]
pub struct CalyxCell {
    pub kind: CalyxCellKind,
    pub value: RRC<calyx_ir::Cell>
}

// might remove these later
pub type CalyxPort = RRC<calyx_ir::Port>;
pub type CalyxGroup = RRC<calyx_ir::Group>;
pub type CalyxCombGroup = RRC<calyx_ir::CombGroup>;

/// A wrapper around [`calyx_ir::Control`] for scoped building.
pub struct CalyxControl {
    children: Vec<calyx_ir::Control>
}

impl CalyxControl {
    /// Unwraps the control builder.
    pub fn to_control(self) -> calyx_ir::Control {
        if self.children.is_empty() {
            calyx_ir::Control::empty()
        } else {
            calyx_ir::Control::seq(self.children)
        }
    }

    /// Enables `group` in the current context. For example, if the current
    /// context is a `seq`, this group will be enabled sequentially after
    /// previous groups and control operators.
    pub fn enable(&mut self, group: &CalyxGroup) {
        self.children.push(calyx_ir::Control::enable(group.clone()));
    }

    /// Opens a `seq` context where `f` is called. For instance,
    /// ```
    /// fn add_seq(control: &mut CalyxControl, my_group: &CalyxGroup) {
    ///     control.seq(|s| {
    ///         s.enable(my_group);
    ///     });
    /// }
    /// ```
    /// produces the following calyx:
    /// ```
    /// ...
    /// seq {
    ///     my_group;
    /// }
    /// ...
    /// ```
    pub fn seq<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self) {
        let mut child = Self::default();
        f(&mut child);
        self.children.push(calyx_ir::Control::seq(child.children));
    }

    /// Opens a `par` context. See [`CalyxControl::seq`] for details.
    pub fn par<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Self) {
        let mut child = Self::default();
        f(&mut child);
        self.children.push(calyx_ir::Control::par(child.children));
    }

    /// Opens an `if` context. See [`CalyxControl::seq`] for details.
    pub fn if_<F>(
        &mut self, port: CalyxPort, cond: Option<CalyxCombGroup>, true_f: F,
        false_f: F
    ) where
        F: FnOnce(&mut Self) {
        let mut true_branch = Self::default();
        let mut false_branch = Self::default();
        true_f(&mut true_branch);
        false_f(&mut false_branch);
        self.children.push(calyx_ir::Control::if_(
            port,
            cond.map(|cond| cond),
            Box::new(true_branch.to_control()),
            Box::new(false_branch.to_control())
        ));
    }

    /// Opens a `while` context. See [`CalyxControl::seq`] for details.
    pub fn while_<F>(
        &mut self, port: CalyxPort, cond: Option<CalyxCombGroup>, f: F
    ) where
        F: FnOnce(&mut Self) {
        let mut body = Self::default();
        f(&mut body);
        self.children.push(calyx_ir::Control::while_(
            port,
            cond.map(|cond| cond),
            Box::new(body.to_control())
        ));
    }

    // TODO: more control
}

impl Default for CalyxControl {
    fn default() -> Self {
        Self { children: vec![] }
    }
}

/// A wrapper for a calyx component that can only be created through
/// [`CalyxBuilder::build_component`], where it must live no longer than the
/// builder that created it.
///
/// The wrapper maintains cell and control manipulation. Cells can be created
/// through methods such as [`CalyxComponent::named_reg`] or
/// [`CalyxComponent::component_cell`]. It also contains unique per-component
/// data initialized via `ComponentData::default` which can be accessed through
/// appropriate getters.
pub struct CalyxComponent<'a, ComponentData: Default> {
    ext_sigs: &'a HashMap<String, Vec<calyx_ir::PortDef<u64>>>,
    lib_sig: &'a calyx_ir::LibrarySignatures,
    env: Environment<String, CalyxCell>,
    component: calyx_ir::Component,
    cell_name_prefix: String,
    unique_counter: usize,
    user_data: ComponentData,
    control_builder: CalyxControl
}

impl<'a, ComponentData: Default> CalyxComponent<'a, ComponentData> {
    fn new(
        component: calyx_ir::Component, cell_name_prefix: String,
        ext_sigs: &'a HashMap<String, Vec<calyx_ir::PortDef<u64>>>,
        lib_sig: &'a calyx_ir::LibrarySignatures
    ) -> Self {
        Self {
            ext_sigs,
            lib_sig,
            env: Environment::new(),
            component,
            cell_name_prefix,
            unique_counter: 0,
            user_data: ComponentData::default(),
            control_builder: CalyxControl::default()
        }
    }

    /// The user data associated with the component.
    pub fn user_data_ref(&self) -> &ComponentData {
        &self.user_data
    }

    /// See [`CalyxComponent::user_data_ref`].
    pub fn user_data_mut(&mut self) -> &mut ComponentData {
        &mut self.user_data
    }

    /// The input/output signature of this component as a cell.
    pub fn signature(&mut self) -> CalyxCell {
        CalyxCell {
            kind: CalyxCellKind::GoDoneComponent {
                component: self.component.name.to_string()
            },
            value: self.component.signature.clone()
        }
    }

    /// The control of this component.
    pub fn control(&mut self) -> &mut CalyxControl {
        &mut self.control_builder
    }

    /// Enables direct access to a [`calyx_ir::Builder`] for this component.
    pub fn with_calyx_builder<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut calyx_ir::Builder) -> T {
        let mut ir_builder =
            calyx_ir::Builder::new(&mut self.component, &self.lib_sig)
                .not_generated()
                .validate();
        f(&mut ir_builder)
    }

    /// A register cell bound to `name`.
    pub fn named_reg(&mut self, name: String, width: usize) -> CalyxCell {
        let mut bind_name = self.cell_name_prefix.clone();
        bind_name.push_str(&name);
        self.find_or_create_cell(
            bind_name,
            CalyxCellKind::Register { size: width }
        )
    }

    /// A memory cell bound to `name`.
    pub fn named_mem(
        &mut self, name: String, cell_size: usize, length: usize,
        address_bits: usize
    ) -> CalyxCell {
        let mut bind_name = self.cell_name_prefix.clone();
        bind_name.push_str(&name);
        self.find_or_create_cell(
            bind_name,
            CalyxCellKind::CombMemoryD1 {
                size: cell_size,
                length,
                address_bits
            }
        )
    }

    pub fn named_prim(
        &mut self, name: &str, prim: &str, params: Vec<u64>
    ) -> CalyxCell {
        self.find_or_create_cell(
            name.into(),
            CalyxCellKind::Primitive {
                name: prim.into(),
                params
            }
        )
    }

    /// A cell for a component `component` whose name is guaranteed to begin
    /// with `prefix`. If `instantiate_new`, then a unique cell will be created.
    /// Both the cell and the actual cell name are returned.
    pub fn component_cell(
        &mut self, prefix: String, component: String, instantiate_new: bool
    ) -> (String, CalyxCell) {
        let cell_name = if instantiate_new {
            format!("{}{}", prefix, self.unique_number())
        } else {
            prefix
        };
        let cell = CalyxCell {
            kind: CalyxCellKind::GoDoneComponent {
                component: component.clone()
            },
            value: self._create_component_cell(cell_name.clone(), component)
        };
        (cell_name, cell)
    }

    /// An unnamed register cell.
    pub fn unnamed_cell(&mut self, kind: CalyxCellKind) -> CalyxCell {
        let cell_name = format!("t{}", self.unique_number());
        self.find_or_create_cell(cell_name, kind)
    }

    /// A constant cell.
    pub fn constant(&'a mut self, value: i64, width: usize) -> CalyxCell {
        self.with_calyx_builder(|b| CalyxCell {
            kind: CalyxCellKind::Constant { width },
            value: b.add_constant(value as u64, width as u64)
        })
    }

    fn find_or_create_cell(
        &mut self, key: String, kind: CalyxCellKind
    ) -> CalyxCell {
        if let Some(cell) = self.env.find(key.clone()) {
            cell.clone()
        } else {
            let calyx_cell = if kind.is_primitive() {
                self._create_primitive(
                    key.clone(),
                    kind.to_string(),
                    kind.primitive_params()
                )
            } else if let CalyxCellKind::GoDoneComponent { component } = &kind {
                self._create_component_cell(key.clone(), component.clone())
            } else {
                panic!("unknown cell kind")
            };
            let cell = CalyxCell {
                kind,
                value: calyx_cell
            };
            self.env.bind(key, cell.clone());
            cell
        }
    }

    fn unique_number(&mut self) -> usize {
        let result = self.unique_counter;
        self.unique_counter += 1;
        result
    }

    fn finalize(self) -> calyx_ir::Component {
        *self.component.control.borrow_mut() =
            self.control_builder.to_control();
        self.component
    }

    /// Creates a [`calyx_ir::Cell`] for a `primitive`.
    fn _create_primitive(
        &mut self, name: String, primitive: String, params: Vec<u64>
    ) -> RRC<calyx_ir::Cell> {
        self.with_calyx_builder(|b| b.add_primitive(name, primitive, &params))
    }

    /// Creates a [`calyx_ir::Cell`] for a `component`.
    fn _create_component_cell(
        &mut self, name: String, component: String
    ) -> RRC<calyx_ir::Cell> {
        let mut port_defs = self.ext_sigs.get(&name).unwrap().clone();

        let mut go_attr = calyx_ir::Attributes::default();
        go_attr.insert(calyx_ir::Attribute::Num(calyx_ir::NumAttr::Go), 1);
        port_defs.push(calyx_ir::PortDef::new(
            "go",
            1,
            calyx_ir::Direction::Input,
            go_attr
        ));

        let mut done_attr = calyx_ir::Attributes::default();
        done_attr.insert(calyx_ir::Attribute::Num(calyx_ir::NumAttr::Done), 1);
        port_defs.push(calyx_ir::PortDef::new(
            "done",
            1,
            calyx_ir::Direction::Output,
            done_attr
        ));

        let mut clk_attr = calyx_ir::Attributes::default();
        clk_attr.insert(calyx_ir::Attribute::Bool(calyx_ir::BoolAttr::Clk), 1);
        port_defs.push(calyx_ir::PortDef::new(
            "clk",
            1,
            calyx_ir::Direction::Input,
            clk_attr
        ));

        let mut reset_attr = calyx_ir::Attributes::default();
        reset_attr
            .insert(calyx_ir::Attribute::Bool(calyx_ir::BoolAttr::Reset), 1);
        port_defs.push(calyx_ir::PortDef::new(
            "reset",
            1,
            calyx_ir::Direction::Input,
            reset_attr
        ));

        let cell = self._cell_from_signature(
            name.clone().into(),
            calyx_ir::CellType::Component {
                name: component.clone().into()
            },
            port_defs
        );
        self.component.cells.add(cell.clone());
        cell
    }

    /// For some reason, this is private: https://github.com/calyxir/calyx/blob/main/calyx-ir/src/builder.rs#L361
    fn _cell_from_signature(
        &self, name: calyx_ir::Id, typ: calyx_ir::CellType,
        ports: Vec<calyx_ir::PortDef<u64>>
    ) -> RRC<calyx_ir::Cell> {
        let cell = calyx_ir::rrc(calyx_ir::Cell::new(name, typ));
        ports.into_iter().for_each(|pd| {
            let port = calyx_ir::rrc(calyx_ir::Port {
                name: pd.name(),
                width: pd.width,
                direction: pd.direction,
                parent: calyx_ir::PortParent::Cell(calyx_ir::WRC::from(&cell)),
                attributes: pd.attributes
            });
            cell.borrow_mut().ports.push(port);
        });
        cell
    }
}

pub struct CalyxBuilder {
    ctx: calyx_ir::Context,
    sigs: HashMap<String, Vec<calyx_ir::PortDef<u64>>>,
    cell_name_prefix: String
}

impl CalyxBuilder {
    /// Constructs a new calyx builder. See the documentation at
    /// [`CalyxBuilder`] for general usage information.
    ///
    /// - `prelude` is an optional calyx file that will be parsed and inlined in
    ///   additional to the standard library, which is useful for additional
    ///   component definitions or imports.
    ///
    /// - `lib_path` should be the root of the calyx installation location,
    ///   e.g., the folder generated from cloning the repository from GitHub.
    ///
    /// - `entrypoint` is the name of the entry component in the program.
    ///
    /// - `cell_name_prefix` is the non-empty prefix applied to all named cells
    ///   (e.g., those requested via [`CalyxComponent::named_reg`]) to guarantee
    ///   no collisions with unnamed cells (e.g., those requested via
    ///   [`CalyxComponent::unnamed_cell`]). It must be non-empty.
    pub fn new(
        prelude: Option<PathBuf>, lib_path: PathBuf, entrypoint: String,
        cell_name_prefix: String
    ) -> Self {
        assert!(!cell_name_prefix.is_empty());

        // A workspace is created for the sole purpose of obtaining standard
        // library definitions -- it is immediately turned into a context.
        let ws =
            calyx_frontend::Workspace::construct(&prelude, &lib_path).unwrap();
        let ctx = calyx_ir::Context {
            components: vec![],
            lib: ws.lib,
            entrypoint: entrypoint.into(),
            bc: calyx_ir::BackendConf::default(),
            extra_opts: vec![],
            metadata: None
        };
        Self {
            ctx,
            sigs: HashMap::new(),
            cell_name_prefix
        }
    }

    /// <div class="warning">This builder cannot be used.</div>
    pub fn dummy() -> Self {
        Self {
            ctx: calyx_ir::Context {
                components: vec![],
                lib: calyx_ir::LibrarySignatures::default(),
                entrypoint: "".into(),
                bc: calyx_ir::BackendConf::default(),
                extra_opts: vec![],
                metadata: None
            },
            sigs: HashMap::new(),
            cell_name_prefix: "".into()
        }
    }

    /// Binds a component `name` to a list of `ports` so it can be instantiated
    /// by another component.
    pub fn register_component(
        &mut self, name: String, ports: Vec<calyx_ir::PortDef<u64>>
    ) {
        self.sigs.insert(name, ports);
    }

    /// Returns a component builder for a registered component. Invalidates a
    /// previous component builder.
    ///
    /// Requires: [`CalyxBuilder::register_component`] has been issued for
    /// `name`.
    pub fn start_component<'b, ComponentData: Default>(
        &'b mut self, name: String
    ) -> CalyxComponent<'b, ComponentData> {
        CalyxComponent::new(
            calyx_ir::Component::new(
                name.clone(),
                self.sigs.get(&name).unwrap().clone(),
                true,
                false,
                None
            ),
            self.cell_name_prefix.clone(),
            &self.sigs,
            &self.ctx.lib
        )
    }

    pub fn finish_component<ComponentData: Default>(
        &mut self, component: CalyxComponent<ComponentData>
    ) {
        self.ctx.components.push(component.finalize());
    }

    /// Yields a [`calyx_ir::Context`].
    ///
    /// Requires: the entrypoint provided at [`CalyxBuilder::new`] is the name
    /// of a component added.
    pub fn finalize(self) -> calyx_ir::Context {
        self.ctx
    }
}