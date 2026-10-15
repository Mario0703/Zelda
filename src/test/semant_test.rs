// src/test/semant_test.rs

use crate::ast::{Integer, TypedInteger};
use crate::semant::semantic_analyser;

#[test]
fn given_an_integer_the_typed_ast_is_generated() {
    let int_ast = Integer::Int(1);
    let typed_ast = semantic_analyser(int_ast);

    assert_eq!(typed_ast, TypedInteger::TypedInt(1));
}