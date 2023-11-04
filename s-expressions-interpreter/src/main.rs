//! # S-Expressions Interpreter
//!
//! This is a simple interpreter REPL for a custom S-Expressions.
//!
//! ## Usage
//!
//! ```bash
//! $ cargo install s-expressions-interpreter
//! $ s-expressions-interpreter
//! >> (+ 1 2)
//! 3
//! >> (let x 1)
//! 1
//! >> (let y 2)
//! 2
//! >> (+ x y)
//! 3
//! >> (def add (x y) (+ x y))
//! ()
//! >> (add 1 2)
//! 3
//! >> (def fact (n) (if (== n 0) 1 (* n (fact (- n 1)))))
//! ()
//! >> (fact 10)
//! 3628800
//! >> exit
//! ```
//!
//! ## Grammar
//!
//! The grammar is defined in the parser module.

pub mod parser;
pub mod repl;
pub mod tokenizer;
pub mod interpreter;

use repl::Repl;

fn main() {
    let mut repl = Repl::new(">> ");
    repl.run();
}
