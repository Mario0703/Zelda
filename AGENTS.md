# AGENTS.md

Zelda: a compiler for a new programming language, written in Rust. Early stage — mostly scaffolding. Intended pipeline (README.md): `logos` lexer → LALRPOP parser → custom AST → Typed AST semantic pass → `inkwell` codegen → `clang` links LLVM IR to an executable.

## Build & verify

- Plain `cargo` only — no scripts, CI, or test suite yet (`cargo check` / `cargo build` / `cargo test`; 0 tests at the moment).
- **LLVM 23 is required for the build.** `inkwell` pulls `llvm-sys 231`, which needs `llvm-config-23` and libpolly 23 static libs. Ubuntu 22.04's own repos only ship LLVM 14 — the build fails without apt.llvm.org packages. The devcontainer (`.devcontainer/Dockerfile`) preinstalls LLVM 23 + `libpolly-23-dev`; elsewhere, install the same or point `LLVM_SYS_231_PREFIX` at an LLVM 23 prefix (must contain `bin/llvm-config-23`).
- **Do not swap `inkwell` back to the crates.io release even with `features = ["llvm23-1"]`** — that release predates LLVM 23 support. The git pin with `rev = ...` in Cargo.toml is deliberate (see its comment); revert it only once a registry release ships llvm23 support.

## State & layout

- Parser is not implemented yet: `lalrpop` is already a build-dependency, but there is no `build.rs` and no `.lalrpop` grammar. When adding the parser, add `build.rs` calling `lalrpop::process_root()` plus the grammar file.
- `src/main.rs` is a hello-world stub; `src/ast.rs`, `src/typed_ast.rs`, `src/hello.rs` are empty placeholders for the pipeline.