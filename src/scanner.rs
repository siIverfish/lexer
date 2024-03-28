
use crate::error::ScanError;
use crate::token::{NumberType, Token};
use crate::func::builtins::token::*;
// use crate::input::Input;
// use crate::error::ScanError;

#[derive(Debug)]
pub enum Lexeme {
    Token(Token),
    OpenParen,
    CloseParen,
}

impl Into<Lexeme> for Token      {fn into(self) -> Lexeme { Lexeme::Token(self       )}}
impl Into<Lexeme> for NumberType {fn into(self) -> Lexeme { Lexeme::Token(self.into())}}

pub fn scan(input: &[char]) -> Result<Vec<Lexeme>, Box<dyn std::error::Error>> {
    let mut i: usize = 0;
    let mut tokens: Vec<Lexeme> = Vec::with_capacity(input.len() / 2);

    while let Some(&next_char) = input.get(i) {
        tokens.push(match next_char {
            '+' => Ok(Lexeme::Token(*ADDITION      )),
            '-' => Ok(Lexeme::Token(*SUBTRACTION   )),
            '*' => Ok(Lexeme::Token(*MULTIPLICATION)),
            '/' => Ok(Lexeme::Token(*DIVISION      )),
            '%' => Ok(Lexeme::Token(*MODULUS       )),
            '(' => Ok(Lexeme::OpenParen             ),
            ')' => Ok(Lexeme::CloseParen            ),
            w if w.is_whitespace() => { i += 1; continue},
            other   => Err(ScanError::NoMatch(other))
        }
        .map(|lexeme| { i += 1; lexeme })
        .or_else(|err| {
            match err {
                ScanError::NoMatch(c) if c.is_numeric() => {
                    input[i..].iter()
                    .take_while(|&&c| c.is_numeric() || c == '.')
                    .map(|x| { i += 1; x })
                    .collect::<String>()
                    .parse::<NumberType>()
                    .map(NumberType::into)
                    .map_err(ScanError::from)
                },
                other_err => Err(other_err)
            }
        })?);
    }

    Ok(tokens)
}