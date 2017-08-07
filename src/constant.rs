//! Constants

use std::mem::transmute;

use super::*;

/// Should only be used by immutable reference, `&Constant`.
pub struct Constant(LLVMValue); // TODO: mark as unsized
// TODO: Display and Debug

impl<'a> From<&'a Constant> for LLVMValueRef {
    fn from(con: &'a Constant) -> LLVMValueRef {
        unsafe { transmute::<&Constant, LLVMValueRef>(con) }
    }
}

impl Constant {
    pub unsafe fn from_raw<'a>(ptr: LLVMValueRef) -> &'a Self {
        transmute::<LLVMValueRef, &Self>(ptr)
    }

    pub fn is_null(&self) -> bool {
        unsafe { LLVMIsNull(self.into()) == 1 }
    }
}

// TODO: Many decendants of llvm::Constant don't seem to actually be the kind of
// constant this module wraps. For example, Functions have mutable attributes
// and such. Exactly what `Constant` represents and how to represent these other
// types still needs to be figured out.
/*macro_rules! subclass {
    ($t:ty, $super:ty) => {
        impl Deref for $t {
            type Target = $super;

            fn deref(&self) -> &Self::Target {
                unsafe { transmute::<&Self, &Self::Target>(self) }
            }
        }

        // This would not be needed if the compiler could infer that the
        // From<&Type> for LLVMValueRef above worked on &SubType.
        impl<'a> From<&'a $t> for LLVMValueRef {
            fn from(ty: &'a $t) -> LLVMValueRef {
                unsafe { transmute::<&$t, LLVMValueRef>(ty) }
            }
        }

        impl $t {
            pub unsafe fn from_raw<'a>(ptr: LLVMValueRef) -> &'a Self {
                transmute::<LLVMValueRef, &Self>(ptr)
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.deref().fmt(f)
            }
        }

        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "llvm::{}({})", stringify!($t), self)
            }
        }
    }
}

pub struct Function(Constant);
subclass!(Function, Constant);
*/
