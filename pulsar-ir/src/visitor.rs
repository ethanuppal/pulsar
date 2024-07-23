//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::Component,
    control::{Control, For, IfElse, Par, Seq},
    from_ast::AsGeneratorPool,
    Ir
};

pub enum Action {
    None,
    Remove,
    Replace(Control)
}

impl Action {
    fn execute(self, control: &mut Control) {
        match self {
            Action::None => {}
            Action::Remove => {
                *control = Control::Empty;
            }
            Action::Replace(new_control) => {
                *control = new_control;
            }
        }
    }
}

pub trait Visitor<P: AsGeneratorPool> {
    #[allow(unused_variables)]
    fn start_component(&mut self, comp: &mut Component) {}

    #[allow(unused_variables)]
    fn start_for(&mut self, for_: &mut For, pool: &mut P) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn start_seq(&mut self, seq: &mut Seq, pool: &mut P) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn start_par(&mut self, par: &mut Par, pool: &mut P) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn start_if_else(&mut self, if_else: &mut IfElse, pool: &mut P) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn start_enable(&mut self, enable: &mut Ir, pool: &mut P) -> Action {
        Action::None
    }

    #[allow(unused_variables)]
    fn finish_for(&mut self, for_: &mut For, pool: &mut P) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn finish_seq(&mut self, seq: &mut Seq, pool: &mut P) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn finish_par(&mut self, par: &mut Par, pool: &mut P) -> Action {
        Action::None
    }
    #[allow(unused_variables)]
    fn finish_if_else(&mut self, if_else: &mut IfElse, pool: &mut P) -> Action {
        Action::None
    }

    fn traverse_component(&mut self, comp: &mut Component, pool: &mut P) {
        self.start_component(comp);
        self.traverse_control(&mut comp.cfg, pool);
    }

    fn traverse_control(&mut self, control: &mut Control, pool: &mut P) {
        match control {
            Control::Empty => Action::None,
            Control::For(for_) => self.start_for(for_, pool),
            Control::Seq(seq) => self.start_seq(seq, pool),
            Control::Par(par) => self.start_par(par, pool),
            Control::IfElse(if_else) => self.start_if_else(if_else, pool),
            Control::Enable(enable) => self.start_enable(enable, pool)
        }
        .execute(control);

        match control {
            Control::Empty => {}
            Control::For(for_) => {
                self.traverse_control(&mut for_.body, pool);
            }
            Control::Seq(seq) => {
                for child in &mut seq.children {
                    self.traverse_control(child, pool);
                }
            }
            Control::Par(par) => {
                for child in &mut par.children {
                    self.traverse_control(child, pool);
                }
            }
            Control::IfElse(if_else) => {
                self.traverse_control(&mut if_else.true_branch, pool);
                self.traverse_control(&mut if_else.false_branch, pool);
            }
            Control::Enable(_) => {}
        }

        match control {
            Control::Empty => Action::None,
            Control::For(for_) => self.finish_for(for_, pool),
            Control::Seq(seq) => self.finish_seq(seq, pool),
            Control::Par(par) => self.finish_par(par, pool),
            Control::IfElse(if_else) => self.finish_if_else(if_else, pool),
            Control::Enable(_) => Action::None
        }
        .execute(control);
    }
}
