//! Interpreter module.
//!
//! This module contains the interpreter for the s-expressions language.

mod values;
mod visitor;

pub use values::Value;

use crate::parser;
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

use std::collections::HashMap;

pub struct AstInterpreter {
    pub stack: Vec<Value>,
    pub heap: HashMap<String, Value>,
}

impl AstInterpreter {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            heap: HashMap::new(),
        }
    }

    pub fn eval(&mut self, input: &str) -> Result<(), String> {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();

        self.eval_ast(ast)
    }

    pub fn eval_ast(&mut self, ast: parser::AstNode) -> Result<(), String> {
        ast.accept(self)
    }
}
