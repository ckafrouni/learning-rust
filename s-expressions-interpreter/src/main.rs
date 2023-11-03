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

pub mod tokenizer;
pub mod parser;

use tokenizer::Tokenizer;
use parser::Parser;

const VERSION: &str = "0.0.1";
const AUTHOR: &str = "Christophe Kafrouni";

const PROMPT: &str = ">> ";

const WELCOME_MESSAGE: &str = "\
Welcome to the S-Expressions Interpreter!

This is a simple interpreter REPL for a custom S-Expressions.";

fn repl() {
    use std::io::Write;
    
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        }
        let tokens = Tokenizer::new(input).tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_expr();
        println!("{:?}", expr);
    }
}

fn main() {

    println!("{}\n\nVERSION: {}\nAUTHOR: {}\n", WELCOME_MESSAGE, VERSION, AUTHOR);

    repl();

}
