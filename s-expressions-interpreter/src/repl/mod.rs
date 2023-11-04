use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

use std::io::Write;

pub struct Repl {
    prompt: &'static str,
    last_result: String,
    history: Vec<String>,
    history_index: usize,
}

#[allow(dead_code)]
impl Repl {
    pub fn new(prompt: &'static str) -> Self {
        Self {
            prompt,
            last_result: String::new(),
            history: Vec::new(),
            history_index: 0,
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

    pub fn run(&mut self) {
        loop {
            self.display_prompt();

            let input = self.read_line();
            self.add_to_history(input.clone());

            let tokens = Tokenizer::new(input).tokenize();
            let expr = Parser::new(tokens).parse_expr();

            match expr {
                Ok(expr) => {
                    // let result = expr.eval();
                    let result = expr.to_string();
                    self.last_result = result.clone();

                    self.display_last_result();
                    println!("History: {:?}", self.history)
                }
                Err(err) => {
                    println!("Error: {}", err);
                    continue;
                }
            }

        }
    }
}
