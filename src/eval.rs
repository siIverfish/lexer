use std::cell::RefCell;
use std::collections::HashMap;

use crate::token::{Token, Token::*, Data::*};
use crate::error::EvalError;

use crate::error::EvalError::*;

pub type EvalResult = Result<Token, EvalError>;

pub struct Scope<'parent> {
    vars: HashMap<String, Token>,
    parent: Option<&'parent RefCell<Scope<'parent>>>,
}

impl<'parent> Scope<'parent> {
    pub fn create_global_scope() -> Self {
        // duplication of the BUILTINS map --
        // could this be avoided?
        Scope {
            vars: crate::builtins::BUILTINS.clone(),
            parent: None,
        }
    }

    pub fn with_parent(parent: &'parent RefCell<Scope<'parent>>) -> Self {
        let vars = HashMap::new();
        let parent = Some(parent);

        Scope { vars, parent }
    }

    pub fn get(&mut self, name: &str) -> Option<Token> {
        self.vars.get(name)
            .map(Token::clone)
            .or_else(|| self.parent.and_then(
                |parent_cell| (&mut parent_cell.borrow_mut()).get(name)
            )
            )
    }

    pub fn create(&mut self, name: Token, value: Token) -> Result<(), EvalError> {
        if let Ident(name) = name {
            if self.vars.contains_key(&name) {
                Err(RedefinedVariable(name))
            } else {
                self.vars.insert(name, value);
                Ok(())
            }
        } else {
            Err(InvalidAssignment(name))
        }
    }

    pub fn set_ident(&mut self, name: String, value: Token) -> Result<(), EvalError> {
        if self.vars.contains_key(&name) {
            self.vars.insert(name, value);
            Ok(())
        } else {
            // delegate to parent scope if we don't own the value
            let mut parent_cell = self.parent
                .ok_or(UndefinedVariable(name.clone()))?
                .borrow_mut();
            (&mut parent_cell).set_ident(name, value)
        }
    }

    pub fn set(&mut self, token: Token, value: Token) -> Result<(), EvalError> {
        // if this scope owns the key, set it here
        if let Ident(name) = token {
            self.set_ident(name, value)
        } else {
            Err(InvalidAssignment(token))
        }
    }

    pub fn eval(&mut self, token: Token) -> EvalResult {
        match token {
            Application { f, args } =>
                args.into_iter()
                    .map(|t| self.eval(t))
                    .try_collect()
                    .and_then(|args| {
                        match self.eval(*f)? {
                            Token::Function(function) => function(args),
                            other => Err(NotAFunction(other))
                        }
                    }),
            Ident(name) => self.get(&name).ok_or(UndefinedVariable(name.clone())),
            Declaration { name, value } => {
                let value = self.eval(*value)?;
                self.create(*name, value).map(|()| Value(Void))
            }
            Set { name, value } => {
                let value = self.eval(*value)?;
                self.set(*name, value).map(|()| Value(Void))
            }
            other => Ok(other), // bad
        }
    }
}