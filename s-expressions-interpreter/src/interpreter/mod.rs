//! Interpreter module.
//!
//! This module contains the interpreter for the s-expressions language.

mod values;
mod interpreter;

use values::Value;

use crate::parser;
use crate::parser::{AstNode, Parser};
use crate::tokenizer::Tokenizer;

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
        let ast = parser.parse_expr();

        // TODO: maybe we should check if the ast contains any errors
        // (we have AstNode::TokenError and AstNode::ParserError)
        // We could implement an AstChecker with AstVisitor trait to walk
        // the ast and print errors if any. Maybe the interpreter shouldn't
        // be responsible for this. The REPL binary could be, same for the
        // interpreter binary.

        self.eval_ast(ast)
    }

    pub fn eval_ast(&self, ast: parser::AstNode) -> Result<Value, String> {
        todo!("eval_ast")
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
