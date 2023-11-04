use crate::parser::{Parser, AstNode};
use crate::tokenizer::Tokenizer;
use crate::interpreter::Interpreter;

use std::io::Write;

pub struct Repl {
    prompt: &'static str,
    last_result: String,
    history: Vec<String>,
    history_index: usize,
    interpreter: Interpreter,
}

#[allow(dead_code)]
impl Repl {
    pub fn new(prompt: &'static str) -> Self {
        Self {
            prompt,
            last_result: String::new(),
            history: Vec::new(),
            history_index: 0,
            interpreter: Interpreter::new(),
        }
    }

    fn history_up(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
        }
    }

    fn history_down(&mut self) {
        if self.history_index < self.history.len() {
            self.history_index += 1;
        }
    }

    fn display_prompt(&self) {
        print!("{}", self.prompt);
        std::io::stdout().flush().unwrap();
    }

    fn display_last_result(&self) {
        println!("{}", self.last_result);
    }

    fn read_line(&mut self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop();
        input
    }

    fn add_to_history(&mut self, input: String) {
        self.history.push(input);
        self.history_index = self.history.len();
    }

    fn eval(&mut self, node: AstNode) {
        println!("eval: {:?}", node);
    }

    pub fn mainloop(&mut self) {
        loop {
            self.display_prompt();

            let input = self.read_line();
            self.add_to_history(input.clone());

            let tokens = Tokenizer::new(input).tokenize();
            let node = Parser::new(tokens).parse_expr();

            self.eval(node);
            println!("History: {:?}", self.history)
        }
    }
}
