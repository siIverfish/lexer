// use crate::token::ValueType;
use crate::token::Token;
use crate::eval::EvalResult;

type FunctionType = dyn Sync + Fn(Vec<Token>) -> EvalResult;

pub struct LFunction(
    pub &'static FunctionType
);

impl PartialEq for LFunction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl std::fmt::Debug for LFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LFunction( <?> )")
    }
}

impl Clone for LFunction {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl Copy for LFunction {}

impl Deref for LFunction {
    type Target = FunctionType;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

use std::ops::*;

// macro_rules! define_op {
//     ($trait:ident, $fname:ident) => {
//         impl $trait for Token {
//             type Output = EvalResult;

//             fn $fname(self, rhs: Self) -> Self::Output {
//                 match (self, rhs) {
//                     (Token::Number(a), Token::Number(b)) => Ok(Token::Number(a.$fname(b))),
//                     _ => Err(ValueError)
//                 }
//             }
//         }
//     }
// }

pub(crate) mod builtins {}