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
