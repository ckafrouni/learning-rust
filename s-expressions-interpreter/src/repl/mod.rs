use crate::interpreter::{AstInterpreter, Value};
use crate::parser::Parser;
use crate::tokenizer::Tokenizer;

use std::io::Write;

pub struct Repl {
    interactive: bool,
    prompt: &'static str,
    last_result: Value,
    history: History,
    interpreter: AstInterpreter,
}

#[derive(Debug)]
pub struct History {
    history: Vec<String>,
    history_index: usize,
}

impl History {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            history_index: 0,
        }
    }

    pub fn add(&mut self, input: String) {
        self.history.push(input);
        self.history_index = self.history.len();
    }

    pub fn up(&mut self) -> Option<String> {
        if self.history_index > 0 {
            self.history_index -= 1;
        }
        self.history.get(self.history_index).cloned()
    }

    pub fn down(&mut self) -> Option<String> {
        if self.history_index < self.history.len() {
            self.history_index += 1;
        }
        self.history.get(self.history_index).cloned()
    }
}

#[allow(dead_code)]
impl Repl {
    pub fn interactive(prompt: &'static str) -> Self {
        Self {
            interactive: true,
            prompt,
            ..Self::non_interactive()
        }
    }

    pub fn non_interactive() -> Self {
        Self {
            interactive: false,
            prompt: "",
            last_result: Value::Nil,
            history: History::new(),
            interpreter: AstInterpreter::new(),
        }
    }

    fn read_line(&mut self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
    }

    /// REPL main loop
    ///
    /// This is where the REPL reads, evaluates and prints the results.
    ///
    /// ### Usage
    /// ```
    /// use s_expressions_interpreter::repl::Repl;
    ///
    /// let mut repl = Repl::interactive(">> ");
    /// repl.mainloop();
    /// ```
    pub fn mainloop(&mut self) -> Result<(), String> {
        // Loop
        loop {
            // Read
            if self.interactive {
                print!("{}", self.prompt);
                std::io::stdout().flush().unwrap();
            }
            let input = self.read_line();
            if self.interactive {
                if input.trim() == "exit" {
                    break;
                }
                self.history.add(input.clone());
            }

            // Evaluate
            let tokens = Tokenizer::new(input).tokenize();
            let node = Parser::new(tokens).parse_expr();
            if let Err(e) = self.interpreter.eval_ast(node) {
                if !self.interactive {
                    Err(e.clone())?
                }
                println!("Error: {}", e);
                continue;
            }

            self.last_result = self.interpreter.stack.pop().unwrap();

            // Print
            println!("{:?}", self.last_result);
            if self.interactive {
                println!("History: {:?}", self.history)
            }
        }

        Ok(())
    }
}
