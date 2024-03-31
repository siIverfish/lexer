#![feature(iterator_try_reduce)]
#![feature(iterator_try_collect)]


pub(crate) mod spec {
    pub fn is_valid_identifier(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '!' || c == '?' || c == '~'
    }

    // static is_valid_identifier: &(dyn Sync + Fn(char) -> bool) = 
    //     &|c| c.is_ascii_alphanumeric() || c == '!' || c == '?' || c == '~';
}

pub mod func;
pub mod token;
pub mod scanner;
pub mod error;
pub mod input;
pub mod parser;
pub mod lexeme;
pub mod eval;
pub mod builtins;