// compile-flags: --edition 2021
use std::sync::atomic::{AtomicUsize, Ordering};
static CREATED: AtomicUsize = AtomicUsize::new(0);
static DROPPED: AtomicUsize = AtomicUsize::new(0);
struct Owned;
impl Owned { fn new() -> Self { CREATED.fetch_add(1, Ordering::SeqCst); Owned } }
impl Drop for Owned { fn drop(&mut self) { DROPPED.fetch_add(1, Ordering::SeqCst); } }
struct Container { child: Owned, number: usize }
impl Drop for Container { fn drop(&mut self) { self.number += 1; } }
#[inline(never)] fn consume(_: Owned) {}
#[inline(never)] fn identity(value: Owned) -> Owned { value }
fn check() { assert_eq!(CREATED.load(Ordering::SeqCst), DROPPED.load(Ordering::SeqCst)); }
fn run(count: usize) {
    let mut value: Owned;
    for index in 0..count {
        value = Owned::new();
        if index % 2 == 0 { consume(value); }
    }
}
fn initialized(count: usize) {
    let mut value = Owned::new();
    for index in 0..count {
        value = Owned::new();
        if index % 2 == 0 { consume(value); continue; }
        value = identity(value);
        value = value;
        drop(std::mem::replace(&mut value, Owned::new()));
    }
}
fn fields(count: usize) {
    let mut value = Container { child: Owned::new(), number: 0 };
    for index in 0..count {
        value = Container { child: Owned::new(), number: 0 };
        value.child = Owned::new();
        value.number += 1;
        if index % 2 == 0 { drop(value); }
    }
}
fn main() {
    for count in 0..12 {
        run(count); check();
        initialized(count); check();
        fields(count); check();
    }
}
