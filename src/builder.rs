use std::mem::transmute;

use super::*;

#[derive(Debug)]
#[repr(C)]
pub enum IntPredicate {
    Eq = 32,
    Ne,
    Ugt,
    Uge,
    Ult,
    Ule,
    Sgt,
    Sge,
    Slt,
    Sle,
}

macro_rules! build_named_ops {
    ($($(#[$attr:meta])*pub fn $name:ident($($argn:ident: $argty:ty),*) { $llvm_fn:path })*) => {
        $(
            /// Specifying a name is optional; just pass an empty string
            $(#[$attr])*pub fn $name(&mut self, $($argn: $argty),*, name: &AsRef<Str>) -> LLVMValueRef {
            unsafe {
                $llvm_fn(self.as_mut(),
                         $($argn),*,
                         name.as_ref().as_ptr())
            }
        })*
    }
}
macro_rules! build_ops {
    ($($(#[$attr:meta])*pub fn $name:ident($($argn:ident: $argty:ty),*) { $llvm_fn:path })*) => {
        $($(#[$attr])*pub fn $name(&mut self, $($argn: $argty),*) -> LLVMValueRef {
            unsafe { $llvm_fn(self.as_mut(), $($argn),*) }
        })*
    }
}

#[derive(Debug)]
pub struct Builder {
    ptr: LLVMBuilderRef,
}

impl_llvm_ref!(Builder, LLVMBuilderRef);

// http://llvm.org/docs/doxygen/html/group__LLVMCCoreInstructionBuilder.html
//TODO: Get/Set Volatile
//TODO: Get/Set Ordering
//TODO: Almost everything from LLVMBuildAdd and upwards

impl Builder {
    build_named_ops! {
        pub fn build_add(lhs: LLVMValueRef, rhs: LLVMValueRef) { LLVMBuildAdd }
        pub fn build_sub(lhs: LLVMValueRef, rhs: LLVMValueRef) { LLVMBuildSub }
        pub fn build_mul(lhs: LLVMValueRef, rhs: LLVMValueRef) { LLVMBuildMul }

        pub fn build_fadd(lhs: LLVMValueRef, rhs: LLVMValueRef) { LLVMBuildFAdd }
        pub fn build_fsub(lhs: LLVMValueRef, rhs: LLVMValueRef) { LLVMBuildFSub }
        pub fn build_fmul(lhs: LLVMValueRef, rhs: LLVMValueRef) { LLVMBuildFMul }
        pub fn build_fdiv(lhs: LLVMValueRef, rhs: LLVMValueRef) { LLVMBuildFDiv }

/*pub fn build_nswadd, LLVMBuildNSWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_nswsub, LLVMBuildNSWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_nswmul, LLVMBuildNSWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

pub fn build_nuwadd, LLVMBuildNUWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_nuwsub, LLVMBuildNUWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_nuwmul, LLVMBuildNUWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

pub fn build_udiv, LLVMBuildUDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_sdiv, LLVMBuildSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_exact_sdiv, LLVMBuildExactSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);

pub fn build_urem, LLVMBuildURem, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_srem, LLVMBuildSRem, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_frem, LLVMBuildFRem, lhs: LLVMValueRef, rhs: LLVMValueRef);

pub fn build_shl, LLVMBuildShl, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_lshr, LLVMBuildLShr, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_ashr, LLVMBuildAShr, lhs: LLVMValueRef, rhs: LLVMValueRef);

pub fn build_and, LLVMBuildAnd, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_or, LLVMBuildOr, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_xor, LLVMBuildXor, lhs: LLVMValueRef, rhs: LLVMValueRef);*/

// TODO: LLVMBuildBinOp

        pub fn build_neg(v: LLVMValueRef) { LLVMBuildNeg }
/*pub fn build_fneg, LLVMBuildFNeg, v: LLVMValueRef);
pub fn build_nswneg, LLVMBuildNSWNeg, v: LLVMValueRef);
pub fn build_nuwneg, LLVMBuildNUWNeg, v: LLVMValueRef);

pub fn build_not, LLVMBuildNot, v: LLVMValueRef);

pub fn build_malloc, LLVMBuildMalloc, typ: LLVMTypeRef);
pub fn build_array_malloc, LLVMBuildArrayMalloc, typ: LLVMTypeRef,
                                                              val: LLVMValueRef);

pub fn build_alloca, LLVMBuildAlloca, ty: LLVMTypeRef);
pub fn build_array_alloca, LLVMBuildArrayAlloca, ty: LLVMTypeRef,
                                                              val: LLVMValueRef);

build_op!(build_free, LLVMBuildFree, pval: LLVMValueRef);
pub fn build_load, LLVMBuildLoad, ptr: LLVMValueRef);
build_op!(build_store, LLVMBuildStore,  val: LLVMValueRef, pval: LLVMValueRef);

pub fn build_trunc, LLVMBuildTrunc, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_zext(val: LLVMValueRef, dest_ty: &Type) { LLVMBuildZExt }
pub fn build_sext, LLVMBuildSExt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_fp_to_ui, LLVMBuildFPToUI, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_fp_to_si, LLVMBuildFPToSI, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_ui_to_fp, LLVMBuildUIToFP, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_si_to_fp, LLVMBuildSIToFP, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_fp_trunc, LLVMBuildFPTrunc, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_fp_ext, LLVMBuildFPExt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_ptr_to_int, LLVMBuildPtrToInt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_int_to_ptr, LLVMBuildIntToPtr, val: LLVMValueRef, dest_ty: LLVMTypeRef);

pub fn build_bit_cast, LLVMBuildBitCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_addr_space_cast, LLVMBuildAddrSpaceCast, val: LLVMValueRef,
                                                                   dest_ty: LLVMTypeRef);
pub fn build_zext_or_bit_cast, LLVMBuildZExtOrBitCast, val: LLVMValueRef,
                                                                    dest_ty: LLVMTypeRef);
pub fn build_sext_or_bit_cast, LLVMBuildSExtOrBitCast, val: LLVMValueRef,
                                                                    dest_ty: LLVMTypeRef);
pub fn build_trunc_or_bit_cast, LLVMBuildTruncOrBitCast, val: LLVMValueRef,
                                                                      dest_ty: LLVMTypeRef);

// TODO: improve LLVMOpcode
pub fn build_cast, LLVMBuildCast, op: LLVMOpcode,
                                               val: LLVMValueRef,
                                               dest_ty: LLVMTypeRef);

pub fn build_pointer_cast, LLVMBuildPointerCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_int_cast, LLVMBuildIntCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);
pub fn build_fpcast, LLVMBuildFPCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);

pub fn build_phi, LLVMBuildPhi, ty: LLVMTypeRef);
//build_call is manually defined in impl Builder
pub fn build_select, LLVMBuildSelect, i: LLVMValueRef,
                                                   the: LLVMValueRef,
                                                   els: LLVMValueRef);

pub fn build_vaarg, LLVMBuildVAArg, list: LLVMValueRef, ty: LLVMTypeRef);


pub fn build_extract_element, LLVMBuildExtractElement, vec_val: LLVMValueRef,
                                                                    index: LLVMValueRef);
pub fn build_insert_element, LLVMBuildInsertElement, vec_val: LLVMValueRef,
                                                                  eltval: LLVMValueRef,
                                                                  index: LLVMValueRef);
pub fn build_shuffle_vector, LLVMBuildShuffleVector, v1: LLVMValueRef,
                                                                  v2: LLVMValueRef,
                                                                  mask: LLVMValueRef);

// TODO: Both these types use unsigned, change this to libc::unsigned
pub fn build_extract_value, LLVMBuildExtractValue, aggval: LLVMValueRef,
                                                                index: u32);
pub fn build_insert_value, LLVMBuildInsertValue, aggval: LLVMValueRef,
                                                              eltval: LLVMValueRef,
                                                              index: u32);

// TODO: LLVMBuildAtomicRMW

pub fn build_is_null, LLVMBuildIsNull, val: LLVMValueRef);
pub fn build_is_not_null, LLVMBuildIsNotNull, val: LLVMValueRef);
pub fn build_ptr_diff, LLVMBuildPtrDiff, lhs: LLVMValueRef, rhs: LLVMValueRef);
pub fn build_fence, LLVMBuildFence, ordering: LLVMAtomicOrdering,
                                                 singlethread: LLVMBool);*/
    }

    build_ops! {
        pub fn build_ret(ret_val: LLVMValueRef) { LLVMBuildRet }
        pub fn build_ret_void() { LLVMBuildRetVoid }
        pub fn build_br(dest: LLVMBasicBlockRef) { LLVMBuildBr }

        pub fn build_cond_br(cond: LLVMValueRef,
                             then: LLVMBasicBlockRef,
                             else_: LLVMBasicBlockRef) { LLVMBuildCondBr }
    }

    pub fn build_zext(
        &mut self,
        val: LLVMValueRef,
        dst_ty: &Type,
        name: &AsRef<Str>,
    ) -> LLVMValueRef {
        unsafe {
            LLVMBuildZExt(
                self.as_mut(),
                val,
                dst_ty.into(),
                name.as_ref().as_ptr(),
            )
        }
    }

    pub fn build_icmp(
        &mut self,
        op: IntPredicate,
        lhs: LLVMValueRef,
        rhs: LLVMValueRef,
        name: &AsRef<Str>,
    ) -> LLVMValueRef {
        unsafe {
            LLVMBuildICmp(
                self.as_mut(),
                transmute(op),
                lhs,
                rhs,
                name.as_ref().as_ptr(),
            )
        }
    }

    // TODO: pub fn build_fcmp

    // TODO: check which methods should borrow mutably

    pub fn position_at_end(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_mut(), basic_block);
        }
    }

    pub fn build_call(
        &mut self,
        func: LLVMValueRef,
        args: &[LLVMValueRef],
        name: &AsRef<Str>,
    ) -> LLVMValueRef {
        unsafe {
            LLVMBuildCall(
                self.as_mut(),
                func,
                args.as_ptr() as *mut LLVMValueRef,
                args.len() as u32,
                name.as_ref().as_ptr(),
            )
        }
    }

    pub fn build_global_string(&self, s: &AsRef<Str>, name: &AsRef<Str>) -> LLVMValueRef {
        unsafe { LLVMBuildGlobalString(self.as_raw(), s.as_ref().as_ptr(), name.as_ref().as_ptr()) }
    }

    // NOTE: requires a function and basic block to be present
    pub fn build_global_string_ptr(
        &self,
        s: &AsRef<Str>,
        name: &AsRef<Str>,
    ) -> LLVMValueRef {
        unsafe {
            LLVMBuildGlobalStringPtr(
                self.as_raw(),
                s.as_ref().as_ptr(),
                name.as_ref().as_ptr(),
            )
        }
    }

    pub fn build_in_bounds_gep(
        &self,
        ptr: LLVMValueRef,
        mut indices: Vec<LLVMValueRef>,
        name: &AsRef<Str>,
    ) -> LLVMValueRef {
        unsafe {
            LLVMBuildInBoundsGEP(
                self.as_raw(),
                ptr,
                indices.as_mut_ptr(),
                indices.len() as u32,
                name.as_ref().as_ptr(),
            )
        }
    }
    pub fn build_gep(
        &self,
        ptr: LLVMValueRef,
        mut indices: Vec<LLVMValueRef>,
        name: &AsRef<Str>,
    ) -> LLVMValueRef {
        unsafe {
            LLVMBuildGEP(
                self.as_raw(),
                ptr,
                indices.as_mut_ptr(),
                indices.len() as u32,
                name.as_ref().as_ptr(),
            )
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe { LLVMDisposeBuilder(self.as_mut()) };
    }
}
