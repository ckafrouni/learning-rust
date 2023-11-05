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
