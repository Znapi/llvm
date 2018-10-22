//! LLVM types.
//!
//! LLVM uses a number of base types, which are enumerated by [`Kind`], and of
//! which [`Type`] acts as the "superclass". See [`Type`] for more
//! information.
//!
//! [`Kind`]: enum.Kind.html
//! [`Type`]: struct.Type.html

use std::fmt;
use std::mem::transmute;
use std::ops::Deref;

use libc::c_ulonglong;

use llvm_sys::prelude::*;
use llvm_sys::core::*;
use llvm_sys::*;

use super::*;

/// Enumeration of all the base types of the LLVM type system. Used for safe
/// downcasting of `Type`.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Kind<'a> {
    Void(&'a Void),
    Half(&'a Half),
    Float(&'a Float),
    Double(&'a Double),
    X86_FP80(&'a X86_FP80),
    FP128(&'a FP128),
    PPC_FP128(&'a PPC_FP128),
    Label(&'a Label),
    Integer(&'a Integer),
    Function(&'a Function),
    Struct(&'a Struct),
    Array(&'a Array),
    Pointer(&'a Pointer),
    Vector(&'a Vector),
    Metadata(&'a Metadata),
    X86_MMX(&'a X86_MMX),
    Token(&'a Token),
}

/// The "superclass" of the LLVM base types. Types can only be passed as
/// immutable references, `&Type`s.
///
/// `&Type`s are constructed using the `*_type` methods on a `Context`
///
/// # Ownership
///
/// `Type`s are owned by `Context` instances such that only one instance of a
/// specific `Type` exists per `Context`, e.g. only 1 `Float` instance exists
/// per `Context`. Once created, `Type`s are never mutated nor destroyed,
/// living for the lifetime of the `Context` that they belong to.
///
/// # Casting to and from Subtypes
///
/// Any "subclass" of `Type`, such as `Integer`, will be implicitly upcast into
/// `Type`, where necessary, with zero cost. Going the other way and performing
/// downcast, on the other hand, requires an enum lookup, meaning it's not
/// free. Downcasts can only be performed explicitly with the [`downcast`] or
/// [`try_as_*`] methods.
///
/// # Representation
///
/// Because the LLVM C API represents all types as `LLVMTypeRef`s, or `*mut
/// LLVMType`s (in Rust's type system), these `LLVMTypeRef`s are simply
/// `transmute`d into `&Type`s, where `Type` is an opaque, unsized type (much
/// like `str`), and whose lifetimes are tied to the `Context`s that they
/// belong to. This allows Rust's type system to enforce the behavior of
/// LLVM types (that they are immutable objects belonging to LLVM contexts)
/// with zero runtime cost.
///
/// Some functions in the C API only expect only `LLVMTypeRef`s that belong to
/// a specific branch of LLVM's type hierarchy. The [newtype] pattern is used
/// to model this in Rust with type safety while maintaining 0-cost. [Automatic
/// deref coercions] allow implicit upcasts to `&Type`s.
///
/// [`downcast`]: #method.downcast
/// [`try_as_*`]: #method.try_as_void
/// [newtype]: https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html#using-the-newtype-pattern-for-type-safety-and-abstraction
/// [Automatic deref coercions]: https://doc.rust-lang.org/book/second-edition/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
pub struct Type(LLVMType); // TODO: mark this as an unsized type

impl<'a> From<&'a Type> for LLVMTypeRef {
    fn from(ty: &'a Type) -> LLVMTypeRef {
        unsafe { transmute::<&Type, LLVMTypeRef>(ty) }
    }
}

macro_rules! try_as_fns {
    ($(pub fn $name:ident -> $variant:tt)*) => {
        $(
            /// Attempt a downcast, returning `None` if the type of `self`
            /// doesn't match the type requested.
            ///
            /// # Example
            ///
            /// ```rust
            /// # fn main() {
            /// # let context = llvm::Context::new();
            /// let generic_type: &llvm::Type = context.i16_type(); // upcast
            /// println!("{:?}", generic_type); // width() is not a method for &Type
            ///
            /// let t = generic_type.try_as_integer().unwrap();
            /// println!("{:?} {}", t, t.width());
            /// # }
            /// ```
            pub fn $name<'a>(&'a self) -> Option<&'a $variant> {
                if let Kind::$variant(t) = self.downcast() {
                    Some(t)
                }
                else {
                    None
                }
            }
        )*
    }
}

impl Type {
    pub unsafe fn from_raw<'a>(ptr: LLVMTypeRef) -> &'a Self {
        transmute::<LLVMTypeRef, &Self>(ptr)
    }

    /// Downcast an `&Type`, returning a variant of `Kind` that encodes the
    /// type information and contains the result of the downcast.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() {
    /// # let context = llvm::Context::new();
    /// let generic_type: &llvm::Type = context.i16_type(); // upcast
    /// println!("{:?}", generic_type); // width() is not a method for &Type
    ///
    /// if let llvm::types::Kind::Integer(t) = generic_type.downcast() {
    ///     println!("{:?} {}", t, t.width()); // width() can be used on &Integer
    /// }
    /// # }
    /// ```
    pub fn downcast(&self) -> Kind {
        unsafe {
            match LLVMGetTypeKind(self.into()) {
                LLVMTypeKind::LLVMVoidTypeKind => Kind::Void(transmute(self)),
                LLVMTypeKind::LLVMHalfTypeKind => Kind::Half(transmute(self)),
                LLVMTypeKind::LLVMFloatTypeKind => Kind::Float(transmute(self)),
                LLVMTypeKind::LLVMDoubleTypeKind => Kind::Double(transmute(self)),
                LLVMTypeKind::LLVMX86_FP80TypeKind => Kind::X86_FP80(transmute(self)),
                LLVMTypeKind::LLVMFP128TypeKind => Kind::FP128(transmute(self)),
                LLVMTypeKind::LLVMPPC_FP128TypeKind => Kind::PPC_FP128(transmute(self)),
                LLVMTypeKind::LLVMLabelTypeKind => Kind::Label(transmute(self)),
                LLVMTypeKind::LLVMIntegerTypeKind => Kind::Integer(transmute(self)),
                LLVMTypeKind::LLVMFunctionTypeKind => Kind::Function(transmute(self)),
                LLVMTypeKind::LLVMStructTypeKind => Kind::Struct(transmute(self)),
                LLVMTypeKind::LLVMArrayTypeKind => Kind::Array(transmute(self)),
                LLVMTypeKind::LLVMPointerTypeKind => Kind::Pointer(transmute(self)),
                LLVMTypeKind::LLVMVectorTypeKind => Kind::Vector(transmute(self)),
                LLVMTypeKind::LLVMMetadataTypeKind => Kind::Metadata(transmute(self)),
                LLVMTypeKind::LLVMX86_MMXTypeKind => Kind::X86_MMX(transmute(self)),
                LLVMTypeKind::LLVMTokenTypeKind => Kind::Token(transmute(self)),
            }
        }
    }

    try_as_fns! {
        pub fn try_as_void -> Void
        pub fn try_as_half -> Half
        pub fn try_as_float -> Float
        pub fn try_as_double -> Integer
        pub fn try_as_x86_fp80 -> X86_FP80
        pub fn try_as_fp128 -> FP128
        pub fn try_as_ppc_fp128 -> PPC_FP128
        pub fn try_as_label -> Label
        pub fn try_as_integer -> Integer
        pub fn try_as_function -> Function
        pub fn try_as_struct -> Struct
        pub fn try_as_array -> Array
        pub fn try_as_pointer -> Pointer
        pub fn try_as_vector -> Vector
        pub fn try_as_metadata -> Metadata
        pub fn try_as_x86_mmx -> X86_MMX
        pub fn try_as_token -> Token
    }

    /// Return true if the type has a known sized. For subclasses of `Type`
    /// the result of this method can be assumed.
    pub fn is_sized(&self) -> bool {
        unsafe { LLVMTypeIsSized(self.into()) == 1 }
    }

    pub fn pointer(&self) -> &Pointer {
        unsafe { Pointer::from_raw(LLVMPointerType(self.into(), 0)) }
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
                String::from_mut(LLVMPrintTypeToString(self.into()))
            )
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "llvm::Type({})", self)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        LLVMTypeRef::from(self) == LLVMTypeRef::from(other)
    }
}

impl Eq for Type {}

macro_rules! subclass {
    ($t:ty, $super:ty) => {
        impl Deref for $t {
            type Target = $super;

            fn deref(&self) -> &Self::Target {
                unsafe { transmute::<&Self, &Self::Target>(self) }
            }
        }

        // This would not be needed if the compiler could infer that the
        // From<&Type> for LLVMTypeRef above worked on &SubType.
        impl<'a> From<&'a $t> for LLVMTypeRef {
            fn from(ty: &'a $t) -> LLVMTypeRef {
                unsafe { transmute::<&$t, LLVMTypeRef>(ty) }
            }
        }

        impl $t {
            pub unsafe fn from_raw<'a>(ptr: LLVMTypeRef) -> &'a Self {
                transmute::<LLVMTypeRef, &Self>(ptr)
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.deref().fmt(f)
            }
        }

        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "llvm::types::{}({})", stringify!($t), self)
            }
        }
    }
}

macro_rules! impl_type {
    ($t:ty) => { subclass!($t, Type); }
}

// Base types:

/// Type with no size
pub struct Void(Type);
impl_type!(Void);

/// 16 bit floating point type
pub struct Half(Type);
impl_type!(Half);

/// 32 bit floating point type
pub struct Float(Type);
impl_type!(Float);

/// 64 bit floating point type
pub struct Double(Type);
impl_type!(Double);

impl Double {
    pub fn constant<'a>(&'a self, val: f64) -> &'a Constant {
        unsafe { Constant::from_raw(LLVMConstReal(self.into(), val)) }
    }
}

/// 80 bit floating point type (X87)
#[allow(non_camel_case_types)]
pub struct X86_FP80(Type);
impl_type!(X86_FP80);

/// 128 bit floating point type (112-bit mantissa)
#[allow(non_camel_case_types)]
pub struct FP128(Type);
impl_type!(FP128);

/// 128 bit floating point type (two 64-bits)
#[allow(non_camel_case_types)]
pub struct PPC_FP128(Type);
impl_type!(PPC_FP128);

/// Labels
pub struct Label(Type);
impl_type!(Label);

/// Metadata
pub struct Metadata(Type);
impl_type!(Metadata);

/// X86 MMX
#[allow(non_camel_case_types)]
pub struct X86_MMX(Type);
impl_type!(X86_MMX);

/// Tokens
pub struct Token(Type);
impl_type!(Token);

/// Aribitrary bit width integers
pub struct Integer(Type);
impl_type!(Integer);

impl Integer {
    fn constant<'a>(&'a self, val: c_ulonglong, sign_extend: bool) -> &'a Constant {
        unsafe {
            Constant::from_raw(LLVMConstInt(
                self.into(),
                val,
                if sign_extend { 1 } else { 0 },
            ))
        }
    }

    /// Returns the bit width of an `Integer` type.
    pub fn width(&self) -> u32 {
        unsafe { LLVMGetIntTypeWidth(self.into()) }
    }
}

/// Signed integer
pub struct Int(Integer);
subclass!(Int, Integer);

impl Int {
    pub fn constant<'a>(&'a self, val: i64) -> &'a Constant {
        self.deref().constant(val as c_ulonglong, true)
    }
}

/// Unsinged integer
pub struct UInt(Integer);
subclass!(UInt, Integer);

impl UInt {
    pub fn constant<'a>(&'a self, val: u64) -> &'a Constant {
        self.deref().constant(val as c_ulonglong, true)
    }
}

/// Function
///
/// Function types are tuples consisting of a return type and an array of
/// parameter types.
pub struct Function(Type);
impl_type!(Function);

impl Function {
    /// Construct a new `Function` type.
    pub fn new<'a>(
        return_type: &'a Type,
        param_types: &[&'a Type],
        is_var_args: bool,
    ) -> &'a Function {
        unsafe {
            Function::from_raw(LLVMFunctionType(
                return_type.into(),
                transmute::<*const &Type, *mut LLVMTypeRef>(
                    param_types.as_ptr(),
                ),
                param_types.len() as u32,
                is_var_args as LLVMBool,
            ))
        }
    }
}

/// Structures
pub struct Struct(Type);
impl_type!(Struct);

/// Arrays
pub struct Array(Type);
impl_type!(Array);

/// Pointers
pub struct Pointer(Type);
impl_type!(Pointer);

/// SIMD 'packed' format, or other vector type
pub struct Vector(Type);
impl_type!(Vector);
