mod tokens;
use std::fs;
use logos::Logos;
mod tokens;
use crate::Tokens::


mod lexer_impl{
    enum regexExpressions {
        #[regex("^[1-9][0-9]?", |lex| lex.slice().parse())]
        TOKENS::INTEGER(i64)
}

pub fn LexerImplementations(file: String){

}

