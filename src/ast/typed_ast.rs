// src/ast/typed_ast.rs

// The typed AST produced by the semantic analyser.
#[derive(Debug, PartialEq)]
pub enum TypedInteger {
    TypedInt(i64),
}