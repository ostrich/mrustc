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

static OTHER: u8 = 9;
#[repr(C, packed)]
pub struct Multiple {
    tag: u8,
    first: &'static u8,
    gap: u8,
    second: &'static u8,
    tail: u8,
}
static MULTIPLE: Multiple = Multiple {
    tag: 0x11,
    first: &DATA,
    gap: 0x22,
    second: &OTHER,
    tail: 0x33,
};
#[repr(C, packed)]
pub struct Text {
    tag: u8,
    text: &'static str,
    tail: u8,
}
static TEXT: Text = Text {
    tag: 0x44,
    text: "hello",
    tail: 0x55,
};
static ARRAY: [Reloc; 2] = [
    Reloc {
        prefix: 0x1111,
        pointer: &DATA,
        tail: 0x22,
    },
    Reloc {
        prefix: 0x3333,
        pointer: &OTHER,
        tail: 0x44,
    },
];
#[no_mangle]
pub extern "C" fn rust_multiple() -> *const Multiple {
    &MULTIPLE
}
#[no_mangle]
pub extern "C" fn rust_text() -> *const Text {
    &TEXT
}
#[no_mangle]
pub extern "C" fn rust_array() -> *const Reloc {
    &ARRAY as *const [Reloc; 2] as *const Reloc
}

#[repr(C, packed)]
pub struct First {
    pointer: &'static u8,
    tail: u8,
}
static FIRST: First = First {
    pointer: &OTHER,
    tail: 0x66,
};
#[no_mangle]
pub extern "C" fn rust_first() -> *const First {
    &FIRST
}
