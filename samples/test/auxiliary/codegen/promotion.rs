#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
#[repr(C, align(64))]
pub struct Root {
    prefix: u16,
    bytes: [u8; 3],
}
const A: Root = Root {
    prefix: 0x1234,
    bytes: [7, 8, 9],
};
const B: Root = Root {
    prefix: 0x1234,
    bytes: [8, 8, 9],
};
static STORED: Root = Root {
    prefix: 0x5678,
    bytes: [11, 12, 13],
};
const REF: &Root = &A;
#[no_mangle]
pub extern "C" fn rust_promoted(i: u32) -> *const u8 {
    match i {
        0 => &A.bytes as *const [u8; 3] as *const u8,
        1 => &B.bytes as *const [u8; 3] as *const u8,
        _ => &REF.bytes as *const [u8; 3] as *const u8,
    }
}
#[no_mangle]
pub extern "C" fn rust_static_base() -> *const u8 {
    &STORED as *const Root as *const u8
}
#[no_mangle]
pub extern "C" fn rust_static_field() -> *const u8 {
    &STORED.bytes as *const [u8; 3] as *const u8
}
