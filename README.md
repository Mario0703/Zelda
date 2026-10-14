## Implementation Details

This programming language is implemented in Rust. The development toolchain consists of the following:

- **Lexer**: `Logos` for high-performance tokenization.
- **Parser**: `LALRPOP` for formal grammar analysis and AST generation.
- **Semantic Analysis**: A custom implementation to transform the raw AST into a Typed AST.
- **CodeGen**: `Inkwell` to generate LLVM Intermediate Representation (IR).
- **Backend**: `Clang` to compile the LLVM IR into an executable binary.
