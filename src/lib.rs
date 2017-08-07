//! A safe Rust API to LLVM.
extern crate libc;
extern crate llvm_sys;

/// Trait denoting types that wrap an LLVM*Ref.
pub(crate) trait LLVMRef {
    /// The LLVM*Ref type that the implementing type wraps.
    type LLVMRef;

    /// Create a new instance of the implementor from the given corresponding
    /// LLVM*Ref. It is up to the caller to ensure that it is not violating
    /// Rust's ownership principle, e.g. making multiple mutable references
    /// with the same LLVM*Ref.
    unsafe fn from_raw(ptr: Self::LLVMRef) -> Self;

    /// Returns the LLVM*Ref that the implementing type wraps. Marked as
    /// `unsafe` because it converts an immutable reference to a mutable one.
    /// This is convenient for the many functions that just want to borrow an
    /// object immutably and pass it to an LLVM function that won't mutate it,
    /// but still has the type `*mut` because C.
    unsafe fn as_raw(&self) -> Self::LLVMRef;

    /// Returns the LLVM*Ref that the implementing type wraps. Though `as_raw`
    /// would work just fine, this communicates the intention.
    fn as_mut(&mut self) -> Self::LLVMRef;
}

macro_rules! impl_llvm_ref {
    ($name:ty, $ref:ty) => {
        impl LLVMRef for $name {
            type LLVMRef = $ref;
            
            unsafe fn from_raw(ptr: Self::LLVMRef) -> Self {
                Self { ptr }
            }

            unsafe fn as_raw(&self) -> Self::LLVMRef {
                self.ptr
            }

            fn as_mut(&mut self) -> Self::LLVMRef {
                self.ptr
            }
        }
    }
}

// Import all of llvm_sys. All (direct) submodules can simply import `super::*`
// to be able to use anything from llvm_sys.
use llvm_sys::prelude::*;
use llvm_sys::*;
use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::initialization::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;

#[macro_use]
mod string;
mod context;
pub mod types;
mod constant;
mod builder;
mod module;
mod pass_manager;
mod target;
mod execution_engine;

pub use string::*;
pub use context::*;
pub use types::{Type, ContextType};
pub use constant::*;
pub use builder::*;
pub use builder::IntPredicate;
pub use module::*;
pub use pass_manager::*;
pub use target::*;
pub use execution_engine::*;

// hacky namespacing follows:
use string as llvm; // so we can do llvm::String in submodules to refer to string::String

/// Convenience type
pub type Result<T> = std::result::Result<T, llvm::String>;
