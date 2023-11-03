mod tokenizer;
mod parser;

use tokenizer::Tokenizer;
use parser::Parser;

fn parsing_an_expression() {
    // let expr = "1";
    // let expr = "(+ 1 2)";
    // let expr = "(+ 1 (- 2 3))";
    // let expr = "(+ 1 (- 2 ()))";
    // let expr = "(def (a))";
    // let expr = "(1)";
    let expr = "(+ 1 2)";
    println!("expr: {}", expr);
    let tokens = Tokenizer::new(expr.to_string()).tokenize();
    let ast = Parser::new(tokens).parse_expr();
    println!("Final AST: {:#?}", ast);
}

fn parsing_a_program() {
    let prog = "
    PROG
    (+ 1 2)
    (+ 2 1)
    ";
    println!("prog: {}", prog);
    let tokens = Tokenizer::new(prog.to_string()).tokenize();
    let ast = Parser::new(tokens).parse_prog();
    println!("Final AST: {:#?}", ast);
}

fn main() {

    println!("parsing an expression");
    parsing_an_expression();
    println!("");

    println!("parsing a program");
    parsing_a_program();
    println!("");
}
 //