
use crate::token::{ValueType, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    Token(Token),
    OpenParen,
    Comma,
    CloseParen,
}

impl From<Token> for Lexeme {
    fn from(value: Token) -> Self {
        Lexeme::Token(value)
    }
}

impl From<ValueType> for Lexeme {
    fn from(value: ValueType) -> Self {
        Lexeme::Token(value.into())
    } 
}

impl From<String> for Lexeme {
    fn from(value: String) -> Self {
        Lexeme::Token(value.into())
    }
}