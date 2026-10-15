// src/test/codegen_test.rs

use crate::ast::TypedInteger;
use crate::codegen::generate_code;
use crate::parser::parse_file;
use crate::semant::semantic_analyser;

#[test]
fn given_a_typed_integer_the_ir_defines_main_returning_it() {
    let typed_ast = TypedInteger::TypedInt(1);

    let ir = generate_code(typed_ast);

    assert!(
        ir.contains("define i64 @main"),
        "generated IR should define @main() -> i64:\n{ir}"
    );
    assert!(
        ir.contains("ret i64 1"),
        "generated IR should return the typed AST's value:\n{ir}"
    );
}

#[test]
fn generated_ir_carries_the_ast_value() {
    // the value flows through the pipeline: TypedInt(42) -> "ret i64 42"
    let ir = generate_code(TypedInteger::TypedInt(42));

    assert!(
        ir.contains("ret i64 42"),
        "the typed AST's value should appear in the IR:\n{ir}"
    );
}

#[test]
fn the_whole_pipeline_emits_ir_from_a_zelda_file() {
    let zelda_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/test/zeldaInt.zd");

    // lexer -> parser -> semantic analyser -> codegen
    let program = parse_file(zelda_path.to_string()).unwrap();
    let integer = program.into_iter().next().unwrap(); // take the only node
    let typed_ast = semantic_analyser(integer);
    let ir = generate_code(typed_ast);

    assert!(
        ir.contains("ret i64 1"),
        "the .zd file's content should survive every stage and land in the IR:\n{ir}"
    );
}