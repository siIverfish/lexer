// TODO refactor into struct
// w/ advance method like parser

use crate::error::ScanError;
use crate::token::ValueType;
use crate::builtins::token::*;
use crate::lexeme::Lexeme;
use crate::spec::is_valid_identifier;


pub fn scan(input: &[char]) -> Result<Vec<Lexeme>, ScanError> {
    let mut lexemes: Vec<Lexeme> = Vec::with_capacity(input.len() / 2); // guesswork
    let mut i: usize = 0;

    while let Some(&next_char) = input.get(i) {
        match next_char {
            '+' => Ok(Lexeme::Token(ADDITION      .clone())),
            '-' => Ok(Lexeme::Token(SUBTRACTION   .clone())),
            '*' => Ok(Lexeme::Token(MULTIPLICATION.clone())),
            '/' => Ok(Lexeme::Token(DIVISION      .clone())),
            '%' => Ok(Lexeme::Token(MODULUS       .clone())),
            '(' => Ok(Lexeme::OpenParen                    ),
            ')' => Ok(Lexeme::CloseParen                   ),
            ',' => Ok(Lexeme::Comma,                       ),
            w if w.is_whitespace() => { i += 1; continue; },
            other   => Err(ScanError::NoMatch(other))
        }
        .inspect(|_| { i += 1 })
        // .filter(|lexeme| lexeme != Lexeme::Whitespace)
        .or_else(|err|
            match err {
                ScanError::NoMatch(c) if c.is_numeric() =>
                    input[i..]
                        .iter()
                        .take_while(|&&c| c.is_numeric() || c == '.')
                        .inspect(|_| { i += 1 })
                        .collect::<String>()
                        .parse::<ValueType>()
                        .map(ValueType::into)
                        .map_err(ScanError::from),
                other_err => Err(other_err)
            }
        )
        .or_else(|err|
            match err {
                ScanError::NoMatch(c) if is_valid_identifier(c) =>
                    Ok(input[i..]
                        .iter()
                        .take_while(|&&c| is_valid_identifier(c))
                        .inspect(|_| { i += 1 })
                        .collect::<String>()
                        .into()),
                    other_err => Err(other_err)
            }
        )
        .map(|final_token| lexemes.push(final_token))?
    }

    Ok(lexemes)
}