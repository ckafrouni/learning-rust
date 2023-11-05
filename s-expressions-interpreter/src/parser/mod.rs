//! Parser module for lisp interpreter.
//!
//! Parser is a module to parse tokens into AST.
//!
//! We follow the Lisplike grammar:
//!
//! ```
//! program ::=
//!     *expr EOF
//!
//! expr ::=
//!     LITERAL
//!     | NIL
//!     | IDENT
//!     | '(' paren_expr ')'
//!
//! paren_expr ::=
//!     expr*
//!     | binary_op expr expr
//!     | unary_op expr
//!     | func_call
//!     | RESERVED expr
//!
//! binary_op ::= ADD | SUB | MUL | DIV
//!
//! unary_op ::=
//!     NEG
//!     | NOT
//!
//! func_call ::= IDENT expr
//!
//! NUMBER ::= Token::Number
//! STRING ::= Token::String
//! NIL ::= Token::Nil
//!
//! ADD ::= Token::Add
//! SUB ::= Token::Sub
//! MUL ::= Token::Mul
//! DIV ::= Token::Div
//!
//! RESERVED ::= Token::Reserved
//! ```

mod parser;
mod ast;

// #[cfg(test)]
// mod tests;

pub use ast::{AstNode, AstVisitor};

use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors_pos: Vec<usize>,
}

