// src/test/lexer_test.rs

use crate::lexer::lexer_impl::LexerImplementations;
use crate::lexer::tokens::TOKENS;

#[test]
fn given_an_integer_token_int_is_generated() {
    // cargo runs tests from the package root, and fs paths in
    // LexerImplementations resolve from the current directory —
    // anchor this file to the crate root so the test is location-independent.
    let zelda_source = concat!(env!("CARGO_MANIFEST_DIR"), "/src/test/zeldaInt.zd");

    let tokens = LexerImplementations(zelda_source.to_string());

    assert_eq!(tokens, vec![TOKENS::INTEGER(1)]);
}