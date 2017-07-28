use std::fmt;
use std::mem::transmute;
use std::ops::Deref;

use super::*;

/// Enumeration of all the base types of the LLVM type system.
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum Kind {
    Void = 0,
    Half,
    Float,
    Double,
    X86_FP80,
    FP128,
    PPC_FP128,
    Label,
    Metadata,
    X86_MMX,
    Token,

    Integer,
    Function,
    Struct,
    Array,
    Pointer,
    Vector,
}

/// Should always be used as `&Type`.
///
/// `Type`s are owned by `Context` instances such that only one instance of a
/// specific `Type` exists per `Context`, e.g. only 1 `Type` for `i64` exists
/// per context. `Type`s are also never mutated and never destroyed, living for
/// the lifetime of the `Context` that owns them.
// TODO: can this be made into an unsized type?
pub struct Type(LLVMType);

impl Type {
    /// Returns the LLVMTypeRef instance that is being wrapped.
    pub(crate) fn as_raw(&self) -> LLVMTypeRef {
        unsafe { transmute::<&Self, LLVMTypeRef>(self) }
    }

    /// Constructs a safe, Rust-y reference to an instance of an LLVM type.
    /// Maked `unsafe` because the caller must ensure that the pointer is
    /// correct.
    pub(crate) unsafe fn from_raw<'a>(ptr: LLVMTypeRef) -> &'a Self {
        transmute::<LLVMTypeRef, &Self>(ptr)
    }

    /*/// Dumps a representation of the type to stderr via LLVM.
    // TODO: It this even necessary? The LLVM doc implies that it's only purpose
    // is to be callable from gdb, which I don't know how ensure yet.
    pub fn dump(&self) {
        unsafe { LLVMDumpType(self.as_raw()) };
    }*/

    // TODO: implement `context(&self) -> Context` (see llvm::Type::getContext)

    pub fn kind(&self) -> Kind {
        unsafe { transmute(LLVMGetTypeKind(self.as_raw())) }
    }

    // TODO: Implement all the is*Ty() variants and related methods (see
    // llvm/IR/Type.h lines 139-249)

    pub fn is_sized(&self) -> bool {
        unsafe { LLVMTypeIsSized(self.as_raw()) == 1 }
    }
}

// This counts as the llvm::Type::print method from the C++ API, though the C++
// version has more options.
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(
                f,
                "{}",
                llvm::String::from_mut(LLVMPrintTypeToString(self.as_raw()))
            )
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "llvm::Type({})", self)
    }
}

// Base types:

#[derive(Type)]
pub struct Void(Type);

#[derive(Type)]
pub struct Half(Type);

#[derive(Type)]
pub struct Float(Type);

#[derive(Type)]
pub struct Double(Type);

#[allow(non_camel_case_types)]
#[derive(Type)]
pub struct X86_FP80(Type);

#[allow(non_camel_case_types)]
#[derive(Type)]
pub struct FP128(Type);

#[allow(non_camel_case_types)]
#[derive(Type)]
pub struct PPC_FP128(Type);

#[derive(Type)]
pub struct Label(Type);

#[derive(Type)]
pub struct Metadata(Type);

#[allow(non_camel_case_types)]
#[derive(Type)]
pub struct X86_MMX(Type);

#[derive(Type)]
pub struct Token(Type);

/// Integer types are constructed with a size. Construct them with the methods
/// that `Context` provides.
#[derive(Type)]
pub struct Integer(Type);

/// A function type is a tuple consisting of a return type and an array of
/// parameter types.
// TODO: check how varargs work
#[derive(Type)]
pub struct Function(Type);

impl Function {
    pub fn new<'a>(return_type: &'a Type, param_types: &[&'a Type], is_var_args: bool) -> &'a Function {
        unsafe {
            Self::from_raw(LLVMFunctionType(
                return_type.as_raw(),
                transmute::<*const &Type, *mut LLVMTypeRef>(param_types.as_ptr()),
                param_types.len() as u32,
                is_var_args as LLVMBool
            ))
        }
    }
}

// TODO: Struct, Array, Pointer, Vector
