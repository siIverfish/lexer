

pub type ValueType = f64;
pub type FuncType = &'static crate::func::LFunction;

// I think this *should* be done with separate structs & dynamic dispatch.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(ValueType),
    Function(FuncType),
    Application { f: Box<Token>, args: Vec<Token> }
}

impl From<ValueType> for Token {
    fn from(value: ValueType) -> Token {
        Token::Number(value)
    }
}

impl From<String> for Token {
    fn from(value: String) -> Token {
        Token::Ident(value)
    }
}