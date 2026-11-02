#![cfg_attr(mrustc, feature(no_core, lang_items, intrinsics))]
#![cfg_attr(mrustc, no_core)]
#![cfg_attr(not(mrustc), no_std)]
include!("support.rs");
#[repr(C)]
pub struct ByteC {
    value: u8,
}
#[repr(C)]
pub union ByteUnion {
    byte: u8,
}
#[repr(C, packed)]
pub struct Packed {
    byte: u8,
    word: u32,
}
#[repr(transparent)]
pub struct Transparent(u8);
pub struct Empty;
#[no_mangle]
pub unsafe extern "C" fn rust_metric(i: u32) -> usize {
    match i {
        0 => size_of::<ByteC>(),
        1 => min_align_of::<ByteC>(),
        2 => size_of::<ByteUnion>(),
        3 => min_align_of::<ByteUnion>(),
        4 => size_of::<Packed>(),
        5 => min_align_of::<Packed>(),
        6 => size_of::<Transparent>(),
        7 => min_align_of::<Transparent>(),
        8 => size_of::<[u8; 3]>(),
        9 => min_align_of::<[u8; 3]>(),
        10 => size_of::<()>(),
        11 => min_align_of::<()>(),
        12 => size_of::<Empty>(),
        13 => min_align_of::<Empty>(),
        _ => 0,
    }
}
#[repr(C, align(4))]
pub struct Aligned {
    byte: u8,
}
#[repr(C, packed(2))]
pub struct PackedTwo {
    byte: u8,
    word: u32,
}
pub struct RustByte {
    byte: u8,
}
// By-value signatures force complete C definitions and backend size assertions.
#[no_mangle]
pub extern "C" fn rust_type0(v: ByteC) -> ByteC {
    v
}
#[no_mangle]
pub extern "C" fn rust_type1(v: ByteUnion) -> ByteUnion {
    v
}
#[no_mangle]
pub extern "C" fn rust_type2(v: Packed) -> Packed {
    v
}
#[no_mangle]
pub extern "C" fn rust_type3(v: Transparent) -> Transparent {
    v
}
#[no_mangle]
pub extern "C" fn rust_type4(v: Empty) -> Empty {
    v
}
#[no_mangle]
pub extern "C" fn rust_type5(v: [u8; 3]) -> [u8; 3] {
    v
}
#[no_mangle]
pub extern "C" fn rust_type6(v: Aligned) -> Aligned {
    v
}
#[no_mangle]
pub extern "C" fn rust_type7(v: PackedTwo) -> PackedTwo {
    v
}
#[no_mangle]
pub extern "C" fn rust_type8(v: RustByte) -> RustByte {
    v
}
#[no_mangle]
pub extern "C" fn rust_extra(i: u32) -> usize {
    unsafe {
        match i {
            0 => size_of::<Aligned>(),
            1 => min_align_of::<Aligned>(),
            2 => size_of::<PackedTwo>(),
            3 => min_align_of::<PackedTwo>(),
            4 => size_of::<RustByte>(),
            _ => min_align_of::<RustByte>(),
        }
    }
}
