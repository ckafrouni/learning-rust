//! Tokenizer for the Lisp interpreter.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Delimiter(Delimiter),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Number(i32),    // [0-9]+
    String(String), // '"' [a-zA-Z0-9]* '"'
    Nil,            // '()'
    Ident(String),  // [a-zA-Z]+
    Keyword(ReservedKeyword),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Delimiter {
    Space,   // ' '
    NewLine, // '\n'
    LParen,  // '('
    RParen,  // ')'
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add, // '+'
    Sub, // '-'
    Mul, // '*'
    Div, // '/'
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Neg, // '~'
    Not, // '!'
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReservedKeyword {
    If,     // 'if'
    Else,   // 'else'
    ElseIf, // 'elseif'
    Def,    // 'def'
    Let,    // 'let'
    True,   // 'true'
    False,  // 'false'
    And,    // 'and'
    Or,     // 'or',
}

use BinaryOp::*;
use Delimiter::*;
use ReservedKeyword::*;
use UnaryOp::*;

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

    fn next_char(&mut self) -> char {
        let c = self.input.chars().nth(self.pos).unwrap();
        self.pos += 1;
        c
    }

    fn back_char(&mut self) {
        self.pos -= 1;
    }

    fn peek_char(&self) -> char {
        self.input.chars().nth(self.pos).unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        if self.pos >= self.input.len() {
            return Token::Delimiter(EOF);
        }

        let c = self.next_char();

        let tok = match c {
            '(' => {
                if self.pos < self.input.len() {
                    if self.peek_char() == ')' {
                        self.next_char();
                        return Token::Nil;
                    }
                }
                Token::Delimiter(LParen)
            }
            ')' => Token::Delimiter(RParen),
            '+' => Token::BinaryOp(Add),
            '-' => Token::BinaryOp(Sub),
            '*' => Token::BinaryOp(Mul),
            '/' => Token::BinaryOp(Div),
            '~' => Token::UnaryOp(Neg),
            '!' => Token::UnaryOp(Not),
            ' ' => Token::Delimiter(Space),
            '\n' => Token::Delimiter(NewLine),
            _ => {
                if c.is_digit(10) {
                    let mut num = c.to_string();
                    while self.pos < self.input.len() {
                        if !self.peek_char().is_digit(10) {
                            break;
                        }
                        num.push(self.next_char());
                    }
                    Token::Number(num.parse::<i32>().unwrap())
                } else if c == '"' {
                    let mut s = String::new();
                    while self.pos < self.input.len() {
                        let c = self.next_char();
                        if c == '"' {
                            break;
                        }
                        s.push(c);
                    }
                    Token::String(s)
                } else if c.is_alphabetic() {
                    self.back_char();
                    self.tokenize_ident_or_reserved()
                } else {
                    panic!("unexpected character: {}", c);
                }
            }
        };
        tok
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            match token {
                Token::Delimiter(EOF) => {
                    tokens.push(token);
                    break;
                }
                // Token::Delimiter(Space) | Token::Delimiter(NewLine) => continue, // TODO: do not skip space, newline, we should handle it in parser (maybe combine spaces into one token with count)
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
            "if" => Token::Keyword(If),
            "else" => Token::Keyword(Else),
            "elseif" => Token::Keyword(ElseIf),
            "def" => Token::Keyword(Def),
            "let" => Token::Keyword(Let),
            "true" => Token::Keyword(True),
            "false" => Token::Keyword(False),
            "and" => Token::Keyword(And),
            "or" => Token::Keyword(Or),

            _ => Token::Ident(ident),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Token::Delimiter(Space) => "Space",
            Token::Delimiter(NewLine) => "NewLine",
            Token::Delimiter(LParen) => "LParen",
            Token::Delimiter(RParen) => "RParen",
            Token::Delimiter(EOF) => "EOF",
            Token::BinaryOp(Add) => "Add",
            Token::BinaryOp(Sub) => "Sub",
            Token::BinaryOp(Mul) => "Mul",
            Token::BinaryOp(Div) => "Div",
            Token::UnaryOp(Neg) => "Neg",
            Token::UnaryOp(Not) => "Not",
            Token::Number(n) => return write!(f, "{}", n),
            Token::String(s) => return write!(f, "{}", s),
            Token::Nil => "Nil",
            Token::Ident(s) => return write!(f, "{}", s),
            Token::Keyword(reserved_keyword) => return write!(f, "{}", reserved_keyword),
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for ReservedKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            If => "if",
            Else => "else",
            ElseIf => "elseif",
            Def => "def",
            Let => "let",
            True => "true",
            False => "false",
            And => "and",
            Or => "or",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use Token as T;

    #[test]
    fn test_next_token() {
        let mut tokenizer = Tokenizer::new("(+ 1 2)".to_string());
        assert_eq!(tokenizer.next_token(), T::Delimiter(LParen));
        assert_eq!(tokenizer.next_token(), T::BinaryOp(Add));
        assert_eq!(tokenizer.next_token(), T::Delimiter(Space));
        assert_eq!(tokenizer.next_token(), T::Number(1));
        assert_eq!(tokenizer.next_token(), T::Delimiter(Space));
        assert_eq!(tokenizer.next_token(), T::Number(2));
        assert_eq!(tokenizer.next_token(), T::Delimiter(RParen));
        assert_eq!(tokenizer.next_token(), T::Delimiter(EOF));
    }

    #[test]
    fn test_tokenize() {
        let mut tokenizer = Tokenizer::new("(+ 1 2)".to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                T::Delimiter(LParen),
                T::BinaryOp(Add),
                T::Delimiter(Space),
                T::Number(1),
                T::Delimiter(Space),
                T::Number(2),
                T::Delimiter(RParen),
                T::Delimiter(EOF),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_newline() {
        let mut tokenizer = Tokenizer::new("(+ 1 2)\n(+ 3 4)".to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                T::Delimiter(LParen),
                T::BinaryOp(Add),
                T::Delimiter(Space),
                T::Number(1),
                T::Delimiter(Space),
                T::Number(2),
                T::Delimiter(RParen),
                T::Delimiter(NewLine),
                T::Delimiter(LParen),
                T::BinaryOp(Add),
                T::Delimiter(Space),
                T::Number(3),
                T::Delimiter(Space),
                T::Number(4),
                T::Delimiter(RParen),
                T::Delimiter(EOF),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_space() {
        let mut tokenizer = Tokenizer::new("(+ 1 2) (+ 3 4)".to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                T::Delimiter(LParen),
                T::BinaryOp(Add),
                T::Delimiter(Space),
                T::Number(1),
                T::Delimiter(Space),
                T::Number(2),
                T::Delimiter(RParen),
                T::Delimiter(Space),
                T::Delimiter(LParen),
                T::BinaryOp(Add),
                T::Delimiter(Space),
                T::Number(3),
                T::Delimiter(Space),
                T::Number(4),
                T::Delimiter(RParen),
                T::Delimiter(EOF),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_string() {
        let mut tokenizer = Tokenizer::new("(+ \"abc\" \"def\")".to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                T::Delimiter(LParen),
                T::BinaryOp(Add),
                T::Delimiter(Space),
                T::String("abc".to_string()),
                T::Delimiter(Space),
                T::String("def".to_string()),
                T::Delimiter(RParen),
                T::Delimiter(EOF),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_nil() {
        let mut tokenizer = Tokenizer::new("()".to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, vec![T::Nil, T::Delimiter(EOF),]);
    }

    #[test]
    fn test_tokenize_with_ident() {
        let mut tokenizer = Tokenizer::new("(+ x y)".to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                T::Delimiter(LParen),
                T::BinaryOp(Add),
                T::Delimiter(Space),
                T::Ident("x".to_string()),
                T::Delimiter(Space),
                T::Ident("y".to_string()),
                T::Delimiter(RParen),
                T::Delimiter(EOF),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_reserved() {
        let mut tokenizer = Tokenizer::new("(if true false)".to_string());
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                T::Delimiter(LParen),
                T::Keyword(If),
                T::Delimiter(Space),
                T::Keyword(True),
                T::Delimiter(Space),
                T::Keyword(False),
                T::Delimiter(RParen),
                T::Delimiter(EOF),
            ]
        );
    }
}
