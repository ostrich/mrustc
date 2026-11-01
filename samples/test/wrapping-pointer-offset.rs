// compile-flags: --edition 2021
use std::sync::atomic::{AtomicUsize, Ordering};
static TAG: AtomicUsize = AtomicUsize::new(1);
#[inline(always)]
fn clear_tag(pointer: *mut u8) -> *mut u8 {
    let address = pointer as usize;
    let destination = address & !3;
    pointer.wrapping_offset(destination.wrapping_sub(address) as isize)
}
fn main() {
    let pointer = TAG.load(Ordering::Relaxed) as *mut u8;
    assert!(clear_tag(pointer).is_null());
    assert!((4usize as *mut u32).wrapping_offset(-1).is_null());
    assert!(((usize::MAX - 3) as *mut u32).wrapping_add(1).is_null());
    let zero_sized = 16usize as *mut ();
    assert_eq!(zero_sized.wrapping_offset(isize::MIN), zero_sized);
    assert_eq!(zero_sized.wrapping_offset(isize::MAX), zero_sized);
    let mut array = [7u32, 11, 19];
    let pointer = array.as_mut_ptr();
    let roundtrip = pointer.wrapping_offset(isize::MAX).wrapping_offset(-isize::MAX);
    assert_eq!(unsafe { *roundtrip }, 7);
    println!("wrapping pointer offsets pass");
}
