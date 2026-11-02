#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
static DATA: u8 = 7;
#[repr(C)]
pub struct Reloc {
    prefix: u16,
    pointer: &'static u8,
    tail: u8,
}
#[repr(C, packed)]
pub struct Packed {
    prefix: u8,
    pointer: &'static u8,
    tail: u8,
}
static R: Reloc = Reloc {
    prefix: 0x1234,
    pointer: &DATA,
    tail: 0x56,
};
static P: Packed = Packed {
    prefix: 0x12,
    pointer: &DATA,
    tail: 0x34,
};
#[no_mangle]
pub extern "C" fn rust_reloc() -> *const Reloc {
    &R
}
#[no_mangle]
pub extern "C" fn rust_packed() -> *const Packed {
    &P
}
