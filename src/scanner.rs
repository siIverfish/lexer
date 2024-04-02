// TODO refactor into struct
// w/ advance method like parser

// TODO base parsing class for Scanner & parser

use crate::error::ScanError;
use crate::token::{Token, Token::*, Data::*, ValueType};
use crate::builtins::ops::*;
use crate::lexeme::Keyword::*;
use crate::lexeme::{Lexeme, Lexeme::*};
use crate::spec::is_valid_identifier;

#[derive(Debug)]
pub struct Scanner<'input> {
    index: usize,
    input: &'input [char],
}

impl<'input> Iterator for Scanner<'input> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.peek().inspect(|_| self.advance())
    }
}

impl<'input> Scanner<'input> {
    pub fn with_input(input: &'input [char]) -> Self {
        Self { index: 0, input }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.input.get(self.index).copied()
    }

    pub fn is_finished(&self) -> bool {
        self.index >= self.input.len()
    }

    #[inline(always)]
    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn consume(&mut self, consumed: char) -> Option<()> {
        match self.next() {
            Some(c) if c == consumed => Some(()),
            _ => None
        }
    }

    pub fn scan_input(&mut self) -> Result<Vec<Lexeme>, ScanError> {
        let mut lexemes: Vec<Lexeme> = Vec::with_capacity(self.input.len() / 2); // guesswork

        while !self.is_finished() {
            match &self.input[self.index..] {
                &['*', '*', ..] => Ok((2, Token(POW.clone()))),
                &['+', ..]      => Ok((1, Token(ADDITION.clone()))),
                &['-', ..]      => Ok((1, Token(SUBTRACTION.clone()))),
                &['*', ..]      => Ok((1, Token(MULTIPLICATION.clone()))),
                &['/', ..]      => Ok((1, Token(DIVISION.clone()))),
                &['%', ..]      => Ok((1, Token(MODULUS.clone()))),
                &[';', ..]      => Ok((1, Token(STATEMENT.clone()))),
                &['(', ..]      => Ok((1, OpenParen)),
                &[')', ..]      => Ok((1, CloseParen)),
                &[',', ..]      => Ok((1, Comma)),
                &['=', ..]      => Ok((1, Keyword(ToEqual))),
                &['#', ..]      => {
                    self.index += &self.input[self.index..].iter().take_while(|&&c| c != '\n').count();
                    continue;
                }
                &[w, ..] if w.is_whitespace() => {
                    self.index += 1;
                    continue;
                }
                &[other, ..] => Err(ScanError::NoMatch(other)),
                &[..] => panic!("no match! {self:?}\n\n{:#?}", &self.input[self.index..]),
            }
            .map(|(j, token)| {self.index += j; token} )
            // strings
            .or_else(|err|
                match err {
                    ScanError::NoMatch('"') => {
                        self.consume('"').unwrap();
                        let string = self.into_iter().take_while(|&c| c != '"').collect();
                        Ok(Token(Value(Str(string))))
                    },
                    other => Err(other)
                }
            )
            // numbers
            .or_else(|err|
                match err {
                    ScanError::NoMatch(c) if c.is_numeric() =>
                        self.input[self.index..]
                            .iter()
                            .take_while(|&&c| c.is_numeric() || c == '.')
                            .inspect(|_| { self.index += 1 })
                            .collect::<String>()
                            .parse::<ValueType>()
                            .map(ValueType::into)
                            .map_err(ScanError::from),
                    other_err => Err(other_err)
                }
            )
            // ident
            .or_else(|err|
                match err {
                    ScanError::NoMatch(c) if is_valid_identifier(c) =>
                        Ok(self.input[self.index..]
                            .iter()
                            .take_while(|&&c| is_valid_identifier(c))
                            .inspect(|_| { self.index += 1 })
                            .collect::<String>()
                            .into()),
                    other_err => Err(other_err)
                }
            )
            // recognize keywords
            .map(|lexeme|
                if let Lexeme::Token(Token::Ident(ref string)) = lexeme {
                    if let Some(keyword) = Lexeme::keyword(string) {
                        keyword
                    } else { lexeme }
                } else { lexeme }
            )
            .map(|final_token| lexemes.push(final_token))?
        }

        // turn identifiers into keywords

        Ok(lexemes)
    }
}