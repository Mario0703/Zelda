// src/test/mod.rs

// The `test` module in main.rs is declared with #[cfg(test)],
// so everything in this folder is compiled for `cargo test` only.
mod codegen_test;
mod lexer_test;
mod parser_test;
mod semant_test;