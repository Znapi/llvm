use super::*;

/// A `Context` is an execution state for the core LLVM IR system. Multiple
/// `Context`s can exist simultaneously. A single context is not thread safe,
/// but different contexts on different threads can execute simultaneously.
#[derive(Debug)]
pub struct Context {
    ptr: LLVMContextRef,
}

impl_llvm_ref!(Context, LLVMContextRef);

impl Context {
    pub fn new() -> Self {
        unsafe { Self::from_raw(LLVMContextCreate()) }
    }

    pub fn create_builder(&mut self) -> Builder {
        unsafe { Builder::from_raw(LLVMCreateBuilderInContext(self.as_mut())) }
    }

    pub fn create_module_with_name(&mut self, name: &AsRef<Str>) -> Module {
        unsafe {
            Module::from_raw(LLVMModuleCreateWithNameInContext(
                name.as_ref().as_ptr(),
                self.as_mut(),
            ))
        }
    }

    pub fn append_basic_block(
        &mut self,
        func: LLVMValueRef,
        name: &AsRef<Str>,
    ) -> LLVMBasicBlockRef {
        unsafe { LLVMAppendBasicBlockInContext(self.as_mut(), func, name.as_ref().as_ptr()) }
    }

    /// Returns a reference to the `types::Void` instance for an instance of
    /// `Context`.
    pub fn void_type<'a>(&'a self) -> &'a types::Void {
        unsafe { types::Void::from_raw(LLVMVoidTypeInContext(self.as_raw())) }
    }

    pub fn i1_type(&self) -> &types::Integer {
        unsafe { types::Integer::from_raw(LLVMInt1TypeInContext(self.as_raw())) }
    }

    pub fn i8_type(&self) -> &types::Integer {
        unsafe { types::Integer::from_raw(LLVMInt8TypeInContext(self.as_raw())) }
    }

    pub fn i16_type(&self) -> &types::Integer {
        unsafe { types::Integer::from_raw(LLVMInt16TypeInContext(self.as_raw())) }
    }

    pub fn i32_type(&self) -> &types::Integer {
        unsafe { types::Integer::from_raw(LLVMInt32TypeInContext(self.as_raw())) }
    }

    pub fn i64_type(&self) -> &types::Integer {
        unsafe { types::Integer::from_raw(LLVMInt64TypeInContext(self.as_raw())) }
    }

    pub fn i128_type(&self) -> &types::Integer {
        unsafe { types::Integer::from_raw(LLVMInt128TypeInContext(self.as_raw())) }
    }

    pub fn integer_type(&self, num_bits: u32) -> &types::Integer {
        unsafe { types::Integer::from_raw(LLVMIntTypeInContext(self.as_raw(), num_bits)) }
    }

    /*/// Creates a constant in this context
    /// The value must implement the trait `IntoValue`
    pub fn cons<T: IntoConstValue>(&self, val: T) -> LLVMValueRef {
        val.gen_const(self)
    }*/
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.as_mut());
        }
    }
}
