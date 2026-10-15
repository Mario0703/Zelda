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
    let welcome = "welcome to the compiler";
    println!("{}",welcome);

}