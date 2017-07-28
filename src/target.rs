use std::mem;
use std::ptr;

use super::*;

// Re-define enums in a more idiomatic way here.
// Unfortunately, they are only interchangable with their llvm-sys counterparts
// with a mem::transmute().
#[derive(Copy, Clone)]
#[repr(C)]
/// LLVMCodeGenOptLevel
pub enum CodeGenOptLevel {
    None = 0,
    Less = 1,
    Default = 2,
    Aggressive = 3,
}

#[derive(Copy, Clone)]
#[repr(C)]
/// LLVMRelocMode
pub enum RelocMode {
    Default = 0,
    Static = 1,
    PIC = 2,
    DynamicNoPic = 3,
}

#[derive(Copy, Clone)]
#[repr(C)]
/// LLVMCodeModel
pub enum CodeModel {
    Default = 0,
    JITDefault = 1,
    Small = 2,
    Kernel = 3,
    Medium = 4,
    Large = 5,
}

#[derive(Copy, Clone)]
#[repr(C)]
/// LLVMCodeGenFileType
pub enum CodeGenFileType {
    AssemblyFile = 0,
    ObjectFile = 1,
}

#[derive(Debug)]
pub struct Target {
    ptr: LLVMTargetRef,
}

impl_llvm_ref!(Target, LLVMTargetRef);

impl Target {
    // LLVMGetTargetFromName returns 0 for failure, but because it isn't using
    // error messages, this function just uses an Option rather than a Result
    // to signify failure.
    pub fn from_name(name: &AsRef<Str>) -> Option<Target> {
        let res = unsafe { LLVMGetTargetFromName(name.as_ref().as_ptr()) };

        if res.is_null() {
            None
        } else {
            unsafe { Some(Self::from_raw(res)) }
        }
    }

    pub fn from_triple(triple: &AsRef<Str>) -> Result<Target> {
        unsafe {
            let mut target_ptr: LLVMTargetRef = mem::uninitialized();
            let mut err_msg = ptr::null_mut::<i8>();
            LLVMGetTargetFromTriple(
                triple.as_ref().as_ptr(),
                &mut target_ptr,
                &mut err_msg as *mut *mut i8,
            );
            if target_ptr.is_null() {
                Err(llvm::String::from_mut(err_msg))
            } else {
                Ok(Self::from_raw(target_ptr))
            }
        }
    }

    pub fn create_target_machine(
        &self,
        triple: &AsRef<Str>,
        cpu: &AsRef<Str>,
        features: &AsRef<Str>,
        level: CodeGenOptLevel,
        reloc: RelocMode,
        model: CodeModel,
    ) -> TargetMachine {
        unsafe {
            TargetMachine::new(
                self,
                triple,
                cpu,
                features,
                mem::transmute(level),
                mem::transmute(reloc),
                mem::transmute(model),
            )
        }
    }
}

#[derive(Debug)]
pub struct TargetMachine {
    ptr: LLVMTargetMachineRef,
}

impl_llvm_ref!(TargetMachine, LLVMTargetMachineRef);

impl TargetMachine {
    pub fn new(
        target: &Target,
        triple: &AsRef<Str>,
        cpu: &AsRef<Str>,
        features: &AsRef<Str>,
        level: CodeGenOptLevel,
        reloc: RelocMode,
        model: CodeModel,
    ) -> TargetMachine {
        TargetMachine {
            ptr: unsafe {
                LLVMCreateTargetMachine(
                    target.ptr,
                    triple.as_ref().as_ptr(),
                    cpu.as_ref().as_ptr(),
                    features.as_ref().as_ptr(),
                    mem::transmute(level),
                    mem::transmute(reloc),
                    mem::transmute(model),
                )
            },
        }
    }

    pub fn create_data_layout(&self) -> TargetData {
        unsafe { TargetData { ptr: LLVMCreateTargetDataLayout(self.as_raw()) } }
    }

    pub fn emit_to_file(
        &mut self,
        module: &mut Module,
        path: &AsRef<Str>,
        file_type: CodeGenFileType,
    ) -> Result<()> {
        let mut err_msg = ptr::null_mut::<i8>();
        unsafe {
            LLVMTargetMachineEmitToFile(
                self.as_mut(),
                module.as_mut(),
                path.as_ref().as_ptr() as *mut i8,
                mem::transmute(file_type),
                &mut err_msg as *mut *mut i8,
            );
            if err_msg.is_null() {
                // no error message was set
                Ok(())
            } else {
                Err(llvm::String::from_mut(err_msg))
            }
        }
    }
}

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetMachine(self.as_mut());
        }
    }
}

#[derive(Debug)]
pub struct TargetData {
    ptr: LLVMTargetDataRef,
}

impl_llvm_ref!(TargetData, LLVMTargetDataRef);

impl Drop for TargetData {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeTargetData(self.as_mut());
        }
    }
}

/// Returns a string in the format of
///   CPU_TYPE-VENDOR-OPERATING_SYSTEM
/// or
///   CPU_TYPE-VENDOR-KERNEL-OPERATING_SYSTEM
pub fn get_default_target_triple() -> llvm::String {
    unsafe { llvm::String::from_mut(LLVMGetDefaultTargetTriple()) }
}

pub fn initialize_all_target_infos() {
    unsafe {
        LLVM_InitializeAllTargetInfos();
    }
}

pub fn initialize_all_targets() {
    unsafe {
        LLVM_InitializeAllTargets();
    }
}

pub fn initialize_all_target_mcs() {
    unsafe {
        LLVM_InitializeAllTargetMCs();
    }
}

pub fn initialize_all_asm_printers() {
    unsafe {
        LLVM_InitializeAllAsmPrinters();
    }
}

pub fn initialize_all_asm_parsers() {
    unsafe {
        LLVM_InitializeAllAsmParsers();
    }
}

pub fn initialize_all_disassemblers() {
    unsafe {
        LLVM_InitializeAllDisassemblers();
    }
}

// these LLVM_InitializeNative* functions all return 1 on failure, but failure
// means some macros were left undefined by the LLVM build, so we just ignore
// it.

pub fn initialize_native_target() {
    unsafe {
        LLVM_InitializeNativeTarget();
    }
}

pub fn initialize_native_asm_parser() {
    unsafe {
        LLVM_InitializeNativeAsmParser();
    }
}

pub fn initialize_native_asm_printer() {
    unsafe {
        LLVM_InitializeNativeAsmPrinter();
    }
}

pub fn initialize_native_disassembler() {
    unsafe {
        LLVM_InitializeNativeDisassembler();
    }
}
