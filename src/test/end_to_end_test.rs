// src/test/end_to_end_test.rs

use crate::codegen::generate_code;
use crate::lexer::lexer_impl::Lexer;
use crate::parser::ProgramParser;
use crate::semant::semantic_analyser;
use crate::test::codegen_test::generate_code_execution;

// the whole compiler in one function: source -> tokens -> AST -> typed AST -> IR -> executed
fn compiler(source: &str) -> i64 {
    let lexer = Lexer::new(source);
    let parser = ProgramParser::new();
    let integer = parser.parse(lexer).expect("parse error");
    let typed_ast = semantic_analyser(integer);
    let ir = generate_code(typed_ast);

    generate_code_execution(&ir)
}

#[test]
fn test_compiler() {
    let result = compiler("2");
    assert_eq!(result, 2);
}