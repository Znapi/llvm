extern crate libc;
extern crate llvm_sys;

use std::ffi::{CString, CStr};

use llvm_sys::prelude::*;
use llvm_sys::core as llvm;

// This should only be used for static strings
macro_rules! c_str_to_str {
    ($s:expr) => {
        ::std::str::from_utf8(CStr::from_ptr($s).to_bytes()).unwrap()
    }
}

#[macro_use]
mod macros;
#[macro_use] mod string;
mod context;
mod types;
mod builder;
mod module;
mod function;
mod pass_manager;
mod target;
mod execution_engine;
mod value;

// TODO: This was to maintain compatiblity, we should remove this
pub use string::*;
pub use context::*;
pub use types::*;
pub use builder::*;
pub use module::*;
pub use function::*;
pub use pass_manager::*;
pub use target::*;
pub use execution_engine::*;
pub use value::*;

pub fn set_value_name(val: LLVMValueRef, name: &str) {
    let c_name = CString::new(name).unwrap();
    unsafe {
        llvm::LLVMSetValueName(val, c_name.as_ptr());
    }
}
// hacky namespacing follows:
use string as llvm; // so we can do llvm::String in submodules to refer to string::String
//use std::string::String; // so that plain String in submodules still refers to std::string::String
use string::Str;

/// Convenience type
pub type Result<T> = std::result::Result<T, llvm::String>;
