
use std::collections::HashMap;

use crate::token::Token;
use crate::func::LFunction;
use crate::error::EvalError;

use crate::error::EvalError::*;

pub type EvalResult = Result<Token, EvalError>;

pub struct Scope<'parent> {
    vars: HashMap<String, Token>,
    parent: Option<&'parent Scope<'parent>>
}

impl<'parent> Scope<'parent> {
    pub fn create_global_scope() -> Self {
        // duplication of the BUILTINS map --
        // could this be avoided?
        Scope {
            vars: crate::builtins::BUILTINS.clone(), 
            parent: None
        }
    }

    pub fn with_parent(parent: &'parent Scope) -> Self {
        let vars = HashMap::new();
        let parent = Some(parent);

        Scope { vars, parent }
    }

    pub fn eval(&mut self, token: Token) -> EvalResult {
        match token {
            Token::Application { f, args } => 
                match self.eval(*f)? {
                    Token::Function(LFunction(function)) => 
                        function(args.into_iter().map(|t| self.eval(t)).try_collect()?),
                    other => Err(NotAFunction(other))
                }
            Token::Ident(name) => self.vars
                .get(&*name)
                .map(|t| t.clone())
                .ok_or(UndefinedVariable(name.to_string())),
            other => Ok(other), // bad
        }
    }
}