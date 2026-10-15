mod ast;
mod lexer;
mod parser;
mod semant;
mod codegen;
#[cfg(test)]
mod test;

use crate::codegen::generate_code;
use crate::parser::parse_file;
use crate::semant::semantic_analyser;

// for now the compiler compiles exactly one integer literal per .zd file
fn main() {
    let file = std::env::args()
        .nth(1)
        .expect("usage: Zelda <path-to-.zd-file>");

    // lexer -> parser -> semantic analyser -> codegen
    let integer = parse_file(file).expect("parse error");
    let typed_ast = semantic_analyser(integer);
    let ir = generate_code(typed_ast);

    // the final linking step consumes this: clang-23 program.ll -o program
    std::fs::write("program.ll", ir).expect("could not write program.ll");
    println!("wrote program.ll — link it with: clang-23 program.ll -o program");
}