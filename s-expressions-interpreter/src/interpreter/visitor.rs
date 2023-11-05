use crate::parser::*;
use crate::tokenizer::*;
use crate::parser::AstVisitor;

use super::values::Value;
use super::AstInterpreter;


impl AstVisitor for AstInterpreter {
    fn visit_literal(&mut self, node: &AstNode) -> Result<(), String> {

        match node {
            AstNode::Literal(literal) => {
                match literal {
                    Literal::NumberLit(n) => {
                        let n = n.parse::<f64>().unwrap();
                        self.stack.push(Value::Number(n));
                    },
                    Literal::StringLit(s) => {
                        self.stack.push(Value::String(s.clone()));
                    },
                    Literal::BoolLit(b) => {
                        let b = b.parse::<bool>().unwrap();
                        self.stack.push(Value::Bool(b));
                    },
                    Literal::CharLit(c) => {
                        let c = c.parse::<char>().unwrap();
                        self.stack.push(Value::Char(c));
                    },
                }
                Ok(())
            },
            _ => Err(format!("Expected a Literal node, found {:?}", node)),
        }
    }

    fn visit_ident(&mut self, node: &AstNode) -> Result<(), String> {
        match node {
            AstNode::Ident(ident) => {
                if let Some(value) = self.heap.get(ident) {
                    self.stack.push(value.clone());
                } else {
                    return Err(format!("Undefined identifier: {}", ident));
                }
                Ok(())
            },
            _ => Err(format!("Expected an Ident node, found {:?}", node)),
        }
    }

    fn visit_binary_op(&mut self, node: &AstNode) -> Result<(), String> {
        if let AstNode::BinaryOp { op, lhs, rhs } = node {
            lhs.accept(self)?;
            rhs.accept(self)?;

            let rhs_val = self.stack.pop().unwrap();
            let lhs_val = self.stack.pop().unwrap();

            match op {
                BinaryOp::Add => {
                    let result = lhs_val.add(&rhs_val)?;
                    self.stack.push(result);
                },
                BinaryOp::Sub => {
                    let result = lhs_val.sub(&rhs_val)?;
                    self.stack.push(result);
                },
                BinaryOp::Mul => {
                    let result = lhs_val.mul(&rhs_val)?;
                    self.stack.push(result);
                },
                BinaryOp::Div => {
                    let result = lhs_val.div(&rhs_val)?;
                    self.stack.push(result);
                },
            }
            Ok(())
        } else {
            Err(format!("Expected a BinaryOp node, found {:?}", node))
        }
    }

    fn visit_unary_op(&mut self, node: &AstNode) -> Result<(), String> {
        if let AstNode::UnaryOp { op, expr } = node {
            expr.accept(self)?;

            let expr_val = self.stack.pop().unwrap();

            match op {
                UnaryOp::Neg => {
                    let result = expr_val.neg()?;
                    self.stack.push(result);
                },
                UnaryOp::Not => {
                    let result = expr_val.not()?;
                    self.stack.push(result);
                },
            }
            Ok(())
        } else {
            Err(format!("Expected a UnaryOp node, found {:?}", node))
        }
    }

    fn visit_fn_call(&mut self, node: &AstNode) -> Result<(), String> {
        if let AstNode::FnCall { ident, expr } = node {
            // TODO: user defined functions
            expr.accept(self)?;

            let expr_val = self.stack.pop().unwrap();

            // TODO: better way to handle built-in functions
            match ident.as_str() {
                "print" => {
                    println!("{:?}", expr_val);
                },
                "println" => {
                    println!("{:?}", expr_val);
                },
                _ => return Err(format!("Unsupported function call: {}", ident)),
            }
            Ok(())
        } else {
            Err(format!("Expected a FnCall node, found {:?}", node))
        }
    }

    fn visit_nil(&mut self, node: &AstNode) -> Result<(), String> {
        if let AstNode::Nil = node {
            self.stack.push(Value::Nil);
            Ok(())
        } else {
            Err(format!("Expected a Nil node, found {:?}", node))
        }
    }

    fn visit_token_error(&mut self, node: &AstNode) -> Result<(), String> {
        // If we are here, it means that the parser has failed to parse the input
        // and has returned a TokenError node. The error comes from the tokenizer.
        // The parser propagates the error to the AstVisitor to handle it.
        // We will just propagate the error to the caller.

        // Propagate the error to the caller
        if let AstNode::TokenError(token) = node {
            Err(format!("TokenError: {:?}", token))
        } else {
            Err(format!("Expected a TokenError node, found {:?}", node))
        }
    }

    fn visit_parser_error(&mut self, _node: &AstNode) -> Result<(), String> {
        if let AstNode::ParserError(msg, node) = _node {
            Err(format!("ParserError: {} {:?}", msg, node))
        } else {
            Err(format!("Expected a ParserError node, found {:?}", _node))
        }
    }
}
