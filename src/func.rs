
use crate::token::NumberType;

#[derive(Clone)]
pub struct LFunction(
    &'static (dyn Sync + Fn(&[NumberType]) -> NumberType)
);

impl PartialEq for LFunction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl std::fmt::Debug for LFunction  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LFunction( <?> )")
    }
}

impl Copy for LFunction {}

// impl TryFrom<char> for &LFunction {
//     type Error = ();

//     fn try_from(value: char) -> Result<Self, Self::Error> {
//         use builtins::lfunction::*;

//         match value {
//             '+' => Ok(&*ADDITION),
//             '-' => Ok(&*SUBTRACTION),
//             '*' => Ok(&*MULTIPLICATION),
//             '/' => Ok(&*DIVISION),
//             '%' => Ok(&*MODULUS),
//              _  => Err(())?
//         }
//     }
// }

pub mod builtins {
    use lazy_static::lazy_static;
    use crate::token::Token;
    use super::LFunction;

    pub mod lfunction {
        use super::*;
        lazy_static! {
            pub static ref ADDITION:       LFunction = LFunction(&|tokens| tokens[0] + tokens[1]);
            pub static ref SUBTRACTION:    LFunction = LFunction(&|tokens| tokens[0] - tokens[1]);
            pub static ref DIVISION:       LFunction = LFunction(&|tokens| tokens[0] / tokens[1]);
            pub static ref MULTIPLICATION: LFunction = LFunction(&|tokens| tokens[0] * tokens[1]);
            pub static ref MODULUS:        LFunction = LFunction(&|tokens| tokens[0] % tokens[1]);
        }      
    }      

    pub mod token { 
        use super::*;
        lazy_static! {
            pub static ref ADDITION:       Token = Token::Function(&lfunction::ADDITION);
            pub static ref SUBTRACTION:    Token = Token::Function(&lfunction::SUBTRACTION);
            pub static ref DIVISION:       Token = Token::Function(&lfunction::MULTIPLICATION);
            pub static ref MULTIPLICATION: Token = Token::Function(&lfunction::DIVISION);
            pub static ref MODULUS:        Token = Token::Function(&lfunction::MODULUS);
        }
    }            
}