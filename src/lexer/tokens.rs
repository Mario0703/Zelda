use logos::Logos;
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParsingError,
    #[default]
    Other,
}

impl From<ParseIntError> for LexingError {
    fn from(_: ParseIntError) -> Self {
        LexingError::NumberParsingError
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
pub enum Token {
    #[regex(r"[ \n\t\f]+", logos::skip)]
    Ignored,

    #[regex("[1-9][0-9]*", |lex| lex.slice().parse())]
    Integer(i64),
}