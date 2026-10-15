use logos::Logos;
#[derive(Logos, Debug, Eq, PartialEq)]
pub enum TOKENS {
    #[regex("[1-9][0-9]*", |lex| lex.slice().parse::<i64>().unwrap())]
    INTEGER(i64),
}