// src/test/lexer_test.rs

use crate::lexer::lexer_impl::Lexer;
use crate::lexer::tokens::Token;

#[test]
fn given_an_integer_token_int_is_generated() {
    // Read the file contents — the path itself would be what gets lexed,
    // and `/` matches no token pattern.
    let zelda_source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/test/zeldaInt.zd"
    ))
    .unwrap();

    let lexer = Lexer::new(&zelda_source);
    let tokens: Vec<Token> = lexer.map(|token| token.unwrap().1).collect();

    assert_eq!(tokens, vec![Token::Integer(1)]);
}