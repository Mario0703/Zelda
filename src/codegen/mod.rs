// src/codegen/mod.rs

// Flat re-exports so the rest of the crate can `use crate::codegen::generate_code;`
pub mod codegen;

pub use codegen::generate_code;