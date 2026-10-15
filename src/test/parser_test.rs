// src/test/parser_test.rs

use crate::ast::Integer;
use crate::lexer::lexer_impl::Lexer;
use crate::parser::{parse_file, ProgramParser};

#[test]
fn given_an_integer_a_program_is_parsed() {
    let parser = ProgramParser::new();
    let program = parser.parse(Lexer::new("2")).unwrap();

    assert_eq!(program, Integer::Int(2));
}

#[test]
fn a_single_integer_file_is_a_program() {
    // parse_file takes a path and reads the contents itself
    let zelda_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/test/zeldaInt.zd");

    let program = parse_file(zelda_path.to_string()).unwrap();

    assert_eq!(program, Integer::Int(1));
}