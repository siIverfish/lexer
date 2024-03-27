

pub type NumberType = f64;

#[derive(Copy, Debug, Clone)]
pub enum Token {
    Number(NumberType),
    Function(&'static crate::func::LFunction)
}

impl Into<Token> for NumberType {
    fn into(self) -> Token {
        Token::Number(self)
    }
}