use crate::lexer::tokens::TOKENS;
use logos::Logos;
use std::fs;

// Runs the logos lexer over the file's content and returns the tokens.
// Input that matches no pattern is skipped (`Ok(TOKENS)` / `Err(())` items);
// wire error reporting on `Err` later if needed.
pub fn LexerImplementations(file: String) -> Vec<TOKENS> {
    let source = fs::read_to_string(&file).expect("could not open file");

    TOKENS::lexer(&source)
        .filter_map(|result| result.ok())
        .collect()
}