use std::fmt;

use super::*;

#[derive(Debug)]
pub struct Module {
    ptr: LLVMModuleRef,
}

impl_llvm_ref!(Module, LLVMModuleRef);

impl Module {
    pub fn dump(&self) {
        unsafe { LLVMDumpModule(self.as_raw()) };
    }

    pub fn set_data_layout_str(&mut self, data_layout_str: &AsRef<Str>) {
        unsafe { LLVMSetDataLayout(self.as_mut(), data_layout_str.as_ref().as_ptr()) };
    }

    pub fn set_data_layout(&mut self, data_layout: &mut TargetData) {
        unsafe { LLVMSetModuleDataLayout(self.as_mut(), data_layout.as_raw()) };
    }

    pub fn get_target_triple(&self) -> &'static llvm::Str {
        unsafe { llvm::Str::from_ptr(LLVMGetTarget(self.as_raw())) }
    }

    pub fn set_target_triple(&mut self, triple: &AsRef<Str>) {
        unsafe { LLVMSetTarget(self.as_mut(), triple.as_ref().as_ptr()) };
    }

    pub fn add_function(&mut self, func_ty: &types::Function, name: &AsRef<Str>) -> LLVMValueRef {
        unsafe { LLVMAddFunction(self.as_mut(), name.as_ref().as_ptr(), func_ty.into()) }
    }

    pub fn print_to_file(&self, path: &AsRef<Str>) -> Result<()> {
        let mut em: usize = 0;
        let em_ptr: *mut usize = &mut em;
        unsafe {
            LLVMPrintModuleToFile(
                self.as_raw(),
                path.as_ref().as_ptr(),
                em_ptr as *mut *mut i8,
            );
            if em == 0 {
                // no error message was set
                Ok(())
            } else {
                Err(llvm::String::from_mut(em_ptr as *mut i8))
            }
        }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(
                f,
                "{}",
                llvm::String::from_mut(LLVMPrintModuleToString(self.as_raw()))
            )
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { LLVMDisposeModule(self.as_mut()) };
    }
}
