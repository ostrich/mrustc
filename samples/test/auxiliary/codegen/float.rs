#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
const SUM: f64 = 1.25 + 2.25;
const PRODUCT: f32 = -1.5 * 2.0;
const NEG_ZERO: f64 = unsafe { transmute::<u64, f64>(0x8000000000000000) };
const NAN: f64 = unsafe { transmute::<u64, f64>(0x7ff8123456789abc) };
const NAN_BITS: u64 = unsafe { transmute::<f64, u64>(NAN) };
#[no_mangle]
pub extern "C" fn rust_double(i: u32) -> f64 {
    match i {
        0 => SUM,
        1 => NEG_ZERO,
        2 => NAN,
        3 => -12.5,
        _ => 0.0,
    }
}
#[no_mangle]
pub extern "C" fn rust_single() -> f32 {
    PRODUCT
}
#[no_mangle]
pub unsafe extern "C" fn rust_bits(i: u32) -> u64 {
    match i {
        0 => transmute::<f64, u64>(SUM),
        1 => transmute::<f64, u64>(NEG_ZERO),
        2 => NAN_BITS,
        _ => 0,
    }
}
