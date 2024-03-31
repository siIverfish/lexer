
use crate::lexeme::Lexeme;
use crate::token::Token;
use crate::error::{ParseError, ParseError::*};
use crate::builtins::lfunction::*;

type ParseResult = Result<Token, ParseError>;

pub struct Parser {
    index: usize,
    lexemes: Vec<Lexeme>,
}

impl Parser {
    pub fn parse(lexemes: Vec<Lexeme>) -> ParseResult {
        dbg!(Parser::new(lexemes).parse_expression())
    }

    pub fn new(lexemes: Vec<Lexeme>) -> Self {
        Parser { index: 0, lexemes }
    }

    pub fn next(&mut self) -> Result<Lexeme, ParseError> {
        self.peek().inspect(|_| self.advance())
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn peek(&mut self) -> Result<Lexeme, ParseError> {
        self.lexemes.get(self.index)
            .map(Lexeme::clone)
            .ok_or(OutOfInputError)
    }

    pub fn consume(&mut self, expected: Lexeme) -> Result<(), ParseError> {
        match self.next()? {
            lexeme if lexeme == expected => Ok(()),
            unexpected => Err(UnexpectedLexeme(unexpected))
        }
    }

    pub fn parse_expression(&mut self) -> ParseResult {
        self.parse_add_sub()
    }

    pub fn parse_add_sub(&mut self) -> ParseResult {
        let mut left: Token = self.parse_mul_div_mod()?;

        while let Ok(Lexeme::Token(Token::Function(f))) = self.peek() {
            if *f == *ADDITION || *f == *SUBTRACTION {
                self.advance();
                let right = self.parse_mul_div_mod()?;
                let args = vec![left, right];
                left = Token::Application { f: Box::new(Token::Function(f)), args };
                continue;
            }
            break;
        }

        Ok(left)
    }

    pub fn parse_mul_div_mod(&mut self) -> ParseResult {
        let mut left: Token = self.parse_application()?;

        while let Ok(Lexeme::Token(Token::Function(f))) = self.peek() {
            if *f == *MULTIPLICATION || *f == *DIVISION || *f == *MODULUS {
                self.advance();
                let right = self.parse_mul_div_mod()?;
                let args = vec![left, right];
                left = Token::Application { f: Box::new(Token::Function(f)), args };
                continue;
            }
            break;
        }

        Ok(left)
    }

    pub fn parse_application(&mut self) -> ParseResult {
        let mut f = self.parse_literal()?;

        while let Ok(Lexeme::OpenParen) = self.peek() {
            println!("PARSING THE LIST");
            self.advance();
            let args = self.parse_list()?;
            self.consume(Lexeme::CloseParen)?;
            f = Token::Application { f: Box::new(f), args };
        }

        Ok(f)
    }

    pub fn parse_list(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut list_elements: Vec<Token> = Vec::new();

        if self.peek()? == Lexeme::CloseParen { return Ok(list_elements); }

        loop {
            list_elements.push(self.parse_expression()?);
            if let Ok(Lexeme::Comma) = self.peek() {
                self.advance();
                continue;
            } else {
                break Ok(list_elements);
            }
        }
    }

    pub fn parse_literal(&mut self) -> ParseResult {
        let current_lexeme = self.next()?;
        dbg!(&current_lexeme);
    
        match current_lexeme {
            Lexeme::Token(token) => Ok(token),
            Lexeme::OpenParen => self.parse_group(),
            unexpected => Err(UnexpectedLexeme(unexpected))
        }
    }

    pub fn parse_group(&mut self) -> ParseResult {
        // Initial open-paren consumed by `parse_literal` in current implementation
        // self.consume(Lexeme::OpenParen)?;
        let ret = self.parse_expression();
        self.consume(Lexeme::CloseParen)?;
        ret
    }
}