#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
#[repr(C)]
pub struct Fields {
    byte: u8,
    pointer: *const u8,
}
#[no_mangle]
pub unsafe extern "C" fn rust_metric(i: u32) -> usize {
    match i {
        0 => size_of::<*const u8>(),
        1 => min_align_of::<*const u8>(),
        2 => min_align_of::<&u8>(),
        3 => min_align_of::<extern "C" fn()>(),
        4 => size_of::<Fields>(),
        5 => min_align_of::<Fields>(),
        6 => size_of::<&[u8]>(),
        7 => min_align_of::<&[u8]>(),
        _ => 0,
    }
}
