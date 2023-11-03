mod tokenizer;
mod parser;

use tokenizer::Tokenizer;
use parser::Parser;

fn main() {
    println!("Hello, world!");
    // let expr = "1";
    // let expr = "(+ 1 2)";
    // let expr = "(+ 1 (- 2 3))";
    let expr = "(+ 1 (- 2 ()))";

    let tokens = Tokenizer::new(expr.to_string()).tokenize();
    dbg!(&tokens);

    let ast = Parser::new(tokens).parse();
    println!("Final AST: {:#?}", ast)
}
