// compile-flags: --edition 2021
use std::sync::atomic::{AtomicUsize, Ordering};
static DROPS: AtomicUsize = AtomicUsize::new(0);
struct Owned(u32);
impl Drop for Owned { fn drop(&mut self) { DROPS.fetch_add(1, Ordering::SeqCst); } }
fn run() {
    let mut value = Owned(0);
    for index in 0..4 {
        value = Owned(index);
        value.0 += 1;
        if index % 2 == 0 { drop(value); }
    }
}
fn main() {
    run();
    assert_eq!(DROPS.load(Ordering::SeqCst), 5);
}
