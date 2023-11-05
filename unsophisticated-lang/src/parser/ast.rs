use crate::tokenizer::{BinaryOp, Literal, Token, UnaryOp};

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

pub trait AstVisitor {
    fn visit_literal(&mut self, node: &AstNode) -> Result<(), String>;
    fn visit_ident(&mut self, node: &AstNode) -> Result<(), String>;
    fn visit_nil(&mut self, node: &AstNode) -> Result<(), String>;
    fn visit_binary_op(&mut self, node: &AstNode) -> Result<(), String>;
    fn visit_unary_op(&mut self, node: &AstNode) -> Result<(), String>;
    fn visit_fn_call(&mut self, node: &AstNode) -> Result<(), String>;

    fn visit_token_error(&mut self, node: &AstNode) -> Result<(), String>;
    fn visit_parser_error(&mut self, node: &AstNode) -> Result<(), String>;
}

impl AstNode {
    pub fn accept(&self, visitor: &mut dyn AstVisitor) -> Result<(), String> {
        match self {
            AstNode::Literal(_) => visitor.visit_literal(self),
            AstNode::Ident(_) => visitor.visit_ident(self),
            AstNode::Nil => visitor.visit_nil(self),

            AstNode::BinaryOp { .. } => visitor.visit_binary_op(self),
            AstNode::UnaryOp { .. } => visitor.visit_unary_op(self),
            AstNode::FnCall { .. } => visitor.visit_fn_call(self),

            AstNode::TokenError(_) => visitor.visit_token_error(self),
            AstNode::ParserError(_, _) => visitor.visit_parser_error(self),
        }
    }
}

// TODO: orginize reserved keywords
