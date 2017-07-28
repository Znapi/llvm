use super::*;

#[derive(Debug)]
pub struct PassManager {
    ptr: LLVMPassManagerRef,
}

impl_llvm_ref!(PassManager, LLVMPassManagerRef);

impl PassManager {
    pub fn new() -> PassManager {
        unsafe { Self::from_raw(LLVMCreatePassManager()) }
    }
}

#[derive(Debug)]
pub struct PassRegistry {
    ptr: LLVMPassRegistryRef,
}

impl_llvm_ref!(PassRegistry, LLVMPassRegistryRef);

macro_rules! create_init_fns {
    ( $($llvm_fn_name:ident => $fn_name:ident),* ) => {
        $(pub fn $fn_name(&mut self) { unsafe {
            $llvm_fn_name(self.as_mut());
        } })*
    }
}

impl PassRegistry {
    create_init_fns! {
        LLVMInitializeCore => initialize_core,
        LLVMInitializeTransformUtils => initialize_transform_utils,
        LLVMInitializeScalarOpts => initialize_scalar_opts,
        LLVMInitializeObjCARCOpts => initialize_obj_carc_opts,
        LLVMInitializeVectorization => initialize_vectorization,
        LLVMInitializeInstCombine => initialize_inst_combine,
        LLVMInitializeIPO => initialize_ipo,
        LLVMInitializeInstrumentation => initialize_instrumentation,
        LLVMInitializeAnalysis => initialize_analysis,
        LLVMInitializeIPA => initialize_ipa,
        LLVMInitializeCodeGen => initialize_code_gen,
        LLVMInitializeTarget => initialize_target
    }
}
