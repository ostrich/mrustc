#![feature(f16, f128)]
#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
const H: u16 = unsafe { transmute::<f16, u16>(1.5f16) };
const Q: u128 = unsafe { transmute::<f128, u128>(1.5f128) };
#[no_mangle]
pub extern "C" fn rust_half() -> u16 {
    H
}
#[no_mangle]
pub extern "C" fn rust_quad_hi() -> u64 {
    (Q >> 64) as u64
}
#[no_mangle]
pub extern "C" fn rust_quad_lo() -> u64 {
    Q as u64
}
