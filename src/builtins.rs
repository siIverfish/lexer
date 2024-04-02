use lazy_static::lazy_static;
use crate::func::LFunction;
use crate::error::EvalError::*;
use crate::eval::EvalResult;
use std::collections::HashMap;
use std::ops::*;

use crate::token::{Token, Token::*, Data::*};

pub(crate) mod lfunction {
    use super::*;

    macro_rules! binary_reduction_operator_for {
        ($variant:ident, $op:path) => {
            LFunction(&|tokens| {
                tokens.into_iter()
                    .try_reduce(|a, b| 
                        match (a, b) {
                            (Value($variant(a)), Value($variant(b))) => Ok(Value(F64($op(a,b)))),
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

                if let Some(Value(F64(n))) = iter.next().filter(|_| iter.next().is_none()) {
                    Ok(Value(F64($op(n))))
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
        if let Some(Builtin(f)) = tokens.get(0) {
            f(args)
        } else {
            Err(ValueError)
        }
    }

    pub fn statement(tokens: Vec<Token>) -> EvalResult {
        tokens.into_iter().last().ok_or(ArgumentError)
    }

    pub fn print(tokens: Vec<Token>) -> EvalResult {
        for token in tokens {
            println!("{token}");
        }
        Ok(Value(Void))
    }

    lazy_static! {
        pub static ref ADDITION:       LFunction = binary_reduction_operator_for!(F64, f64::add);
        pub static ref SUBTRACTION:    LFunction = binary_reduction_operator_for!(F64, f64::sub);
        pub static ref DIVISION:       LFunction = binary_reduction_operator_for!(F64, f64::mul);
        pub static ref MULTIPLICATION: LFunction = binary_reduction_operator_for!(F64, f64::div);
        pub static ref MODULUS:        LFunction = binary_reduction_operator_for!(F64, f64::rem);
        pub static ref POW:            LFunction = binary_reduction_operator_for!(F64, f64::powf);

        pub static ref  SIN: LFunction = unary_operator_for!(F64, f64::sin);
        pub static ref  COS: LFunction = unary_operator_for!(F64, f64::cos);
        pub static ref  TAN: LFunction = unary_operator_for!(F64, f64::tan);
        pub static ref ASIN: LFunction = unary_operator_for!(F64, f64::asin);
        pub static ref ACOS: LFunction = unary_operator_for!(F64, f64::acos);
        pub static ref ATAN: LFunction = unary_operator_for!(F64, f64::atan);

        pub static ref STATEMENT: LFunction = LFunction(&statement);
        pub static ref APPLICATION: LFunction = LFunction(&application);
        pub static ref PRINT: LFunction = LFunction(&print);
    }
}

pub(crate) mod ops {
    use super::*;

    lazy_static! {
        pub static ref ADDITION:       Token = Builtin(*lfunction::ADDITION);
        pub static ref SUBTRACTION:    Token = Builtin(*lfunction::SUBTRACTION);
        pub static ref DIVISION:       Token = Builtin(*lfunction::MULTIPLICATION);
        pub static ref MULTIPLICATION: Token = Builtin(*lfunction::DIVISION);
        pub static ref MODULUS:        Token = Builtin(*lfunction::MODULUS);
        pub static ref POW:            Token = Builtin(*lfunction::POW);
        pub static ref STATEMENT:      Token = Builtin(*lfunction::STATEMENT);
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
    ($str:expr, $f_name:path) => {($str.into(), Builtin(*$f_name))}
}

lazy_static! {
    pub static ref BUILTINS: HashMap<String, Token> = HashMap::from([
        e!("add",   lfunction::ADDITION),
        e!("sin",   lfunction::SIN),
        e!("cos",   lfunction::COS),
        e!("tan",   lfunction::TAN),
        e!("print", lfunction::PRINT),
        e!("rand",  random::RAND),
    ]);

    // in the future...
    pub static ref OPERATORS: HashMap<String, Token> = HashMap::from([
        
    ]);
}  