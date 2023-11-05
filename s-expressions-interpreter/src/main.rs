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
pub mod tokenizer;

pub mod repl;
pub mod interpreter;

use repl::Repl;

// fn test() {
//     let mut tok = tokenizer::Tokenizer::new("(+ (- 1 2 2)");
//     let tokens = tok.tokenize();
//     dbg!(tokens.clone());
//     let mut parser = parser::Parser::new(tokens);
//     let ast = parser.parse_expr();
//     dbg!(ast.clone());
// }

fn main() {
    // test();
    let mut repl = Repl::interactive(">> ");

    if let Err(e) = repl.mainloop() {
        println!("Error: {}", e);
    }
}
