#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
const C0: u32 = unsafe { transmute::<char, u32>('\0') };
const FROM_BYTE: char = 65u8 as char;
const C1: u32 = unsafe { transmute::<char, u32>(FROM_BYTE) };
const C2: u32 = unsafe { transmute::<char, u32>('\u{7f}') };
const C3: u32 = unsafe { transmute::<char, u32>('\u{80}') };
const C4: u32 = unsafe { transmute::<char, u32>('\u{ffff}') };
const C5: u32 = unsafe { transmute::<char, u32>('\u{10000}') };
const C6: u32 = unsafe { transmute::<char, u32>('\u{10ffff}') };
#[no_mangle]
pub extern "C" fn rust_value(i: u32) -> u32 {
    match i {
        0 => C0,
        1 => C1,
        2 => C2,
        3 => C3,
        4 => C4,
        5 => C5,
        6 => C6,
        _ => 0,
    }
}
