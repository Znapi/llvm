//! Functionality for handling strings when working with LLVM.
//!
//! These functions assume that any strings returned by LLVM are correct.
use std::fmt;
use std::fmt::{Display, Debug};
use std::ffi::{CString, CStr, OsStr};
use std::mem::transmute;
use std::ops::Deref;
use std::marker::PhantomData;
use std::borrow::Borrow;

use super::*;

/// Representation of the data of a C-style (null terminated) string.
///
/// This representation is basically just a replacement for `std::ffi::CStr`.
/// `CStr` calculates a length when it is constructed from a raw pointer,
/// making it not 0-cost. The source code for `CStr` says that this is
/// unintended, so until it's fixed, I will use my replacement instead.
///
/// Because our `Str` is replacing `CStr`, I `impl Borrow<Str> for CString`
/// just as `CString` already has a `Borrow<CStr>`. `Deref`, however, can only
/// be implemented for one type, so where an argument of type `&CStr` could
/// normally be used, instead a generic `T: Borrow<Str>` type has to be used.
///
// TODO: make this an unsized type. Using a slice is an option, but it means
// &Str would be twice the size of a pointer. `std::ffi::CStr` uses the slice
// approach. Once it is possible to make an unsized type whose reference is
// the size of a pointer, CStr will probably implement it and my code can be
// changed to just use CStr.
pub struct Str {
    // PhantomData is used because Str is supposed to be an unsized type that
    // is only supposed to be used by reference.
    _marker: PhantomData<i8>,
}

impl Str {
    /// 0-cost cast to an `&llvm::Str` from a null-terminated string pointer.
    pub const unsafe fn from_ptr<'a>(ptr: *const i8) -> &'a Str {
        transmute::<*const i8, &'a Str>(ptr)
    }

    pub fn as_ptr(&self) -> *const i8 {
        unsafe { transmute::<&Str, *const i8>(self) }
    }

    pub(crate) unsafe fn as_mut(&self) -> *mut i8 {
        transmute::<&Str, *mut i8>(self)
    }

    /// Creates a string slice pointing to the data of this `llvm::Str`, not
    /// including the null-terminator. This performs a length calculation, so
    /// this conversion isn't completely free.
    fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(CStr::from_ptr(self.as_ptr()).to_bytes()) }
    }
}

/*impl Borrow<Str> for CStr {
    fn borrow(&self) -> &Str {
        unsafe { Str::from_ptr(self.as_ptr()) }
    }
}*/

impl Borrow<Str> for CString {
    fn borrow(&self) -> &Str {
        unsafe { Str::from_ptr(self.as_ptr()) }
    }
}

/*impl AsRef<Str> for CStr {
    fn as_ref(&self) -> &Str {
        unsafe { Str::from_ptr(self.as_ptr()) }
    }
}

impl AsRef<Str> for CString {
    fn as_ref(&self) -> &Str {
        unsafe { Str::from_ptr(self.as_ptr()) }
    }
}*/

impl AsRef<OsStr> for Str {
    fn as_ref(&self) -> &OsStr {
        OsStr::new(self.as_str())
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Wrapper for `llvm::Str`s that LLVM gave us ownership of.
///
/// The LLVM C API sometimes returns strings that we have to `free`, so we wrap
/// them with this at 0 cost.
pub struct String {
    ptr: *mut i8,
}

impl String {
    /// 0-cost cast to an `llvm::String` from a pointer to an owned string that
    /// must originate from LLVM.
    pub(crate) unsafe fn from_mut(ptr: *mut i8) -> String {
        String { ptr }
    }
}

impl Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Str as Display>::fmt(self, f)
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Str as Debug>::fmt(self, f)
    }
}

impl Deref for String {
    type Target = Str;

    fn deref(&self) -> &Self::Target {
        unsafe { Str::from_ptr(self.ptr) }
    }
}

impl Borrow<Str> for String {
    fn borrow(&self) -> &Str {
        unsafe { Str::from_ptr(self.as_ptr()) }
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeMessage(self.ptr);
        }
    }
}

/// Turn non-null terminated string literal into null-terminated
/// `&'static llvm::Str`.
///
/// Passing no argument creates an empty string, and is equivalent to
/// `llvmstr!("")`.
///
/// # Example
///
/// ```rust
/// #[macro_use]extern crate llvm;
/// # fn main() {
/// # let mut context = llvm::Context::new();
/// let mut my_module = context.create_module_with_name(llvmstr!("my module"));
/// # }
/// ```
// TODO: when stmt_expr_attributes (rust issue #15701) is finished, uncomment
// the `#[allow(unused_unsafe)]` below
#[macro_export]
macro_rules! llvmstr {
    ($s:expr) => {
        //#[allow(unused_unsafe)]
        unsafe { llvm::Str::from_ptr(concat!($s, "\0").as_ptr() as *mut i8) }
    };
    () => {
        //#[allow(unused_unsafe)]
        unsafe { llvm::Str::from_ptr(&mut 0i8 as *mut i8) }
    }
}
