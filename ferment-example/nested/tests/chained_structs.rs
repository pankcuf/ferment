#![allow(clippy::not_unsafe_ptr_arg_deref)]

use example_nested::fermented::types::dashcore::blockdata::witness::{dashcore_blockdata_witness_Witness_ctor, dashcore_blockdata_witness_Witness_destroy, dashcore_blockdata_witness_Witness_get_indices_start, dashcore_blockdata_witness_Witness_get_witness_elements, dashcore_blockdata_witness_Witness_set_witness_elements};
use example_nested::fermented::types::dashcore::prelude::{dashcore_prelude_CoreBlockHeight_ctor, dashcore_prelude_CoreBlockHeight_destroy, dashcore_prelude_CoreBlockHeight_get_0, dashcore_prelude_CoreBlockHeight_set_0};
use ferment::{FFIConversionTo};

// Chain ctor -> getters -> setters -> destroy for Witness
#[test]
fn chained_witness_get_set_destroy() {
    use example_nested::fermented::generics::Vec_u8 as FFIVecU8;

    // Build content
    let content = vec![1u8, 2, 3, 4];
    let content_ptr = unsafe { <FFIVecU8 as FFIConversionTo<Vec<u8>>>::ffi_to_const(content) };
    assert!(!content_ptr.is_null());

    // Create witness
    let w = unsafe { dashcore_blockdata_witness_Witness_ctor(content_ptr.cast_mut(), 3, 0) };
    assert!(!w.is_null());

    // Check getters
    let elems = unsafe { dashcore_blockdata_witness_Witness_get_witness_elements(w) };
    assert_eq!(elems, 3);
    let idx_start = unsafe { dashcore_blockdata_witness_Witness_get_indices_start(w) };
    assert_eq!(idx_start, 0);

    // Update counts
    unsafe { dashcore_blockdata_witness_Witness_set_witness_elements(w, 4) };
    let elems2 = unsafe { dashcore_blockdata_witness_Witness_get_witness_elements(w) };
    assert_eq!(elems2, 4);

    // Destroy witness (also frees its content pointer)
    unsafe { dashcore_blockdata_witness_Witness_destroy(w) };
}

// Chain simple numeric wrapper: CoreBlockHeight ctor -> get -> set -> get -> destroy
#[test]
fn chained_core_block_height() {

    let h = unsafe { dashcore_prelude_CoreBlockHeight_ctor(10) };
    assert!(!h.is_null());
    let v1 = unsafe { dashcore_prelude_CoreBlockHeight_get_0(h) };
    assert_eq!(v1, 10);
    unsafe { dashcore_prelude_CoreBlockHeight_set_0(h, 42) };
    let v2 = unsafe { dashcore_prelude_CoreBlockHeight_get_0(h) };
    assert_eq!(v2, 42);
    unsafe { dashcore_prelude_CoreBlockHeight_destroy(h) };
}

