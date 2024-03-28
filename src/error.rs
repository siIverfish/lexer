
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("no match for token {0}")]
    NoMatch(char),
    #[error("could not parse number: unexpected char {0}")]
    ParseNumberError(#[from] std::num::ParseFloatError),
}