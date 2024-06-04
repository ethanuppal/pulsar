// Copyright (C) 2024 Ethan Uppal. All rights reserved.

use calyx_ir::RRC;
use std::{collections::HashMap, path::PathBuf};

pub struct CalyxGroup {
    name: String,
    value: RRC<calyx_ir::Group>
}

pub struct CalyxComponent<'a, ComponentData: Default> {
    user_data: ComponentData,
    ir_builder: calyx_ir::Builder<'a>
}

impl<'a, ComponentData: Default> CalyxComponent<'a, ComponentData> {
    pub fn user_data(&self) -> &ComponentData {
        &self.user_data
    }
}

pub struct CalyxBuilder<'a, ComponentData: Default> {
    ctx: calyx_ir::Context,
    sig: HashMap<String, Vec<calyx_ir::PortDef<u64>>>,
    component: Option<calyx_ir::Component>,
    component_wrapper: Option<CalyxComponent<'a, ComponentData>>
}

impl<'a, ComponentData: Default> CalyxBuilder<'a, ComponentData> {
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
    ///  `entrypoint` is the name of the entry component in the program.
    pub fn new(
        prelude: Option<PathBuf>, lib_path: PathBuf, entrypoint: String
    ) -> Self {
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
            sig: HashMap::new(),
            component: None,
            component_wrapper: None
        }
    }

    /// Binds a component `name` to a list of `ports` so it can be instantiated
    /// by another component.
    pub fn register_component(
        &mut self, name: String, ports: Vec<calyx_ir::PortDef<u64>>
    ) {
        self.sig.insert(name, ports);
    }

    /// Returns a component builder for a previously registered component.
    ///
    /// Requires: [`CalyxBuilder::register_component`] has been issued for
    /// `name`.
    pub fn build_component(
        &'a mut self, name: String
    ) -> &'a mut CalyxComponent<'a, ComponentData> {
        self.add_uncollected_component();
        self.component = Some(calyx_ir::Component::new(
            name.clone(),
            self.sig.get(&name).unwrap().clone(),
            true,
            false,
            None
        ));
        self.component_wrapper = Some(CalyxComponent {
            user_data: ComponentData::default(),
            ir_builder: calyx_ir::Builder::new(
                self.component.as_mut().unwrap(),
                &self.ctx.lib
            )
            .not_generated()
        });
        self.component_wrapper.as_mut().unwrap()
    }

    pub fn finalize(mut self) -> calyx_ir::Context {
        self.add_uncollected_component();
        self.ctx
    }

    fn add_uncollected_component(&mut self) {
        if let Some(previous_component) = std::mem::take(&mut self.component) {
            self.ctx.components.push(previous_component);
        }
    }
}
