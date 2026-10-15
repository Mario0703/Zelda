// src/ast/ast.rs

// The untyped AST produced by the parser.
#[derive(Debug, PartialEq, Clone)]
pub enum Integer {
    Int(i64),
}