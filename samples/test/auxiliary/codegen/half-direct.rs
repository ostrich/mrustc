#![feature(f16, f128)]
#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
const POS: u16 =
    unsafe { transmute::<f16, u16>(transmute::<u64, f64>(0x3ff0020000001000u64) as f16) };
const NEG: u16 =
    unsafe { transmute::<f16, u16>(transmute::<u64, f64>(0xbff0020000001000u64) as f16) };
#[no_mangle]
pub extern "C" fn rust_half_pos() -> u16 {
    POS
}
#[no_mangle]
pub extern "C" fn rust_half_neg() -> u16 {
    NEG
}
