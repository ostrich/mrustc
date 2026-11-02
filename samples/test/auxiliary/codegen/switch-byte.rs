#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
const fn map(v: u8) -> u32 {
    match v {
        0 => 1,
        0x7f => 2,
        0x80 => 3,
        0xfe => 4,
        0xff => 5,
        _ => 0,
    }
}
const V0: u32 = map(0);
const V1: u32 = map(0x7f);
const V2: u32 = map(0x80);
const V3: u32 = map(0xfe);
const V4: u32 = map(0xff);
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
