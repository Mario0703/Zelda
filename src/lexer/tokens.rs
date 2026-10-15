use logos::Logos;

// Logos derives the tokenizer directly from this enum: each variant
// becomes a token type, so the #[regex] attributes must live on it
// (a separate `regexExpressions` enum can't reference these variants).
#[derive(Logos, Debug, Eq, PartialEq)]
pub enum TOKENS {
    #[regex("[1-9][0-9]*", |lex| lex.slice().parse::<i64>().unwrap())]
    INTEGER(i64),
}