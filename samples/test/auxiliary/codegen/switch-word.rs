#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
const fn map(v: u32) -> u32 {
    match v {
        0 => 1,
        0x78 => 2,
        0x12345678 => 3,
        0x87654321 => 4,
        0xffffffff => 5,
        _ => 0,
    }
}
const V0: u32 = map(0);
const V1: u32 = map(0x78);
const V2: u32 = map(0x12345678);
const V3: u32 = map(0x87654321);
const V4: u32 = map(0xffffffff);
const V5: u32 = map(0x55);
#[no_mangle]
pub extern "C" fn rust_value(i: u32) -> u32 {
    match i {
        0 => V0,
        1 => V1,
        2 => V2,
        3 => V3,
        4 => V4,
        _ => V5,
    }
}
