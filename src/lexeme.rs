use crate::token::{ValueType, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    DefineVar,
    If,
    Else,
    While,
    Loop,
    Print,
    ToEqual,
    Lambda,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    Token(Token),
    Keyword(Keyword),
    OpenParen,
    Comma,
    CloseParen,
    OpenScope,
    CloseScope,
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

impl From<Keyword> for Lexeme {
    fn from(value: Keyword) -> Self { Lexeme::Keyword(value) }
}

impl Lexeme {
    pub(crate) fn keyword(value: &str) -> Option<Self> {
        use Keyword::*;

        Some(match value {
            "var" => DefineVar,
            "if" => If,
            "else" => Else,
            "while" => While,
            "loop" => Loop,
            "print" => Print,
            "lambda" => Lambda,
            _ => None?,
        }).map(Lexeme::from)
    }
}