// src/test/parser_test.rs

use crate::ast::Integer;
use crate::lexer::lexer_impl::Lexer;
use crate::parser::{parse_file, ProgramParser};

#[test]
fn given_an_integer_a_program_is_parsed() {
    // no file needed here — the parser is exercised directly on a string
    let parser = ProgramParser::new();
    let program = parser.parse(Lexer::new("2")).unwrap();

    assert_eq!(program, Integer::Int(2));
}

#[test]
fn a_zelda_file_is_parsed_into_a_program() {
    // this test's own Zelda program file (single integer)
    let zelda_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/test/CodeFiles/parserInt.zd");

    // parse_file takes a path and reads the contents itself
    let program = parse_file(zelda_path.to_string()).unwrap();

    assert_eq!(program, Integer::Int(7));
}