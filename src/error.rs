use thiserror::Error;

use crate::lexeme::{Keyword, Lexeme};
use crate::token::Token;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("no match for token {0}")]
    NoMatch(char),
    #[error("could not parse number: unexpected char {0}")]
    ParseNumberError(#[from] std::num::ParseFloatError),
    #[error("no token, only whitespace.")]
    Whitespace,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("input ended unexpectedly")]
    OutOfInputError,
    #[error("unused input")]
    UnusedInput,
    #[error("unexpected closeparen")]
    UnexpectedCloseParen,
    #[error("Unexpected lexeme during parsing {0:?}")]
    UnexpectedLexeme(Lexeme),
    #[error("malformed statement")]
    MalformedStatement(Keyword),
}

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("wrong types for function")]
    ValueError,
    #[error("wrong # args")]
    ArgumentError,
    #[error("token '{0:?}' is not a function")]
    NotAFunction(Token),
    #[error("name '{0}' is not defined in this scope.")]
    UndefinedVariable(String),
    #[error("variable '{0}' is already defined.")]
    RedefinedVariable(String),
    #[error("token '{0}' cannot be assigned to.")]
    InvalidAssignment(Token),
}