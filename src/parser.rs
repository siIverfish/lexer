use crate::builtins::lfunction::*;
use crate::error::{ParseError, ParseError::*};
use crate::func::LFunction;
use crate::lexeme::{Keyword::*, Lexeme, Lexeme::*};
use crate::token::{Token, Token::*};

type ParseResult = Result<Token, ParseError>;

#[derive(Debug)]
pub struct Parser {
    index: usize,
    lexemes: Vec<Lexeme>,
}

impl Parser {
    pub fn parse_lexemes(lexemes: Vec<Lexeme>) -> ParseResult {
        Parser::new(lexemes).parse()
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
        self.lexemes.get(self.index).map(Lexeme::clone).ok_or(OutOfInputError)
    }

    pub fn consume(&mut self, expected: Lexeme) -> Result<(), ParseError> {
        match self.next()? {
            lexeme if lexeme == expected => Ok(()),
            unexpected => Err(UnexpectedLexeme(unexpected))
        }
    }

    pub fn parse_binary_ops(&mut self, ops: &[&LFunction], next: impl Fn(&mut Self) -> ParseResult) -> ParseResult {
        let mut left: Token = next(self)?;

        loop {
            if let Ok(Token(Builtin(f))) = self.peek() {
                if ops.contains(&&f) {
                    self.advance();
                    let right = next(self)?;
                    let args = vec![left, right];
                    left = Application { f: Box::new(Builtin(f)), args };
                    continue;
                }
            }
            break Ok(left);
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> ParseResult {
        self.parse_program()
    }

    pub fn parse_program(&mut self) -> ParseResult {
        let ret = self.parse_statements();
        match self.next() {
            Err(OutOfInputError) => ret,
            _ => {dbg!(&self.lexemes[self.index..]);Err(UnusedInput)},
        }
    }

    pub fn parse_statements(&mut self) -> ParseResult {
        let left: Token = self.parse_keyword_statement()?;
        let mut args = vec![left];

        loop {
            if let Ok(Lexeme::Token(Token::Builtin(f))) = self.peek() {
                if f == *STATEMENT { 
                    self.advance();
                    if let Ok(right) = self.parse_keyword_statement() {
                        args.push(right);
                        continue;
                    }
                }
            }
            break Ok(Block(Box::new(
                Application { f: Box::new(Builtin(*STATEMENT)), args }
            )));
        }
    }

    pub fn parse_keyword_statement(&mut self) -> ParseResult {
        if let Keyword(keyword) = self.peek()? {
            self.advance();
            match keyword {
                // eventually refactor out
                Print => {
                    let f = Box::new(Token::Builtin(*PRINT));
                    let args = vec![self.parse_expression()?];
                    Ok(Application { f, args })
                }
                DefineVar => {
                    let Token(Ident(name)) = self.next()? else { return Err(MalformedStatement(DefineVar)); };
                    self.consume(Keyword(ToEqual))?;
                    let value = self.parse_expression()?.into();
                    let name = Ident(name).into();
                    Ok(Declaration { name, value })
                }
                _ => unimplemented!(),
            }
        } else {
            let expr = self.parse_expression()?;
            if let Ok(Keyword(ToEqual)) = self.peek() {
                self.advance();
                let name = expr.into();
                let value = self.parse_expression()?.into();
                Ok(Set { name, value })
            } else {
                Ok(expr)
            }
        }
    }

    pub fn parse_expression(&mut self) -> ParseResult {
        self.parse_add_sub()
    }

    pub fn parse_add_sub(&mut self) -> ParseResult {
        self.parse_binary_ops(&[&ADDITION, &SUBTRACTION], Self::parse_mul_div_mod)
    }

    pub fn parse_mul_div_mod(&mut self) -> ParseResult {
        self.parse_binary_ops(&[&MULTIPLICATION, &DIVISION, &MODULUS], Self::parse_pow)
    }

    pub fn parse_pow(&mut self) -> ParseResult {
        self.parse_binary_ops(&[&POW], Self::parse_application)
    }

    pub fn parse_application(&mut self) -> ParseResult {
        let mut f = self.parse_literal()?;

        while let Ok(Lexeme::OpenParen) = self.peek() {
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

        match current_lexeme {
            Lexeme::Token(token) => Ok(token),
            Lexeme::OpenParen => self.parse_group(),
            Lexeme::OpenScope => self.parse_block(),
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

    pub fn parse_block(&mut self) -> ParseResult {
        // Lexeme::OpenScope already consumed.
        let ret = self.parse_statements();
        self.consume(Lexeme::CloseScope)?;
        ret
    }
}