//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::ops::DerefMut;

use pulsar_utils::{id::Id, pool::Handle};

use crate::{
    component::{Component, ComponentViewMut},
    control::{Control, For, IfElse, Par, Seq},
    from_ast::AsGeneratorPool,
    Ir
};

pub enum Action {
    None,
    ModifiedInternally,
    Remove,
    Replace(Control)
}

impl Action {
    fn execute(self, control: &mut Control, did_modify: &mut bool) {
        match self {
            Action::None => {}
            Action::ModifiedInternally => {
                *did_modify = true;
            }
            Action::Remove => {
                *control = Control::Empty;
                *did_modify = true;
            }
            Action::Replace(new_control) => {
                *control = new_control;
                *did_modify = true;
            }
        }
    }
}

/// To override traversal behavior, implement `traverse_component` and
/// `traverse_control`, although it is highly unlikely to use anything beside
/// the default implementation of these functions.
pub trait VisitorMut<P: AsGeneratorPool> {
    #[allow(unused_variables)]
    fn start_component(&mut self, comp: &mut Component, pool: &mut P) {}
    #[allow(unused_variables)]
    fn finish_component(&mut self, comp: &mut Component, pool: &mut P) {}

    #[allow(unused_variables)]
    fn start_for(
        &mut self, id: Id, for_: &mut For, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn start_seq(
        &mut self, id: Id, seq: &mut Seq, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn start_par(
        &mut self, id: Id, par: &mut Par, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn start_if_else(
        &mut self, id: Id, if_else: &mut IfElse,
        comp_view: &mut ComponentViewMut, pool: &mut P
    ) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn start_enable(
        &mut self, id: Id, enable: &mut Ir, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        Action::None
    }

    #[allow(unused_variables)]
    fn finish_for(
        &mut self, id: Id, for_: &mut For, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn finish_seq(
        &mut self, id: Id, seq: &mut Seq, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn finish_par(
        &mut self, id: Id, par: &mut Par, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn finish_if_else(
        &mut self, id: Id, if_else: &mut IfElse,
        comp_view: &mut ComponentViewMut, pool: &mut P
    ) -> Action {
        Action::None
    }

    /// Returns whether the traversal had any effect on the control.
    fn traverse_component(
        &mut self, comp: &mut Component, pool: &mut P
    ) -> bool {
        self.start_component(comp, pool);
        let (cfg, mut comp_view) = comp.as_view_mut();
        let did_modify = self.traverse_control(cfg, &mut comp_view, pool);
        self.finish_component(comp, pool);
        did_modify
    }

    /// Returns whether the traversal had any effect on the component.
    fn traverse_control(
        &mut self, mut control: Handle<Control>,
        comp_view: &mut ComponentViewMut, pool: &mut P
    ) -> bool {
        let mut did_modify = false;

        let id = control.id_in(pool);
        match control.deref_mut() {
            Control::Empty => Action::None,
            Control::For(for_) => self.start_for(id, for_, comp_view, pool),
            Control::Seq(seq) => self.start_seq(id, seq, comp_view, pool),
            Control::Par(par) => self.start_par(id, par, comp_view, pool),
            Control::IfElse(if_else) => {
                self.start_if_else(id, if_else, comp_view, pool)
            }
            Control::Enable(enable) => {
                self.start_enable(id, enable, comp_view, pool)
            }
        }
        .execute(&mut control, &mut did_modify);

        match control.deref_mut() {
            Control::Empty => {}
            Control::For(for_) => {
                did_modify |= self.traverse_control(for_.body, comp_view, pool);
            }
            Control::Seq(seq) => {
                for child in &seq.children {
                    did_modify |=
                        self.traverse_control(*child, comp_view, pool);
                }
            }
            Control::Par(par) => {
                for child in &par.children {
                    did_modify |=
                        self.traverse_control(*child, comp_view, pool);
                }
            }
            Control::IfElse(if_else) => {
                did_modify |=
                    self.traverse_control(if_else.true_branch, comp_view, pool);
                did_modify |= self.traverse_control(
                    if_else.false_branch,
                    comp_view,
                    pool
                );
            }
            Control::Enable(_) => {}
        }

        let id = control.id_in(pool);
        match control.deref_mut() {
            Control::Empty => Action::None,
            Control::For(for_) => self.finish_for(id, for_, comp_view, pool),
            Control::Seq(seq) => self.finish_seq(id, seq, comp_view, pool),
            Control::Par(par) => self.finish_par(id, par, comp_view, pool),
            Control::IfElse(if_else) => {
                self.finish_if_else(id, if_else, comp_view, pool)
            }
            Control::Enable(_) => Action::None
        }
        .execute(&mut control, &mut did_modify);

        did_modify
    }
}
