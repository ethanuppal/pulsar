//! Handrolled calyx builder API.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

// This code was mostly written before I became better at Rust; as such, it
// contains a mismatch of different patterns.

use calyx_ir::RRC;
use pulsar_utils::environment::Environment;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    marker::PhantomData,
    path::PathBuf
};

pub mod macros;

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

    CombMemoryD2 {
        size: usize,
        length_outer: usize,
        address_bits_outer: usize,
        length_inner: usize,
        address_bits_inner: usize
    },

    /// A calyx primitive other than a register, memory, or constant.
    Primitive { name: String, params: Vec<u64> },

    /// `GoDoneComponent { component }` is a cell for a component named
    /// `component`.
    GoDoneComponent { component: String },

    /// `CombComponent(component)` is a cell for a combinational component
    /// named `component`.
    CombComponent(String),

    /// `Constant { width }` is a `"std_const"` with bit-width `width`.
    Constant { width: usize }
}

impl CalyxCellKind {
    /// Whether the cell represents a primitive, in which case
    /// [`CalyxCellKind::to_string`] retrieves the name of the primitive in the
    /// standard library.
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Self::Register { .. }
                | Self::CombMemoryD1 { .. }
                | Self::Primitive { .. }
        )
    }

    /// Whether the cell is a memory.
    pub fn is_memory(&self) -> bool {
        matches!(self, Self::CombMemoryD1 { .. } | Self::CombMemoryD2 { .. })
    }

    /// The parameters associated with the primitive.
    ///
    /// Requires: `self.is_primitive()`.
    pub(crate) fn primitive_params(&self) -> Vec<u64> {
        match &self {
            Self::Register { size } => vec![*size as u64],
            Self::CombMemoryD1 {
                size,
                length,
                address_bits
            } => vec![*size as u64, *length as u64, *address_bits as u64],
            Self::CombMemoryD2 {
                size,
                length_outer,
                address_bits_outer,
                length_inner,
                address_bits_inner
            } => vec![
                *size as u64,
                *length_outer as u64,
                *length_inner as u64,
                *address_bits_outer as u64,
                *address_bits_inner as u64,
            ],
            Self::Primitive { name: _, params } => params.clone(),
            Self::GoDoneComponent { .. } | Self::CombComponent(_) => {
                panic!("Cell not a primitive")
            }
            CalyxCellKind::Constant { width } => vec![*width as u64]
        }
    }
}

impl Display for CalyxCellKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Register { .. } => "std_reg",
            Self::CombMemoryD1 { .. } => "comb_mem_d1",
            Self::CombMemoryD2 { .. } => "comb_mem_d2",
            Self::Primitive { name, params: _ } => name,
            Self::GoDoneComponent { component } => component,
            Self::CombComponent(component) => component,
            Self::Constant { .. } => "std_const"
        }
        .fmt(f)
    }
}

// might remove these later
pub type CalyxPort = RRC<calyx_ir::Port>;

// pub trait CalyxPortProvider {
//     fn get<S: AsRef<str>>(port: S) -> CalyxPort;
// }

/// A wrapper around [`calyx_ir::Cell`]s containing additional semantic
/// information (see [`CalyxCellKind`]).
#[derive(Clone)]
pub struct CalyxCell {
    pub kind: CalyxCellKind,
    pub value: RRC<calyx_ir::Cell>
}

impl CalyxCell {
    /// See [`calyx_ir::Cell::get`].
    pub fn get<S: AsRef<str>>(&self, port: S) -> CalyxPort {
        self.value.borrow().get(port.as_ref())
    }
}

/// An abstraction over a calyx group for adding assignments.
pub trait CalyxAssignmentContainer {
    type AssignmentMarker;

    /// Inserts a single assignment.
    fn add(&mut self, assignment: calyx_ir::Assignment<Self::AssignmentMarker>);

    fn assign(&mut self, lhs: CalyxPort, rhs: CalyxPort) {
        self.add(calyx_ir::Assignment::new(lhs, rhs));
    }

    /// Inserts a set of assignment.
    fn extend<
        I: IntoIterator<Item = calyx_ir::Assignment<Self::AssignmentMarker>>
    >(
        &mut self, assignments: I
    ) {
        assignments.into_iter().for_each(|a| self.add(a));
    }
}

/// See [`calyx_ir::Group`].
pub struct CalyxGroup {
    pub value: RRC<calyx_ir::Group>
}

impl CalyxAssignmentContainer for CalyxGroup {
    type AssignmentMarker = calyx_ir::Nothing;

    fn add(
        &mut self, assignment: calyx_ir::Assignment<Self::AssignmentMarker>
    ) {
        self.value.borrow_mut().assignments.push(assignment);
    }
}

/// See [`calyx_ir::StaticGroup`].
pub struct CalyxStaticGroup {
    pub value: RRC<calyx_ir::StaticGroup>
}

impl CalyxAssignmentContainer for CalyxStaticGroup {
    type AssignmentMarker = calyx_ir::StaticTiming;

    fn add(
        &mut self, assignment: calyx_ir::Assignment<Self::AssignmentMarker>
    ) {
        self.value.borrow_mut().assignments.push(assignment);
    }
}

/// See [`calyx_ir::CombGroup`].
pub struct CalyxCombGroup {
    pub value: RRC<calyx_ir::CombGroup>
}

impl CalyxAssignmentContainer for CalyxCombGroup {
    type AssignmentMarker = calyx_ir::Nothing;

    fn add(
        &mut self, assignment: calyx_ir::Assignment<Self::AssignmentMarker>
    ) {
        self.value.borrow_mut().assignments.push(assignment);
    }
}

/// A flag for [`CalyxControl`].
pub trait CalyxControlType {}

/// Represents sequential [`CalyxControl`].
pub struct Sequential;
impl CalyxControlType for Sequential {}

/// Represents parallel [`CalyxControl`].
pub struct Parallel;
impl CalyxControlType for Parallel {}

/// A wrapper around [`calyx_ir::Control`] for scoped building.
pub struct CalyxControl<T: CalyxControlType> {
    children: Vec<calyx_ir::Control>,
    phantom: PhantomData<T>
}

impl<T: CalyxControlType> CalyxControl<T> {
    /// Enables the given control in a way dependent on `T: CalyxControlType`.
    /// It is recommended to use the `T`-specific `enable` and `enable_next` to
    /// be explicit in semantics.
    pub fn insert(&mut self, group: &CalyxGroup) {
        self.children
            .push(calyx_ir::Control::enable(group.value.clone()));
    }

    pub fn insert_static(&mut self, group: &CalyxStaticGroup) {
        self.children
            .push(calyx_ir::Control::static_enable(group.value.clone()));
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
        F: FnOnce(&mut CalyxControl<Sequential>) {
        let mut child = CalyxControl::<Sequential>::default();
        f(&mut child);
        self.children.push(calyx_ir::Control::seq(child.children));
    }

    /// Opens a `par` context. See [`CalyxControl::seq`] for details.
    pub fn par<F>(&mut self, f: F)
    where
        F: FnOnce(&mut CalyxControl<Parallel>) {
        let mut child = CalyxControl::<Parallel>::default();
        f(&mut child);
        self.children.push(calyx_ir::Control::par(child.children));
    }

    /// Opens an `if` context. See [`CalyxControl::seq`] for details.
    pub fn if_<F>(
        &mut self, port: CalyxPort, cond: Option<CalyxCombGroup>, true_f: F,
        false_f: F
    ) where
        F: FnOnce(&mut CalyxControl<Sequential>) {
        let mut true_branch = CalyxControl::<Sequential>::default();
        let mut false_branch = CalyxControl::<Sequential>::default();
        true_f(&mut true_branch);
        false_f(&mut false_branch);
        self.children.push(calyx_ir::Control::if_(
            port,
            cond.map(|cond| cond.value),
            Box::new(true_branch.to_control()),
            Box::new(false_branch.to_control())
        ));
    }

    /// Opens a `while` context. See [`CalyxControl::seq`] for details.
    pub fn while_<F>(
        &mut self, port: CalyxPort, cond: Option<CalyxCombGroup>, f: F
    ) where
        F: FnOnce(&mut CalyxControl<Sequential>) {
        let mut body = CalyxControl::<Sequential>::default();
        f(&mut body);
        self.children.push(calyx_ir::Control::while_(
            port,
            cond.map(|cond| cond.value),
            Box::new(body.to_control())
        ));
    }

    // TODO: more control
}

impl<T: CalyxControlType> Default for CalyxControl<T> {
    fn default() -> Self {
        Self {
            children: vec![],
            phantom: PhantomData
        }
    }
}

impl CalyxControl<Sequential> {
    /// Enables `group` to run in sequence.
    pub fn enable_next(&mut self, group: &CalyxGroup) {
        self.insert(group);
    }

    /// Unwraps the control builder.
    pub fn to_control(self) -> calyx_ir::Control {
        if self.children.is_empty() {
            calyx_ir::Control::empty()
        } else {
            calyx_ir::Control::seq(self.children)
        }
    }
}

impl CalyxControl<Parallel> {
    /// Enables `group` to run in parallel.
    pub fn enable(&mut self, group: &CalyxGroup) {
        self.insert(group);
    }

    /// Unwraps the control builder.
    pub fn to_control(self) -> calyx_ir::Control {
        if self.children.is_empty() {
            calyx_ir::Control::empty()
        } else {
            calyx_ir::Control::par(self.children)
        }
    }
}

/// A wrapper for a calyx component that can only be created through
/// [`CalyxBuilder::build_component`], where it must live no longer than the
/// builder that created it (details later; this is a limitation of calyx
/// itself, not this builder API).
///
/// The wrapper maintains cell and control manipulation. Cells can be created
/// through methods such as [`CalyxComponent::named_reg`] or
/// [`CalyxComponent::component_cell`]. It also contains unique per-component
/// data initialized via `ComponentData::default` which can be accessed through
/// appropriate getters.
///
/// If you have a reference to this object, ensure that it's lifetime does not
/// exceed that (`'a`) of the builder it was created from. This may mean adding
/// `<'a, 'b: 'a>` everywhere for `&'a CalyxComponent<'b, ...>`.
pub struct CalyxComponent<'a, ComponentData: Default> {
    ext_sigs: &'a HashMap<String, Vec<calyx_ir::PortDef<u64>>>,
    lib_sig: &'a calyx_ir::LibrarySignatures,
    env: Environment<String, CalyxCell>,
    component: calyx_ir::Component,
    cell_name_prefix: String,
    unique_counter: usize,
    user_data: ComponentData,
    control_builder: CalyxControl<Sequential>
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
    pub fn control(&mut self) -> &mut CalyxControl<Sequential> {
        &mut self.control_builder
    }

    /// Enables direct access to a [`calyx_ir::Builder`] for this component.
    pub fn with_calyx_builder<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut calyx_ir::Builder) -> T {
        // Creating a calyx_ir::Builder is very cheap (for now). If I can figure
        // out a better way, e.g., storing the builder in the struct, I will
        // switch to that, but I tried doing that for many hours to no avail.
        let mut ir_builder =
            calyx_ir::Builder::new(&mut self.component, self.lib_sig)
                .not_generated();
        f(&mut ir_builder)
    }

    /// A register cell bound to `name`.
    ///
    /// Requires: `name` has not been bound.
    pub fn new_reg(&mut self, name: String, width: usize) -> CalyxCell {
        let mut bind_name = self.cell_name_prefix.clone();
        bind_name.push_str(&name);
        self.create_cell(bind_name, CalyxCellKind::Register { size: width })
    }

    /// A memory cell bound to `name`.
    ///
    /// Requires: `name` has not been bound.
    pub fn named_mem_d1(
        &mut self, name: String, cell_size: usize, length: usize,
        address_bits: usize
    ) -> CalyxCell {
        let mut bind_name = self.cell_name_prefix.clone();
        bind_name.push_str(&name);
        self.create_cell(
            bind_name,
            CalyxCellKind::CombMemoryD1 {
                size: cell_size,
                length,
                address_bits
            }
        )
    }

    /// Creates a cell named `name` for a primitive `prim` with parameters
    /// `params`. Before using this function, see if
    /// [`CalyxComponent::named_reg`] or [`CalyxComponent::named_mem`] are more
    /// appropriate.
    ///
    /// Requires: `name` has not been bound.
    pub fn new_prim(
        &mut self, name: &str, prim: &str, params: Vec<u64>
    ) -> CalyxCell {
        self.create_cell(
            name.into(),
            CalyxCellKind::Primitive {
                name: prim.into(),
                params
            }
        )
    }

    pub fn new_unnamed_prim(
        &mut self, prim: &str, params: Vec<u64>
    ) -> CalyxCell {
        self.new_unnamed_cell(CalyxCellKind::Primitive {
            name: prim.into(),
            params
        })
    }

    /// A cell for a component `component` whose name is guaranteed to begin
    /// with `prefix`. If `instantiate_new`, then a unique cell will be created.
    /// Both the cell and the actual cell name are returned.
    pub fn component_cell(
        &mut self, prefix: String, component: String, instantiate_new: bool
    ) -> (String, CalyxCell) {
        let cell_name = if instantiate_new {
            format!("{}{}", prefix, self.get_unique_number())
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

    /// An unnamed cell of a given `kind`.
    pub fn new_unnamed_cell(&mut self, kind: CalyxCellKind) -> CalyxCell {
        let cell_name = format!("t{}", self.get_unique_number());
        self.create_cell(cell_name, kind)
    }

    /// A constant cell, that is, a primitive `"std_const"`.
    pub fn constant(&mut self, value: i64, width: usize) -> CalyxCell {
        CalyxCell {
            kind: CalyxCellKind::Constant { width },
            value: self.with_calyx_builder(|b| {
                b.add_constant(value as u64, width as u64)
            })
        }
    }

    /// Equivalent to `constant(1, 1)`.
    pub fn signal_out(&mut self) -> CalyxCell {
        self.constant(1, 1)
    }

    /// Adds `name` as a named alias to refer to `cell`.
    ///
    /// Requires: `name` has not been previously bound.
    pub fn alias_cell(&mut self, name: String, cell: CalyxCell) {
        assert!(self
            .env
            .bind(format!("{}{}", self.cell_name_prefix, name), cell)
            .is_none());
    }

    /// Looks up a named cell previously bound to `name`.
    ///
    /// Requires: `name` has been bound.
    pub fn find(&mut self, name: String) -> CalyxCell {
        self.env
            .find(format!("{}{}", self.cell_name_prefix, name))
            .expect("Did not find cell in component environment")
            .clone()
    }

    /// See [`Environment::push`].
    pub fn begin_scope(&mut self) {
        self.env.push();
    }

    /// See [`Environment::pop`].
    pub fn end_scope(&mut self) -> bool {
        self.env.pop()
    }

    /// Creates a new group guaranteed to start with `prefix`.
    pub fn add_group(&mut self, prefix: &str) -> CalyxGroup {
        CalyxGroup {
            value: self.with_calyx_builder(|b| b.add_group(prefix))
        }
    }

    /// Creates a new static group guaranteed to start with `prefix`.
    pub fn add_static_group(
        &mut self, prefix: &str, latency: usize
    ) -> CalyxStaticGroup {
        CalyxStaticGroup {
            value: self.with_calyx_builder(|b| {
                b.add_static_group(prefix, latency as u64)
            })
        }
    }

    /// Creates a new combinational group guaranteed to start with `prefix`.
    pub fn add_comb_group(&mut self, prefix: &str) -> CalyxCombGroup {
        CalyxCombGroup {
            value: self.with_calyx_builder(|b| b.add_comb_group(prefix))
        }
    }

    /// Yields a [`calyx_ir::Component`].
    pub fn finalize(self) -> calyx_ir::Component {
        *self.component.control.borrow_mut() =
            self.control_builder.to_control();
        self.component
    }

    /// Creates a cell of type `kind` bound to `key`.
    ///
    /// Requires: `key` has not been bound.
    fn create_cell(&mut self, key: String, kind: CalyxCellKind) -> CalyxCell {
        let calyx_cell = if kind.is_primitive() {
            self._create_primitive(
                key.clone(),
                kind.to_string(),
                kind.primitive_params()
            )
        } else if let CalyxCellKind::GoDoneComponent { component } = &kind {
            self._create_component_cell(key.clone(), component.clone())
        } else if let CalyxCellKind::CombComponent(component) = &kind {
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

    /// A number guaranteed to be unique across all calls to this function for a
    /// specific component builder such as `self`.
    fn get_unique_number(&mut self) -> usize {
        let result = self.unique_counter;
        self.unique_counter += 1;
        result
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
        let mut port_defs = self.ext_sigs.get(&component).unwrap().clone();

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

/// A builder for calyx IR optimized for generation from a higher-level AST or
/// IR.
pub struct CalyxBuilder {
    /// The calyx program being built.
    ctx: calyx_ir::Context,

    /// Component signatures.
    sigs: HashMap<String, Vec<calyx_ir::PortDef<u64>>>,

    /// Prefix for named cells to avoid collision with unnamed cells.
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
    /// - `entrypoint` is the name of the entry component in the program. If
    ///   `None` is passed, it will default to `"main"`. You can use
    ///   [`CalyxBuilder::set_entrypoint`] to update it.
    ///
    /// - `cell_name_prefix` is the non-empty prefix applied to all named cells
    ///   (e.g., those requested via [`CalyxComponent::named_reg`]) to guarantee
    ///   no collisions with unnamed cells (e.g., those requested via
    ///   [`CalyxComponent::unnamed_cell`]). It must be non-empty.
    pub fn new(
        prelude: Option<PathBuf>, lib_path: PathBuf,
        entrypoint: Option<String>, cell_name_prefix: String
    ) -> calyx_utils::CalyxResult<Self> {
        assert!(!cell_name_prefix.is_empty());

        // A workspace is created for the sole purpose of obtaining standard
        // library definitions -- it is immediately turned into a context.
        let ws = calyx_frontend::Workspace::construct(&prelude, &lib_path)?;
        let ctx = calyx_ir::Context {
            components: vec![],
            lib: ws.lib,
            entrypoint: entrypoint.unwrap_or("main".into()).into(),
            bc: calyx_ir::BackendConf::default(),
            extra_opts: vec![],
            metadata: None
        };

        Ok(Self {
            ctx,
            sigs: HashMap::new(),
            cell_name_prefix
        })
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

    /// Binds a component (named `name`)'s signature to a list of `ports` so it
    /// can be constructed or instantiated by another component.
    pub fn register_component(
        &mut self, name: String, ports: Vec<calyx_ir::PortDef<u64>>
    ) {
        self.sigs.insert(name, ports);
    }

    /// Returns a component wrapper for a registered component. Once you are
    /// finished with the component builder, call [`finish_component!`].
    ///
    /// Requires: [`CalyxBuilder::register_component`] has been issued for
    /// `name`.
    pub fn start_component<ComponentData: Default>(
        &self, name: String
    ) -> CalyxComponent<ComponentData> {
        CalyxComponent::new(
            calyx_ir::Component::new(
                name.clone(),
                self.sigs
                    .get(&name)
                    .expect("Use `register_component` first")
                    .clone(),
                true,
                false,
                None
            ),
            self.cell_name_prefix.clone(),
            &self.sigs,
            &self.ctx.lib
        )
    }

    /// Please use [`finish_component!`] instead.
    pub fn _finish_component(&mut self, component: calyx_ir::Component) {
        self.ctx.components.push(component);
    }

    /// Updates the name of the program entrypoint.
    ///
    /// Requires: [`CalyxBuilder::register_component`] has been issued for
    /// `entrypoint`.
    pub fn set_entrypoint(&mut self, entrypoint: String) {
        assert!(self.sigs.contains_key(&entrypoint));
        self.ctx.entrypoint = entrypoint.into();
    }

    /// Yields a [`calyx_ir::Context`].
    ///
    /// Requires: the entrypoint provided at [`CalyxBuilder::new`] is the name
    /// of a component added.
    pub fn finalize(self) -> calyx_ir::Context {
        self.ctx
    }
}

/// `finish_component!(builder, component)` marks a `component` as finalized in
/// `builder`.
#[macro_export]
macro_rules! finish_component {
    ($builder:expr, $component:expr) => {
        $builder._finish_component($component.finalize())
    };
}
