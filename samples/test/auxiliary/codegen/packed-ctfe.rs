#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
static DATA: u8 = 7;
#[repr(C, packed)]
struct Packed {
    tag: u8,
    pointer: &'static u8,
}
const VALUE: u8 = {
    let value = Packed {
        tag: 1,
        pointer: &DATA,
    };
    *value.pointer
};
#[no_mangle]
pub extern "C" fn rust_value() -> u8 {
    VALUE
}
