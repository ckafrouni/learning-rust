//! Parser module for lisp interpreter.
//!
//! Parser is a module to parse tokens into AST.
//!
//! We follow the Lisplike grammar:
//!
//! ```
//! program ::=
//!     *expr EOF
//! 
//! expr ::=
//!     NUMBER
//!     | STRING
//!     | NIL
//!     | IDENT
//!     | '(' paren_expr ')'
//! 
//! paren_expr ::=
//!     expr*
//!     | binary_op expr expr
//!     | unary_op expr
//!     | func_call
//!     | RESERVED expr
//!
//! binary_op ::= ADD | SUB | MUL | DIV
//! 
//! unary_op ::= 
//!     NEG
//!     | NOT
//! 
//! func_call ::= IDENT expr
//!
//! NUMBER ::= Token::Number
//! STRING ::= Token::String
//! NIL ::= Token::Nil
//!
//! ADD ::= Token::Add
//! SUB ::= Token::Sub
//! MUL ::= Token::Mul
//! DIV ::= Token::Div
//! 
//! RESERVED ::= Token::Reserved
//! ```

mod ast;

use ast::{AstNode, AstKind};
use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Token {
        self.tokens[self.pos].clone()
    }

    fn next_token(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }

    pub fn parse_prog(&mut self) -> AstNode {
        let mut node = AstNode::new_node(AstKind::Prog);

        while self.peek() != Token::EOF {
            let expr = self.parse_expr();
            node.add_child(expr);
        }

        node
    }

    pub fn parse_expr(&mut self) -> AstNode {
        let token = self.peek();

        match token {
            Token::Number(_) => self.parse_number(),
            Token::String(_) => self.parse_string(),
            Token::Nil => self.parse_nil(),
            Token::Ident(_) => self.parse_ident(),
            Token::LParen => self.parse_paren_expr(),
            _ => panic!("unexpected token {:?}", token),
        }
    }

    fn parse_paren_expr(&mut self) -> AstNode {
        let next = self.peek();
        dbg!(next.clone());

        match next {
            Token::LParen => {
                self.next_token(); // consume LParen
                match self.peek() {
                    Token::Add | Token::Sub | Token::Mul | Token::Div => {
                        self.parse_binary_op()
                    },
                    Token::Neg | Token::Not => self.parse_unary_op(),
                    Token::Ident(_) => self.parse_func_call(),
                    Token::Reserved(_) => self.parse_reserved_expr(),
                    _ => self.parse_expr(),
                }
            }
            _ => panic!("unexpected token {:?}", next),
        }
    }

    fn parse_number(&mut self) -> AstNode {
        let next = self.next_token();
        match next {
            Token::Number(n) => AstNode::new_leaf(AstKind::Number(n)),
            _ => panic!("unexpected token {:?}", next),
        }
    }

    fn parse_string(&mut self) -> AstNode {
        let next = self.next_token();
        match next {
            Token::String(s) => AstNode::new_leaf(AstKind::String(s)),
            _ => panic!("unexpected token {:?}", next),
        }
    }

    fn parse_nil(&mut self) -> AstNode {
        let next = self.next_token();
        match next {
            Token::Nil => AstNode::new_leaf(AstKind::Nil),
            _ => panic!("unexpected token {:?}", next),
        }
    }

    fn parse_ident(&mut self) -> AstNode {
        let next = self.next_token();
        match next {
            Token::Ident(s) => AstNode::new_leaf(AstKind::Ident(s)),
            _ => panic!("unexpected token {:?}", next),
        }
    }

    fn parse_binary_op(&mut self) -> AstNode {
        let mut node = match self.next_token() {
            Token::Add => AstNode::new_node(AstKind::Add),
            Token::Sub => AstNode::new_node(AstKind::Sub),
            Token::Mul => AstNode::new_node(AstKind::Mul),
            Token::Div => AstNode::new_node(AstKind::Div),
            op => panic!("unexpected token {:?}", op),
        };

        let left = self.parse_expr();
        node.add_child(left);

        let right = self.parse_expr();
        node.add_child(right);

        let next = self.next_token();
        if next != Token::RParen {
            panic!("unexpected token {:?}", next);
        }

        node
    }

    fn parse_unary_op(&mut self) -> AstNode {
        let mut node = match self.next_token() {
            Token::Ident(s) => AstNode::new_node(AstKind::Ident(s)),
            Token::Neg => AstNode::new_node(AstKind::Neg),
            Token::Not => AstNode::new_node(AstKind::Not),
            op => panic!("unexpected token {:?}", op),
        };

        let expr = self.parse_expr();
        node.add_child(expr);

        let next = self.next_token();
        if next != Token::RParen {
            panic!("unexpected token {:?}", next);
        }

        node
    }

    fn parse_func_call(&mut self) -> AstNode {
        let mut node = match self.next_token() {
            Token::Ident(s) => {
                let mut tmp = AstNode::new_node(AstKind::FnCall);
                tmp.add_child(AstNode::new_leaf(AstKind::Ident(s)));
                tmp
            },
            op => panic!("unexpected token {:?}", op),
        };

        let expr = self.parse_expr();
        node.add_child(expr);

        let next = self.next_token();
        if next != Token::RParen {
            panic!("unexpected token {:?}", next);
        }

        node
    }

    fn parse_reserved_expr(&mut self) -> AstNode {
        let mut node = match self.next_token() {
            Token::Reserved(s) => AstNode::new_node(AstKind::Reserved(s)),
            op => panic!("unexpected token {:?}", op),
        };

        let expr = self.parse_expr();
        node.add_child(expr);

        let next = self.next_token();
        if next != Token::RParen {
            panic!("unexpected token {:?}", next);
        }

        node
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokenizer::ReservedKeyword;

    #[test]
    fn test_parse_number() {
        let tokens = vec![Token::Number(1), Token::EOF];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();
        assert_eq!(ast, AstNode::new_leaf(AstKind::Number(1)));
    }

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::String("hello".to_string()), Token::EOF];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();
        assert_eq!(ast, AstNode::new_leaf(AstKind::String("hello".to_string())));
    }

    #[test]
    fn test_parse_nil() {
        let tokens = vec![Token::Nil, Token::EOF];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();
        assert_eq!(ast, AstNode::new_leaf(AstKind::Nil));
    }

    #[test]
    fn test_parse_ident() {
        let tokens = vec![Token::Ident("hello".to_string()), Token::EOF];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();
        assert_eq!(ast, AstNode::new_leaf(AstKind::Ident("hello".to_string())));
    }

    #[test]
    fn test_parse_binary_op() {
        let tokens = vec![
            Token::LParen,
            Token::Add,
            Token::Number(1),
            Token::Number(2),
            Token::RParen,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();
        let mut node = AstNode::new_node(AstKind::Add);
        node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        node.add_child(AstNode::new_leaf(AstKind::Number(2)));
        assert_eq!(ast, node);
    }

    #[test]
    fn test_parse_unary_op() {
        let tokens = vec![
            Token::LParen,
            Token::Neg,
            Token::Number(1),
            Token::RParen,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();
        let mut node = AstNode::new_node(AstKind::Neg);
        node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        assert_eq!(ast, node);
    }

    #[test]
    fn test_parse_func_call() {
        let tokens = vec![
            Token::LParen,
            Token::Ident("hello".to_string()),
            Token::Number(1),
            Token::RParen,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();
        let mut node = AstNode::new_node(AstKind::FnCall);
        node.add_child(AstNode::new_leaf(AstKind::Ident("hello".to_string())));
        node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        assert_eq!(ast, node);
    }

    #[test]
    fn test_parse_reserved_expr() {
        let tokens = vec![
            Token::LParen,
            Token::Reserved(ReservedKeyword::IF),
            Token::Number(1),
            Token::RParen,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr();
        let mut node = AstNode::new_node(AstKind::Reserved(ReservedKeyword::IF));
        node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        assert_eq!(ast, node);
    }

    #[test]
    fn test_parse_prog() {
        let tokens = vec![
            Token::LParen,
            Token::Reserved(ReservedKeyword::IF),
            Token::Number(1),
            Token::RParen,
            Token::LParen,
            Token::Reserved(ReservedKeyword::DEF),
            Token::Number(2),
            Token::RParen,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_prog();
        let mut node = AstNode::new_node(AstKind::Prog);
        let mut if_node = AstNode::new_node(AstKind::Reserved(ReservedKeyword::IF));
        if_node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        let mut def_node = AstNode::new_node(AstKind::Reserved(ReservedKeyword::DEF));
        def_node.add_child(AstNode::new_leaf(AstKind::Number(2)));
        node.add_child(if_node);
        node.add_child(def_node);
        assert_eq!(ast, node);
    }
}