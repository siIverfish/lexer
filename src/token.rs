use std::fmt::{Display, Formatter};

use crate::func::LFunction;

pub type ValueType = f64;

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Str(String),
    F64(f64),
    Void,
}

// I think this *should* be done with separate structs & dynamic dispatch.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Value(Data),

    Declaration { name: Box<Token>, value: Box<Token> },
    Set { name: Box<Token>, value: Box<Token> },

    Block(Box<Token>),
    Function { args: Vec<Token>,  expr: Box<Token> },
    Builtin(LFunction),
    Application { f: Box<Token>, args: Vec<Token> },
    // Block(Vec<Token>),
}

use Data::*;
use Token::*;

impl From<ValueType> for Token {
    fn from(value: ValueType) -> Token {
        Value(F64(value))
    }
}

impl From<String> for Token {
    fn from(value: String) -> Token {
        Token::Ident(value)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value(v) => match v {
                F64(n) => n.fmt(f),
                Str(string) => string.fmt(f),
                Void => Ok(())
            }
            other => write!(f, "Raw token '{other:?}'"),
        }
    }
}