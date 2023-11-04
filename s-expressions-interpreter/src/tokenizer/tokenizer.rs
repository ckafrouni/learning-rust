use super::{BinaryOp::*, Delimiter::*, Literal::*, ReservedKw, Token, Tokenizer, UnaryOp::*};

impl Tokenizer {
    pub fn new<T>(input: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            input: input.into(),
            pos: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek_next_char()?;
        self.pos += 1;
        Some(c)
    }

    fn back_char(&mut self) {
        if self.pos == 0 {
            panic!("cannot back char");
        }
        self.pos -= 1;
    }

    fn peek_next_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    pub fn next_token(&mut self) -> Token {
        let c = self.next_char().unwrap_or('\0');

        match c {
            '\0' => Token::Delimiter(EOF),
            '(' => Token::Delimiter(LParen),
            ')' => Token::Delimiter(RParen),
            ' ' => Token::Delimiter(Space),
            '\n' => Token::Delimiter(NewLine),

            '+' => Token::BinaryOp(Add),
            '-' => Token::BinaryOp(Sub),
            '*' => Token::BinaryOp(Mul),
            '/' => Token::BinaryOp(Div),

            '~' => Token::UnaryOp(Neg),
            '!' => Token::UnaryOp(Not),

            '"' => {
                self.back_char();
                self.tokenize_string()
            }

            c if c.is_digit(10) => {
                self.back_char();
                self.tokenize_number()
            }

            c if c.is_alphabetic() => {
                self.back_char();
                let word = self.build_word();
                match ReservedKw::from_str(&word) {
                    Some(reserved_kw) => Token::ReservedKw(reserved_kw),
                    None => Token::Ident(word),
                }
            }

            _ => Token::TokenError(format!("unexpected char '{}'", c))
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Token::Delimiter(EOF) => {
                    tokens.push(Token::Delimiter(EOF));
                    break;
                }
                tok => tokens.push(tok),
            }
        }
        tokens
    }

    // -- region : token builders --

    fn tokenize_string(&mut self) -> Token {
        match self.next_char().unwrap() {
            '"' => {
                let mut s = String::new();
                while self.pos < self.input.len() {
                    let c = self.next_char().unwrap();
                    if c == '"' {
                        break;
                    }
                    s.push(c);
                }
                Token::Literal(StringLit(s))
            }
            _ => Token::TokenError("expected '\"'".to_string()),
        }
    }

    fn tokenize_number(&mut self) -> Token {
        match self.next_char().unwrap() {
            c if c.is_digit(10) => {
                let mut n = String::new();
                n.push(c);
                while self.pos < self.input.len() {
                    let c = self.input.chars().nth(self.pos).unwrap();
                    if !c.is_digit(10) {
                        break;
                    }
                    n.push(c);
                    self.pos += 1;
                }
                Token::Literal(NumberLit(n))
            }
            _ => Token::TokenError("expected digit".to_string()),
        }
    }

    fn build_word(&mut self) -> String {
        let mut word: String = String::new();
        while let Some(c) = self.next_char() {
            match c {
                c if c.is_alphanumeric() => word.push(c),
                _ => {
                    self.back_char();
                    break;
                }
            }
        }
        word
    }
}
