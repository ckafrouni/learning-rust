//! Tokenizer for the Lisp interpreter.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Delimiters
    Space, // ' '
    NewLine, // '\n'
    LParen, // '('
    RParen, // ')'
    EOF,
    // Binary Operators
    Add, // '+'
    Sub, // '-'
    Mul, // '*'
    Div, // '/'
    // Unary Operators
    Neg, // '~'
    Not, // '!'
    // Literals
    Number(i32), // [0-9]+
    String(String), // '"' [a-zA-Z0-9]* '"'
    Nil, // '()'
    // Identifiers
    Ident(String), // [a-zA-Z]+

    Reserved(ReservedKeyword),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReservedKeyword {
    IF, // 'if'
    ELSE, // 'else'
    ELSEIF, // 'elseif'
    DEF, // 'def'
    LET, // 'let'
    TRUE, // 'true'
    FALSE, // 'false'
    AND, // 'and'
    OR, // 'or',
}

pub struct Tokenizer {
    input: String,
    pos: usize,
}

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

    pub fn next_token(&mut self) -> Token {

        
        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        let c = self.input.chars().nth(self.pos).unwrap();
        self.pos += 1;

        match c {
            '(' => {
                if self.pos < self.input.len() {
                    let c = self.input.chars().nth(self.pos).unwrap();
                    if c == ')' {
                        self.pos += 1;
                        return Token::Nil;
                    }
                }
                Token::LParen
            },
            ')' => Token::RParen,
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            '~' => Token::Neg,
            '!' => Token::Not,
            ' ' => Token::Space,
            '\n' => Token::NewLine,
            _ => {
                if c.is_digit(10) {
                    let mut num = c.to_string();
                    while self.pos < self.input.len() {
                        let c = self.input.chars().nth(self.pos).unwrap();
                        if !c.is_digit(10) {
                            break;
                        }
                        num.push(c);
                        self.pos += 1;
                    }
                    Token::Number(num.parse::<i32>().unwrap())
                } else if c == '"' {
                    let mut s = String::new();
                    while self.pos < self.input.len() {
                        let c = self.input.chars().nth(self.pos).unwrap();
                        self.pos += 1;
                        if c == '"' {
                            break;
                        }
                        s.push(c);
                    }
                    Token::String(s)
                } else if c.is_alphabetic() {
                    self.tokenize_ident_or_reserved()
                } else {
                    panic!("unexpected character: {}", c);
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            match token {
                Token::EOF => {
                    tokens.push(token);
                    break
                },
                Token::Space | Token::NewLine => continue,
                _ => tokens.push(token),
            }
        }
        tokens
    }

    fn tokenize_ident_or_reserved(&mut self) -> Token {
        let mut ident = String::new();
        while self.pos < self.input.len() {
            let c = self.input.chars().nth(self.pos).unwrap();
            if !c.is_alphabetic() {
                break;
            }
            ident.push(c);
            self.pos += 1;
        }
        match ident.as_str() {
            "if" => Token::Reserved(ReservedKeyword::IF),
            "else" => Token::Reserved(ReservedKeyword::ELSE),
            "elseif" => Token::Reserved(ReservedKeyword::ELSEIF),
            "def" => Token::Reserved(ReservedKeyword::DEF),
            "let" => Token::Reserved(ReservedKeyword::LET),
            "true" => Token::Reserved(ReservedKeyword::TRUE),
            "false" => Token::Reserved(ReservedKeyword::FALSE),
            "and" => Token::Reserved(ReservedKeyword::AND),
            "or" => Token::Reserved(ReservedKeyword::OR),

            _ => Token::Ident(ident),
        }
    }
}

/// Implement Display for Token
/// Example:
///    - Token::LParen
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Token::Space => "Space",
            Token::NewLine => "NewLine",
            Token::LParen => "LParen",
            Token::RParen => "RParen",
            Token::EOF => "EOF",
            Token::Add => "Add",
            Token::Sub => "Sub",
            Token::Mul => "Mul",
            Token::Div => "Div",
            Token::Neg => "Neg",
            Token::Not => "Not",
            Token::Number(n) => return write!(f, "{}", n),
            Token::String(s) => return write!(f, "{}", s),
            Token::Nil => "Nil",
            Token::Ident(s) => return write!(f, "{}", s),
            Token::Reserved(k) => return write!(f, "{}", k),
        };
        write!(f, "{}", s)
    }
    
}

/// Implement Display for ReservedKeyword
/// Example:
///   - ReservedKeyword::IF
impl std::fmt::Display for ReservedKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            ReservedKeyword::IF => "if",
            ReservedKeyword::ELSE => "else",
            ReservedKeyword::ELSEIF => "elseif",
            ReservedKeyword::DEF => "def",
            ReservedKeyword::LET => "let",
            ReservedKeyword::TRUE => "true",
            ReservedKeyword::FALSE => "false",
            ReservedKeyword::AND => "and",
            ReservedKeyword::OR => "or",
        };
        write!(f, "{}", s)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let mut tokenizer = Tokenizer::new("(+ 1 2)".to_string());
        assert_eq!(tokenizer.next_token(), Token::LParen);
        assert_eq!(tokenizer.next_token(), Token::Add);
        assert_eq!(tokenizer.next_token(), Token::Space);
        assert_eq!(tokenizer.next_token(), Token::Number(1));
        assert_eq!(tokenizer.next_token(), Token::Space);
        assert_eq!(tokenizer.next_token(), Token::Number(2));
        assert_eq!(tokenizer.next_token(), Token::RParen);
        assert_eq!(tokenizer.next_token(), Token::EOF);
    }

    #[test]
    fn test_tokenize() {
        let mut tokenizer = Tokenizer::new("(+ 1 2)".to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::Add,
                Token::Number(1),
                Token::Number(2),
                Token::RParen,
                Token::EOF,
            ]
        );
    }
}
