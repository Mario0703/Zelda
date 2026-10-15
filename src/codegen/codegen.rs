// src/codegen/codegen.rs

// The codegen pass: typed AST -> LLVM IR (via inkwell).
use crate::ast::TypedInteger;
use inkwell::context::Context;

pub fn generate_code(typed_ast: TypedInteger) -> String {
    // match on the typed AST — each new node type gets its own arm here
    match typed_ast {
        TypedInteger::TypedInt(value) => {
            let context = Context::create();
            let module = context.create_module("codegen_module");
            let builder = context.create_builder();

            // fn main() -> i64
            let i64_type = context.i64_type();
            let fn_type = i64_type.fn_type(&[], false);
            let function = module.add_function("main", fn_type, None);

            let entry = context.append_basic_block(function, "entry");
            builder.position_at_end(entry);

            let constant = i64_type.const_int(value as u64, false);
            builder.build_return(Some(&constant)).unwrap();

            // raw LLVM IR text (what clang links into an executable)
            module.print_to_string().to_string()
        }
    }
}