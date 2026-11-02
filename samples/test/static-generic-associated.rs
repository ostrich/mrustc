use std::marker::PhantomData;
use std::mem::size_of;
static A: u8 = 7;
static B: u8 = 9;
#[repr(C, packed)]
struct Packed {
    tag: u8,
    pointer: &'static u8,
    tail: u8,
}
struct Value<T>(PhantomData<T>);
impl<T> Value<T> {
    const REF: &'static Packed = &Packed {
        tag: size_of::<T>() as u8,
        pointer: if size_of::<T>() == 4 { &A } else { &B },
        tail: 0x77,
    };
}
fn get<T>() -> &'static Packed {
    Value::<T>::REF
}
fn main() {
    let a = get::<u32>();
    let b = get::<u64>();
    assert_eq!(a.tag, 4);
    assert_eq!(*a.pointer, 7);
    assert_eq!(b.tag, 8);
    assert_eq!(*b.pointer, 9);
    assert_eq!(b.tail, 0x77);
}
