// use crate::tokenizer::{BinaryOp::*, Delimiter::*, UnaryOp::*};

use super::{AstNode, Parser};
use crate::tokenizer::{BinaryOp, Delimiter, Literal, Token, UnaryOp};

use Delimiter::*;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors_pos: vec![],
        }
    }

    fn peek_next_token(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    fn next_token(&mut self) -> Option<Token> {
        let token = self.peek_next_token()?;
        self.pos += 1;
        Some(token)
    }

    // -- region : Grammar rules --

    // expr ::=
    //     LITERAL
    //     | NIL
    //     | IDENT
    //     | '(' paren_expr ')'
    pub fn parse_expr(&mut self) -> AstNode {
        let node = match self
            .peek_next_token()
            .unwrap_or_else(|| panic!("unexpected EOF"))
        {
            // LITERAL ::= NUMBER | STRING | BOOL | CHAR
            Token::Literal(_) => self.parse_literal(), // number, string, bool, char

            // IDENT ::= [a-zA-Z_][a-zA-Z0-9_]*
            Token::Ident(ident) => {
                self.next_token();
                AstNode::Ident(ident)
            }

            // paren_expr ::=
            //     expr*
            //     | binary_op expr expr
            //     | unary_op expr
            //     | func_call
            //     | RESERVED expr
            Token::Delimiter(LParen) => {
                self.parse_paren_expr()
                // check if
            }

            // We skip space and newline (TODO: should we?)
            Token::Delimiter(Space | NewLine) => {
                self.next_token();
                self.parse_expr()
            }

            tok => AstNode::TokenError(tok),
        };
        node
    }

    fn parse_literal(&mut self) -> AstNode {
        use Literal::*;

        match self.next_token().unwrap() {
            // NUMBER ::= [0-9]+
            Token::Literal(NumberLit(num)) => AstNode::Literal(NumberLit(num)),

            // STRING ::= '"' [a-zA-Z0-9]* '"'
            Token::Literal(StringLit(s)) => AstNode::Literal(StringLit(s)),

            // BOOL ::= 'true' | 'false'
            Token::Literal(BoolLit(b)) => AstNode::Literal(BoolLit(b)),

            // CHAR ::= [a-zA-Z]
            Token::Literal(CharLit(c)) => AstNode::Literal(CharLit(c)),

            tok => AstNode::TokenError(tok),
        }
    }

    /// paren_expr ::=
    ///     paren_expr*
    ///     | binary_op expr expr
    ///     | unary_op expr
    ///     | func_call
    ///     | RESERVED expr
    fn parse_paren_expr(&mut self) -> AstNode {
        use BinaryOp::*;
        use UnaryOp::*;

        self.next_token(); // consume '(' token

        match self.next_token().unwrap() {
            Token::Delimiter(LParen) => self.parse_paren_expr(),

            // binary_op expr expr
            Token::BinaryOp(op)
                if op == Add || op == Sub || op == Mul || op == Div =>
            {
                let node = AstNode::BinaryOp {
                    op,
                    lhs: Box::new(self.parse_expr()),
                    rhs: Box::new(self.parse_expr()),
                };

                self.expect_token(node, Token::Delimiter(RParen))
            }

            // unary_op expr
            Token::UnaryOp(op) if op == Neg || op == Not => {
                let node = AstNode::UnaryOp {
                    op,
                    expr: Box::new(self.parse_expr()),
                };

                self.expect_token(node, Token::Delimiter(RParen))
            },

            // func_call ::= IDENT expr
            Token::Ident(ident) => {
                let node = AstNode::FnCall {
                    ident,
                    expr: Box::new(self.parse_expr()),
                };

                self.expect_token(node, Token::Delimiter(RParen))
            },
            
            // TODO: RESERVED expr
            Token::ReservedKw(_) => {
                todo!()
            },

            tok => AstNode::TokenError(tok),
        }
    }

    // -- end region : Grammar rules --

    // -- region : helpers --

    fn expect_token(&mut self, node: AstNode, expected: Token) -> AstNode {
        while let Some(Token::Delimiter(Space | NewLine)) = self.peek_next_token() {
            self.next_token();
        }

        let next = self.next_token().unwrap();
        if next != expected {
            AstNode::ParserError(
                format!(
                    "unexpected token {:?}, expected {:?}",
                    next, expected
                ),
                Box::new(node),
            )
        } else {
            node
        }
    }

    // -- end region : helpers --
}
