
use lazy_static::lazy_static;
use crate::token::Token;
use crate::func::LFunction;
use crate::error::EvalError::*;
use crate::eval::EvalResult;
use std::collections::HashMap;
use std::ops::*;

pub(crate) mod lfunction {
    use super::*;

    macro_rules! binary_reduction_operator_for {
        ($variant:ident, $op:path) => {
            LFunction(&|tokens| {
                tokens.into_iter()
                    .try_reduce(|a, b| 
                        match (a, b) {
                            (Token::Number(a), Token::Number(b)) => Ok(Token::Number($op(a,b))),
                            _ => Err(ValueError)
                        }
                    )
                    .and_then(|option| option.ok_or(ArgumentError))
            })
        }
    }

    macro_rules! unary_operator_for {
        ($variant:ident, $op:path) => {
            LFunction(&|tokens| {
                let mut iter = tokens.into_iter();

                if let Some(Token::Number(n)) = iter.next().filter(|_| iter.next().is_none()) {
                    Ok(Token::Number($op(n)))
                } else {
                    Err(ArgumentError)
                }
            })
        }
    }

    pub fn application(mut tokens: Vec<Token>) -> EvalResult {
        if tokens.len() == 0 {
            return Err(ArgumentError);
        }

        let args = tokens.split_off(1);
        if let Some(Token::Function(f)) = tokens.get(0) {
            f(args)
        } else {
            Err(ValueError)
        }
    }

    lazy_static! {
        pub static ref ADDITION:       LFunction = binary_reduction_operator_for!(Number, f64::add);
        pub static ref SUBTRACTION:    LFunction = binary_reduction_operator_for!(Number, f64::sub);
        pub static ref DIVISION:       LFunction = binary_reduction_operator_for!(Number, f64::mul);
        pub static ref MULTIPLICATION: LFunction = binary_reduction_operator_for!(Number, f64::div);
        pub static ref MODULUS:        LFunction = binary_reduction_operator_for!(Number, f64::rem);

        pub static ref  SIN: LFunction = unary_operator_for!(Number, f64::sin);
        pub static ref  COS: LFunction = unary_operator_for!(Number, f64::cos);
        pub static ref  TAN: LFunction = unary_operator_for!(Number, f64::tan);
        pub static ref ASIN: LFunction = unary_operator_for!(Number, f64::asin);
        pub static ref ACOS: LFunction = unary_operator_for!(Number, f64::acos);
        pub static ref ATAN: LFunction = unary_operator_for!(Number, f64::atan);

        pub static ref APPLICATION: LFunction = LFunction(&application);
    }      
}      

pub(crate) mod token { 
    use super::*;

    lazy_static! {
        pub static ref ADDITION:       Token = Token::Function(&lfunction::ADDITION);
        pub static ref SUBTRACTION:    Token = Token::Function(&lfunction::SUBTRACTION);
        pub static ref DIVISION:       Token = Token::Function(&lfunction::MULTIPLICATION);
        pub static ref MULTIPLICATION: Token = Token::Function(&lfunction::DIVISION);
        pub static ref MODULUS:        Token = Token::Function(&lfunction::MODULUS);
    }
}

mod random {
    use std::sync::Mutex;

    use lazy_static::lazy_static;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};

    use crate::func::LFunction;
    use crate::token::Token;
    use crate::eval::EvalResult;

    pub fn rand_float(_: Vec<Token>) -> EvalResult {
        Ok(BUILTIN_RNG.lock().expect("random source is not poisoned").gen::<f64>().into())
    }

    lazy_static! {
        pub static ref BUILTIN_RNG: Mutex<SmallRng> = Mutex::new(SmallRng::from_entropy());
        
        pub static ref RAND: LFunction = LFunction(&rand_float);
    }
}

macro_rules! e {
    ($str:expr, $f_name:path) => {($str.into(), Token::Function(&*$f_name))}
}

lazy_static! {
    pub static ref BUILTINS: HashMap<String, Token> = HashMap::from([
        e!("add",  lfunction::ADDITION),
        e!("sin",  lfunction::SIN),
        e!("cos",  lfunction::COS),
        e!("tan",  lfunction::TAN),
        e!("rand", random::RAND)
    ]);
}  