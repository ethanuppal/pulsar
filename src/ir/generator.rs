use super::{
    basic_block::BasicBlockCell, control_flow_graph::ControlFlowGraph,
    operand::Operand, variable::Variable, Ir
};
use crate::{
    frontend::{
        ast::{Expr, ExprValue, Node, NodeValue, Param},
        token::{Token, TokenType}
    },
    utils::context::Context
};
use std::fmt::Display;

pub enum GeneratedTopLevel {
    Function { name: Token, cfg: ControlFlowGraph }
}

impl Display for GeneratedTopLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Function { name, cfg } => {
                writeln!(f, "func {}:", name.value)?;
                write!(f, "{}", cfg)
            }
        }
    }
}

pub struct Generator {
    program: Box<dyn Iterator<Item = Node>>,
    env: Context<Variable>
}

impl Generator {
    pub fn new(program: Vec<Node>) -> Self {
        Self {
            program: Box::new(program.into_iter()),
            env: Context::new()
        }
    }

    fn gen_expr(&mut self, expr: &Expr, block: BasicBlockCell) -> Operand {
        match &expr.value {
            ExprValue::ConstantInt(value) => Operand::Constant(*value),
            ExprValue::BinOp(lhs, op, rhs) => {
                let lhs = self.gen_expr(&lhs, block.clone());
                let rhs = self.gen_expr(&rhs, block.clone());
                let result = Variable::new();
                match op.ty {
                    TokenType::Plus => {
                        block.as_mut().add(Ir::Add(result, lhs, rhs))
                    }
                    TokenType::Times => {
                        block.as_mut().add(Ir::Mul(result, lhs, rhs))
                    }
                    _ => todo!()
                }
                Operand::Variable(result)
            }
            ExprValue::BoundName(name) => {
                Operand::Variable(*self.env.find(name.value.clone()).unwrap())
            }
            _ => todo!()
        }
    }

    fn gen_node(&mut self, node: &Node, block: BasicBlockCell) {
        match &node.value {
            NodeValue::LetBinding {
                name,
                hint: _,
                value
            } => {
                let value_operand = self.gen_expr(value, block.clone());
                let name_var = Variable::new();
                self.env.bind(name.value.clone(), name_var);
                block.as_mut().add(Ir::Assign(name_var, value_operand));
            }
            NodeValue::Return { token: _, value } => {
                let value_operand = value
                    .as_ref()
                    .map(|value| self.gen_expr(value.as_ref(), block.clone()));
                block.as_mut().add(Ir::Return(value_operand));
            }
            _ => {}
        }
    }

    fn gen_func(
        &mut self, name: &Token, params: &Vec<Param>, body: &Vec<Node>
    ) -> GeneratedTopLevel {
        self.env.push();
        let cfg = ControlFlowGraph::new();
        let entry = cfg.entry();
        for (name, _) in params {
            let param_var = Variable::new();
            self.env.bind(name.value.clone(), param_var);
            entry.as_mut().add(Ir::GetParam(param_var));
        }
        for node in body {
            self.gen_node(node, entry.clone());
        }
        self.env.pop();
        GeneratedTopLevel::Function {
            name: name.clone(),
            cfg
        }
    }
}

impl Iterator for Generator {
    type Item = GeneratedTopLevel;

    fn next(&mut self) -> Option<GeneratedTopLevel> {
        match self.program.next()?.value {
            NodeValue::Function {
                name,
                params,
                ret: _,
                is_pure: _,
                body
            } => Some(self.gen_func(&name, &params, &body)),
            _ => None
        }
    }
}
