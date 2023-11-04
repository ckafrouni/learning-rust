//! Tokenizer for the Lisp interpreter.

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Delimiter(Delimiter),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    Literal(Literal),
    Ident(String),
    ReservedKw(ReservedKw),
    TokenError(String),
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
pub enum Literal {
    NumberLit(String), // [0-9]+
    StringLit(String), // '"' [a-zA-Z0-9]* '"'
    BoolLit(String),   // 'true' | 'false'
    CharLit(String),   // [a-zA-Z]
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReservedKw {
    If,     // 'if'
    Else,   // 'else'
    ElseIf, // 'elseif'
    Def,    // 'def'
    Let,    // 'let'
    True,   // 'true'
    False,  // 'false'
    And,    // 'and'
    Or,     // 'or',
    Nil,    // 'nil'
}

impl ReservedKw {
    pub fn from_str(s: &str) -> Option<Self> {
        use ReservedKw::*;

        match s {
            "if" => Some(If),
            "else" => Some(Else),
            "elseif" => Some(ElseIf),
            "def" => Some(Def),
            "let" => Some(Let),
            "true" => Some(True),
            "false" => Some(False),
            "and" => Some(And),
            "or" => Some(Or),
            "nil" => Some(Nil),
            _ => None,
        }
    }
}

pub struct Tokenizer {
    input: String,
    pos: usize,
}

mod tokenizer;

// #[cfg(test)]
// mod tests;
