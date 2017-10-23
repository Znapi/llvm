use std::mem;

use super::*;

#[derive(Debug)]
pub struct ExecutionEngine {
    ptr: LLVMExecutionEngineRef,
}

impl_llvm_ref!(ExecutionEngine, LLVMExecutionEngineRef);

impl ExecutionEngine {
    /// Creates an execution engine for the given module. The `ExecutionEngine`
    /// takes ownership of the module.
    pub fn create_for_module(module: Module) -> Result<ExecutionEngine> {
        unsafe {
            let mut ee = mem::uninitialized();
            let mut out = mem::zeroed();

            let res = LLVMCreateExecutionEngineForModule(&mut ee, module.as_raw(), &mut out);

            if res == 0 { // if no errors
                mem::forget(module);
                Ok(Self::from_raw(ee))
            } else {
                Err(llvm::String::from_mut(out))
                // module is implicitly disposed
            }
        }
    }

    pub fn get_function_address(&self, fname: &AsRef<Str>) -> Option<extern "C" fn()> {
        unsafe {
            let addr = LLVMGetFunctionAddress(self.ptr, fname.as_ref().as_ptr());

            if addr == 0 {
                None
            } else {
                Some(mem::transmute(addr))
            }
        }
    }
}

impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.ptr);
        }
    }
}

pub fn link_in_mcjit() {
    unsafe {
        LLVMLinkInMCJIT();
    }
}
