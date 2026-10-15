// src/ast/mod.rs

// Flat re-exports so the rest of the crate can `use crate::ast::Integer;`
pub mod ast;
pub mod typed_ast;

pub use ast::*;
pub use typed_ast::*;