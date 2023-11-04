//! Interpreter module.
//!
//! This module contains the interpreter for the s-expressions language.

mod values;

use values::Value;

use crate::parser;
use crate::parser::{AstKind, AstNode, Parser};
use crate::tokenizer::Tokenizer;
use crate::tokenizer::{ReservedKeyword};

use std::cell::RefCell;

#[derive(Debug, PartialEq)]
pub struct Interpreter {
    heap: Vec<Value>,
    stack: RefCell<Vec<Value>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            heap: Vec::new(),
            stack: RefCell::new(Vec::new()),
        }
    }

    pub fn eval(&mut self, input: &str) -> Result<Value, String> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr()?;

        self.eval_ast(ast)
    }

    pub fn eval_ast(&self, ast: parser::AstNode) -> Result<Value, String> {
        let result = match ast {
            AstNode::Leaf { kind } => match kind {
                AstKind::Number(n) => Value::Number(n as f64),
                AstKind::String(s) => Value::String(s),
                AstKind::Ident(s) => Value::Symbol(s),
                AstKind::Nil => Value::Nil,
                AstKind::Reserved(token) => match token {
                    ReservedKeyword::True => Value::Bool(true),
                    ReservedKeyword::False => Value::Bool(false),
                    _ => Err(format!("Invalid reserved keyword: {:?}", token))?,
                },
                _ => Err(format!("Invalid leaf node: {:?}", kind))?,
            },
            AstNode::Node { kind, children } => match kind {
                AstKind::Add => {
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[0].clone())?);
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[1].clone())?);
                    self.eval_add()?
                }
                AstKind::Sub => {
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[0].clone())?);
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[1].clone())?);
                    self.eval_sub()?
                }
                AstKind::Mul => {
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[0].clone())?);
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[1].clone())?);
                    self.eval_mul()?
                }

                AstKind::Div => {
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[0].clone())?);
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[1].clone())?);
                    self.eval_div()?
                }

                AstKind::Neg => {
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[0].clone())?);
                    self.eval_neg()?
                }
                AstKind::Not => {
                    self.stack
                        .borrow_mut()
                        .push(self.eval_ast(children[0].clone())?);
                    self.eval_not()?
                }

                AstKind::FnCall => todo!(),
                AstKind::Reserved(_) => todo!(),
                AstKind::Prog => todo!(),
                _ => Err(format!("Invalid node: {:?}", kind))?,
            },
        };
        Ok(result)
    }

    fn eval_add(&self) -> Result<Value, String> {
        let mut stack = self.stack.borrow_mut();
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        Ok(Value::Number(f64::from(lhs) + f64::from(rhs)))
    }

    fn eval_sub(&self) -> Result<Value, String> {
        let mut stack = self.stack.borrow_mut();
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        Ok(Value::Number(f64::from(lhs) - f64::from(rhs)))
    }

    fn eval_mul(&self) -> Result<Value, String> {
        let mut stack = self.stack.borrow_mut();
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        Ok(Value::Number(f64::from(lhs) * f64::from(rhs)))
    }

    fn eval_div(&self) -> Result<Value, String> {
        let mut stack = self.stack.borrow_mut();
        let lhs = stack.pop().unwrap();
        let rhs = stack.pop().unwrap();
        Ok(Value::Number(f64::from(lhs) / f64::from(rhs)))
    }

    fn eval_neg(&self) -> Result<Value, String> {
        let mut stack = self.stack.borrow_mut();
        let val = stack.pop().unwrap();
        Ok(Value::Number(-f64::from(val)))
    }

    fn eval_not(&self) -> Result<Value, String> {
        let mut stack = self.stack.borrow_mut();
        let val = stack.pop().unwrap();
        Ok(Value::Bool(!bool::from(val)))
    }
}
