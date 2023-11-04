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

use crate::tokenizer::Token;
use crate::tokenizer::{BinaryOp::*, Delimiter::*, UnaryOp::*};
use ast::{AstKind, AstNode};

#[derive(Debug, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    // TODO: 
    //  Fix spaces handling
    //  Add more tests
    //  Add more AST nodes
    //  Better representation of AST nodes and tokens
    //  Evaluate AST nodes

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

    pub fn parse_prog(&mut self) -> Result<AstNode, String> {
        let mut node = AstNode::new_node(AstKind::Prog);

        while self.peek() != Token::Delimiter(EOF) {
            let expr = self.parse_expr()?;
            node.add_child(expr);
        }

        Ok(node)
    }

    pub fn parse_expr(&mut self) -> Result<AstNode, String> {
        let node = match self.peek() {
            Token::Number(_) => self.parse_number()?,
            Token::String(_) => self.parse_string()?,
            Token::Nil => self.parse_nil()?,
            Token::Ident(_) => self.parse_ident()?,
            Token::Delimiter(LParen) => self.parse_paren_expr()?,
            tk => Err(format!("unexpected token {:?}", tk))?,
        };
        Ok(node)
    }

    fn parse_paren_expr(&mut self) -> Result<AstNode, String> {
        let node = match self.next_token() {
            Token::Delimiter(LParen) => match self.peek() {
                Token::BinaryOp(_) => self.parse_binary_op()?,
                Token::UnaryOp(_) => self.parse_unary_op()?,
                Token::Ident(_) => self.parse_func_call()?,
                Token::Keyword(_) => self.parse_reserved_expr()?,
                _ => self.parse_expr()?,
            },
            tk => Err(format!(
                "unexpected token {:?} in parenthesised expression, expected {:?}",
                tk,
                Token::Delimiter(LParen)
            ))?,
        };
        Ok(node)
    }

    fn parse_number(&mut self) -> Result<AstNode, String> {
        let node = match self.next_token() {
            Token::Number(n) => AstNode::new_leaf(AstKind::Number(n)),
            tk => Err(format!("unexpected token {:?}, expected a number", tk))?,
        };
        Ok(node)
    }

    fn parse_string(&mut self) -> Result<AstNode, String> {
        let node = match self.next_token() {
            Token::String(s) => AstNode::new_leaf(AstKind::String(s)),
            tk => Err(format!("unexpected token {:?}, expected a string", tk))?,
        };
        Ok(node)
    }

    fn parse_nil(&mut self) -> Result<AstNode, String> {
        let node = match self.next_token() {
            Token::Nil => AstNode::new_leaf(AstKind::Nil),
            tk => Err(format!("unexpected token {:?}, expected nil", tk))?,
        };
        Ok(node)
    }

    fn parse_ident(&mut self) -> Result<AstNode, String> {
        let node = match self.next_token() {
            Token::Ident(s) => AstNode::new_leaf(AstKind::Ident(s)),
            tk => Err(format!("unexpected token {:?}, expected an identifier", tk))?,
        };
        Ok(node)
    }

    fn parse_binary_op(&mut self) -> Result<AstNode, String> {
        let mut node = match self.next_token() {
            Token::BinaryOp(Add) => AstNode::new_node(AstKind::Add),
            Token::BinaryOp(Sub) => AstNode::new_node(AstKind::Sub),
            Token::BinaryOp(Mul) => AstNode::new_node(AstKind::Mul),
            Token::BinaryOp(Div) => AstNode::new_node(AstKind::Div),
            tk => Err(format!("unexpected token {:?}", tk))?,
        };

        let left = self.parse_expr()?;
        node.add_child(left);

        let right = self.parse_expr()?;
        node.add_child(right);

        let next = self.next_token();
        if next != Token::Delimiter(RParen) {
            Err(format!(
                "unexpected token {:?}, expected {:?}",
                next,
                Token::Delimiter(RParen)
            ))?
            // panic!("unexpected token {:?}", next);
        }
        Ok(node)
    }

    fn parse_unary_op(&mut self) -> Result<AstNode, String> {
        let mut node = match self.next_token() {
            Token::UnaryOp(Neg) => AstNode::new_node(AstKind::Neg),
            Token::UnaryOp(Not) => AstNode::new_node(AstKind::Not),
            tk => Err(format!("unexpected token {:?}", tk))?,
        };

        let expr = self.parse_expr()?;
        node.add_child(expr);
        let next = self.next_token();
        if next != Token::Delimiter(RParen) {
            panic!("unexpected token {:?}", next);
        }
        Ok(node)
    }

    fn parse_func_call(&mut self) -> Result<AstNode, String> {
        let mut node = match self.next_token() {
            Token::Ident(s) => {
                let mut tmp = AstNode::new_node(AstKind::FnCall);
                tmp.add_child(AstNode::new_leaf(AstKind::Ident(s)));
                tmp
            }
            tk => Err(format!("unexpected token {:?}", tk))?,
        };

        let expr = self.parse_expr()?;
        node.add_child(expr);

        let next = self.next_token();
        if next != Token::Delimiter(RParen) {
            panic!("unexpected token {:?}", next);
        }

        Ok(node)
    }

    fn parse_reserved_expr(&mut self) -> Result<AstNode, String> {
        let mut node = match self.next_token() {
            Token::Keyword(kw) => AstNode::new_node(AstKind::Reserved(kw)),
            tk => Err(format!("unexpected token {:?}", tk))?,
        };

        let expr = self.parse_expr()?;
        node.add_child(expr);
        let next = self.next_token();
        if next != Token::Delimiter(RParen) {
            Err(format!(
                "unexpected token {:?}, expected {:?}",
                next,
                Token::Delimiter(RParen)
            ))?;
        }
        Ok(node)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tokenizer::ReservedKeyword::*;

    #[test]
    fn test_parse_number() {
        let tokens = vec![Token::Number(1), Token::Delimiter(EOF)];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr().unwrap();
        assert_eq!(ast, AstNode::new_leaf(AstKind::Number(1)));
    }

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::String("hello".to_string()), Token::Delimiter(EOF)];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr().unwrap();
        assert_eq!(ast, AstNode::new_leaf(AstKind::String("hello".to_string())));
    }

    #[test]
    fn test_parse_nil() {
        let tokens = vec![Token::Nil, Token::Delimiter(EOF)];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr().unwrap();
        assert_eq!(ast, AstNode::new_leaf(AstKind::Nil));
    }

    #[test]
    fn test_parse_ident() {
        let tokens = vec![Token::Ident("hello".to_string()), Token::Delimiter(EOF)];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr().unwrap();
        assert_eq!(ast, AstNode::new_leaf(AstKind::Ident("hello".to_string())));
    }

    #[test]
    fn test_parse_binary_op() {
        let tokens = vec![
            Token::Delimiter(LParen),
            Token::BinaryOp(Add),
            Token::Number(1),
            Token::Number(2),
            Token::Delimiter(RParen),
            Token::Delimiter(EOF),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr().unwrap();
        let mut node = AstNode::new_node(AstKind::Add);
        node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        node.add_child(AstNode::new_leaf(AstKind::Number(2)));
        assert_eq!(ast, node);
    }

    #[test]
    fn test_parse_unary_op() {
        let tokens = vec![
            Token::Delimiter(LParen),
            Token::UnaryOp(Neg),
            Token::Number(1),
            Token::Delimiter(RParen),
            Token::Delimiter(EOF),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr().unwrap();
        let mut node = AstNode::new_node(AstKind::Neg);
        node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        assert_eq!(ast, node);
    }

    #[test]
    fn test_parse_func_call() {
        let tokens = vec![
            Token::Delimiter(LParen),
            Token::Ident("hello".to_string()),
            Token::Number(1),
            Token::Delimiter(RParen),
            Token::Delimiter(EOF),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr().unwrap();
        let mut node = AstNode::new_node(AstKind::FnCall);
        node.add_child(AstNode::new_leaf(AstKind::Ident("hello".to_string())));
        node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        assert_eq!(ast, node);
    }

    #[test]
    fn test_parse_reserved_expr() {
        let tokens = vec![
            Token::Delimiter(LParen),
            Token::Keyword(If),
            Token::Number(1),
            Token::Delimiter(RParen),
            Token::Delimiter(EOF),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_expr().unwrap();
        let mut node = AstNode::new_node(AstKind::Reserved(If));
        node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        assert_eq!(ast, node);
    }

    #[test]
    fn test_parse_prog() {
        let tokens = vec![
            Token::Delimiter(LParen),
            Token::Keyword(If),
            Token::Number(1),
            Token::Delimiter(RParen),
            Token::Delimiter(LParen),
            Token::Keyword(Def),
            Token::Number(2),
            Token::Delimiter(RParen),
            Token::Delimiter(EOF),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse_prog().unwrap();
        let mut node = AstNode::new_node(AstKind::Prog);
        let mut if_node = AstNode::new_node(AstKind::Reserved(If));
        if_node.add_child(AstNode::new_leaf(AstKind::Number(1)));
        let mut def_node = AstNode::new_node(AstKind::Reserved(Def));
        def_node.add_child(AstNode::new_leaf(AstKind::Number(2)));
        node.add_child(if_node);
        node.add_child(def_node);
        assert_eq!(ast, node);
    }
}
