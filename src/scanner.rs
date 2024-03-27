
use crate::token::{NumberType, Token};
use crate::func::builtins::token::*;
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
        // considered factoring out `i += 1;` but it makes the
        // cool number bit so much cleaner :p

        let next_tk: Lexeme = match next_char {
            '+' => { i += 1; Lexeme::Token(*ADDITION      )},
            '-' => { i += 1; Lexeme::Token(*SUBTRACTION   )},
            '*' => { i += 1; Lexeme::Token(*MULTIPLICATION)},
            '/' => { i += 1; Lexeme::Token(*DIVISION      )},
            '%' => { i += 1; Lexeme::Token(*MODULUS       )},

            '(' => { i += 1; Lexeme::OpenParen  },
            ')' => { i += 1; Lexeme::CloseParen },
            
            w if w.is_whitespace() => { i += 1; continue; },

            n if n.is_numeric() => 
                    input[i..].iter()
                        .take_while(|&&c| c.is_numeric() || c == '.')
                        .map(|x| { i += 1; x })
                        .collect::<String>()
                        .parse::<NumberType>()?
                        .into(),
            
            
            other => Err(format!("unparsable char '{other}'"))?
        };

        tokens.push(next_tk);
    }

    Ok(tokens)
}