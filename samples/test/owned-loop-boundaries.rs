// compile-flags: --edition 2021
use std::sync::atomic::{AtomicUsize, Ordering};
static CREATED: AtomicUsize = AtomicUsize::new(0);
static DROPPED: AtomicUsize = AtomicUsize::new(0);
struct Owned(usize);
impl Owned { fn new(n: usize) -> Owned { CREATED.fetch_add(1, Ordering::SeqCst); Owned(n) } }
impl Drop for Owned { fn drop(&mut self) { DROPPED.fetch_add(1, Ordering::SeqCst); } }
#[inline(never)] fn consume(value: Owned) { assert!(value.0 < 100); }
#[inline(never)] fn identity(value: Owned) -> Owned { value }
fn loops(mut value: Owned, iterations: usize) {
    for index in 0..iterations {
        value = Owned::new(index);
        value.0 += 1;
        if index % 3 == 0 { consume(value); continue; }
        for _ in 0..3 { value = identity(value); value.0 += 1; }
        if index % 3 == 1 { consume(value); } else { break; }
    }
}
fn partial() {
    for _ in 0..4 {
        let mut pair = (Owned::new(1), Owned::new(2));
        consume(pair.0);
        pair.0 = Owned::new(3);
        pair.1.0 += 1;
        consume(pair.1);
    }
}
fn boxed() {
    for _ in 0..4 {
        let mut boxed = Box::new((Owned::new(1), Owned::new(2)));
        consume(boxed.0);
        boxed.0 = Owned::new(3);
    }
}
fn main() {
    for count in 0..8 { loops(Owned::new(0), count); }
    partial();
    boxed();
    assert_eq!(CREATED.load(Ordering::SeqCst), DROPPED.load(Ordering::SeqCst));
}
