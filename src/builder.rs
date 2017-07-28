use super::*;

macro_rules! build_op_str {
    ($op_name: ident, $fn: path, $($argn: ident: $argv: path),*) => {
        impl Builder {
            pub fn $op_name(&mut self, $($argn: $argv),*, name: &AsRef<Str>) -> LLVMValueRef {
                unsafe { $fn(self.as_mut(), $($argn),*, name.as_ref().as_ptr()) }
            }
        }
    }
}

macro_rules! build_op {
    ($op_name: ident, $fn: path, $($argn: ident: $argv: path),*) => {
        impl Builder {
            pub fn $op_name(&mut self, $($argn: $argv),*) -> LLVMValueRef {
                unsafe { $fn(self.as_mut(), $($argn),*) }
            }
        }
    }
}

#[derive(Debug)]
pub struct Builder {
    ptr: LLVMBuilderRef
}

impl_llvm_ref!(Builder, LLVMBuilderRef);

// http://llvm.org/docs/doxygen/html/group__LLVMCCoreInstructionBuilder.html
//TODO: Get/Set Volatile
//TODO: Get/Set Ordering
//TODO: Almost everything from LLVMBuildAdd and upwards

build_op_str!(build_add, LLVMBuildAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_sub, LLVMBuildSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_mul, LLVMBuildMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_fadd, LLVMBuildFAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fsub, LLVMBuildFSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fmul, LLVMBuildFMul, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fdiv, LLVMBuildFDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_nswadd, LLVMBuildNSWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nswsub, LLVMBuildNSWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nswmul, LLVMBuildNSWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_nuwadd, LLVMBuildNUWAdd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nuwsub, LLVMBuildNUWSub, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_nuwmul, LLVMBuildNUWMul, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_udiv, LLVMBuildUDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_sdiv, LLVMBuildSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_exact_sdiv, LLVMBuildExactSDiv, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_urem, LLVMBuildURem, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_srem, LLVMBuildSRem, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_frem, LLVMBuildFRem, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_shl, LLVMBuildShl, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_lshr, LLVMBuildLShr, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_ashr, LLVMBuildAShr, lhs: LLVMValueRef, rhs: LLVMValueRef);

build_op_str!(build_and, LLVMBuildAnd, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_or, LLVMBuildOr, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_xor, LLVMBuildXor, lhs: LLVMValueRef, rhs: LLVMValueRef);

// TODO: LLVMBuildBinOp

build_op_str!(build_neg, LLVMBuildNeg, v: LLVMValueRef);
build_op_str!(build_fneg, LLVMBuildFNeg, v: LLVMValueRef);
build_op_str!(build_nswneg, LLVMBuildNSWNeg, v: LLVMValueRef);
build_op_str!(build_nuwneg, LLVMBuildNUWNeg, v: LLVMValueRef);

build_op_str!(build_not, LLVMBuildNot, v: LLVMValueRef);

build_op_str!(build_malloc, LLVMBuildMalloc, typ: LLVMTypeRef);
build_op_str!(build_array_malloc, LLVMBuildArrayMalloc, typ: LLVMTypeRef,
                                                              val: LLVMValueRef);

build_op_str!(build_alloca, LLVMBuildAlloca, ty: LLVMTypeRef);
build_op_str!(build_array_alloca, LLVMBuildArrayAlloca, ty: LLVMTypeRef,
                                                              val: LLVMValueRef);

build_op!(build_free, LLVMBuildFree, pval: LLVMValueRef);
build_op_str!(build_load, LLVMBuildLoad, ptr: LLVMValueRef);
build_op!(build_store, LLVMBuildStore,  val: LLVMValueRef, pval: LLVMValueRef);

build_op_str!(build_trunc, LLVMBuildTrunc, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_zext, LLVMBuildZExt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_sext, LLVMBuildSExt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fp_to_ui, LLVMBuildFPToUI, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fp_to_si, LLVMBuildFPToSI, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_ui_to_fp, LLVMBuildUIToFP, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_si_to_fp, LLVMBuildSIToFP, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fp_trunc, LLVMBuildFPTrunc, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fp_ext, LLVMBuildFPExt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_ptr_to_int, LLVMBuildPtrToInt, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_int_to_ptr, LLVMBuildIntToPtr, val: LLVMValueRef, dest_ty: LLVMTypeRef);

build_op_str!(build_bit_cast, LLVMBuildBitCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_addr_space_cast, LLVMBuildAddrSpaceCast, val: LLVMValueRef,
                                                                   dest_ty: LLVMTypeRef);
build_op_str!(build_zext_or_bit_cast, LLVMBuildZExtOrBitCast, val: LLVMValueRef,
                                                                    dest_ty: LLVMTypeRef);
build_op_str!(build_sext_or_bit_cast, LLVMBuildSExtOrBitCast, val: LLVMValueRef,
                                                                    dest_ty: LLVMTypeRef);
build_op_str!(build_trunc_or_bit_cast, LLVMBuildTruncOrBitCast, val: LLVMValueRef,
                                                                      dest_ty: LLVMTypeRef);

// TODO: improve LLVMOpcode
build_op_str!(build_cast, LLVMBuildCast, op: LLVMOpcode,
                                               val: LLVMValueRef,
                                               dest_ty: LLVMTypeRef);

build_op_str!(build_pointer_cast, LLVMBuildPointerCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_int_cast, LLVMBuildIntCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);
build_op_str!(build_fpcast, LLVMBuildFPCast, val: LLVMValueRef, dest_ty: LLVMTypeRef);


build_op_str!(build_icmp, LLVMBuildICmp, op: LLVMIntPredicate,
                                               lhs: LLVMValueRef,
                                               rhs: LLVMValueRef);

build_op_str!(build_fcmp, LLVMBuildFCmp, op: LLVMRealPredicate,
                                               lhs: LLVMValueRef,
                                               rhs: LLVMValueRef);

build_op_str!(build_phi, LLVMBuildPhi, ty: LLVMTypeRef);
//build_call is manually defined in impl Builder
build_op_str!(build_select, LLVMBuildSelect, i: LLVMValueRef,
                                                   the: LLVMValueRef,
                                                   els: LLVMValueRef);

build_op_str!(build_vaarg, LLVMBuildVAArg, list: LLVMValueRef, ty: LLVMTypeRef);


build_op_str!(build_extract_element, LLVMBuildExtractElement, vec_val: LLVMValueRef,
                                                                    index: LLVMValueRef);
build_op_str!(build_insert_element, LLVMBuildInsertElement, vec_val: LLVMValueRef,
                                                                  eltval: LLVMValueRef,
                                                                  index: LLVMValueRef);
build_op_str!(build_shuffle_vector, LLVMBuildShuffleVector, v1: LLVMValueRef,
                                                                  v2: LLVMValueRef,
                                                                  mask: LLVMValueRef);

// TODO: Both these types use unsigned, change this to libc::unsigned
build_op_str!(build_extract_value, LLVMBuildExtractValue, aggval: LLVMValueRef,
                                                                index: u32);
build_op_str!(build_insert_value, LLVMBuildInsertValue, aggval: LLVMValueRef,
                                                              eltval: LLVMValueRef,
                                                              index: u32);

// TODO: LLVMBuildAtomicRMW

build_op_str!(build_is_null, LLVMBuildIsNull, val: LLVMValueRef);
build_op_str!(build_is_not_null, LLVMBuildIsNotNull, val: LLVMValueRef);
build_op_str!(build_ptr_diff, LLVMBuildPtrDiff, lhs: LLVMValueRef, rhs: LLVMValueRef);
build_op_str!(build_fence, LLVMBuildFence, ordering: LLVMAtomicOrdering,
                                                 singlethread: LLVMBool);




build_op!(build_ret, LLVMBuildRet, ret_val: LLVMValueRef);
build_op!(build_ret_void, LLVMBuildRetVoid,); // TODO: Fix the trailing comma
build_op!(build_br, LLVMBuildBr, dest: LLVMBasicBlockRef);

build_op!(build_cond_br, LLVMBuildCondBr, cond: LLVMValueRef,
                                                then: LLVMBasicBlockRef,
                                                else_: LLVMBasicBlockRef);




// TODO: check which methods should borrow mutably
impl Builder {
    pub fn position_at_end(&mut self, basic_block: LLVMBasicBlockRef) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.as_mut(), basic_block);
        }
    }

    pub fn build_call(&mut self, func: LLVMValueRef, mut args: Vec<LLVMValueRef>,
                      name: &AsRef<Str>) -> LLVMValueRef {
        unsafe {
            LLVMBuildCall(
                self.as_mut(),
                func,
                args.as_mut_ptr(),
                args.len() as u32,
                name.as_ref().as_ptr()
            )
        }
    }

    pub fn build_global_string(&self, s: &AsRef<Str>, name: &AsRef<Str>) -> LLVMValueRef {
        unsafe {
            LLVMBuildGlobalString(self.as_raw(), s.as_ref().as_ptr(), name.as_ref().as_ptr())
        }
    }

    pub fn build_global_string_ptr(&self, s: &AsRef<Str>, name: &AsRef<Str>) -> LLVMValueRef {
        unsafe {
            LLVMBuildGlobalStringPtr(self.as_raw(), s.as_ref().as_ptr(), name.as_ref().as_ptr())
        }
    }

    pub fn build_in_bounds_gep(&self, ptr: LLVMValueRef, mut indices: Vec<LLVMValueRef>,
                               name: &AsRef<Str>) -> LLVMValueRef {
        unsafe {
            LLVMBuildInBoundsGEP(self.as_raw(), ptr, indices.as_mut_ptr(),
                                       indices.len() as u32, name.as_ref().as_ptr())
        }
    }
    pub fn build_gep(&self, ptr: LLVMValueRef, mut indices: Vec<LLVMValueRef>,
                               name: &AsRef<Str>) -> LLVMValueRef {
        unsafe {
            LLVMBuildGEP(self.as_raw(), ptr, indices.as_mut_ptr(),
                                       indices.len() as u32, name.as_ref().as_ptr())
        }
    }
}

impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.as_mut());
        }
    }
}

