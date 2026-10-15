// src/semant/semantic_analyser.rs

use crate::ast::{Integer, TypedInteger};

// The semantic pass: untyped AST -> typed AST.
pub fn semantic_analyser(ast: Integer) -> TypedInteger {
    match ast {
        Integer::Int(t) => TypedInteger::TypedInt(t),
    }
}