extern crate llvm_sys;
#[macro_use]extern crate llvm;

use std::mem;

use llvm_sys::core::LLVMGetParam;

fn main() {
    let mut context = llvm::Context::new();
    let mut module = context.create_module_with_name(llvm_str!("my module"));
    let mut builder = context.create_builder();

    let func_name = llvm_str!("add");
    let func = module.add_function(
        llvm::types::Function::new(
            context.i64_type(),
            &[
                context.i64_type(),
                context.i64_type(),
            ],
            false,
        ),
        func_name,
    );
    let bb = context.append_basic_block(func, func_name);
    builder.position_at_end(bb);

    // get the function's arguments
    unsafe {
        let x = LLVMGetParam(func, 0);
        let y = LLVMGetParam(func, 1);

        let s1 = builder.build_add(x, y, llvm_str!("s1"));
        builder.build_ret(s1);
    }
    module.dump();

    llvm::link_in_mcjit();
    llvm::initialize_native_target();
    llvm::initialize_native_asm_printer();

    let ee = llvm::ExecutionEngine::create_for_module(&module).unwrap();
    let addr = ee.get_function_address(func_name).unwrap();

    unsafe {
        let f: extern "C" fn(u64, u64) -> u64 = mem::transmute(addr);

        let x: u64 = 10;
        let y: u64 = 2;
        let res = f(x, y);

        println!("{} + {} = {}", x, y, res);
    }
}
