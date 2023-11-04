use crate::tokenizer::{Token,BinaryOp, Literal, UnaryOp};

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {

    // Leaf
    Literal(Literal),
    Ident(String),
    Nil,

    // Node
    BinaryOp {
        op: BinaryOp,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<AstNode>,
    },
    FnCall {
        ident: String,
        expr: Box<AstNode>,
    },

    // Error
    TokenError(Token),
    ParserError(String, Box<AstNode>),
}


// TODO: orginize reserved keywords