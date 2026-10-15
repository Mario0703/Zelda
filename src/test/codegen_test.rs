// src/test/codegen_test.rs

use crate::ast::TypedInteger;
use crate::codegen::generate_code;
use inkwell::context::Context;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::OptimizationLevel;

// The shape of the generated `fn main() -> i64`. The ABI must be "C"
// (inkwell's JitFunction only supports extern "C") — that's what the
// `E0703: invalid ABI` was complaining about.
type CFunction = unsafe extern "C" fn() -> i64;

#[test]
fn given_a_typed_integer_the_ir_defines_main_returning_it() {
    let typed_ast = TypedInteger::TypedInt(1);

    let ir = generate_code(typed_ast);
    print!("{ir}"); // shows up with `cargo test -- --nocapture`

    assert!(
        ir.contains("define i64 @main"),
        "generated IR should define @main() -> i64:\n{ir}"
    );
    assert!(
        ir.contains("ret i64 1"),
        "generated IR should return the typed AST's value:\n{ir}"
    );
    let res = generate_code_execution(&ir);
    assert_eq!(res, 1)
}

#[test]
fn generated_ir_carries_the_ast_value() {
    // the value flows through the pipeline: TypedInt(42) -> "ret i64 42"
    let ir = generate_code(TypedInteger::TypedInt(42));

    assert!(
        ir.contains("ret i64 42"),
        "the typed AST's value should appear in the IR:\n{ir}"
    );

    let res = generate_code_execution(&ir);
    assert_eq!(res, 42)
}

pub fn generate_code_execution(ir: &str) -> i64 {
    let context = Context::create();
    let memory_buffer = MemoryBuffer::create_from_memory_range_copy(ir.as_bytes(), "program");
    let module = context.create_module_from_ir(memory_buffer).expect("could not load the IR");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).expect("could not create execution engine");
    let main = unsafe { execution_engine.get_function::<CFunction>("main").expect("main not found") };
    let result = unsafe { main.call() };
    result
}
