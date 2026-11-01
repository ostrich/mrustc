// compile-flags: --edition 2021
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT: AtomicUsize = AtomicUsize::new(0);
static SEEN: Mutex<Vec<usize>> = Mutex::new(Vec::new());
struct Owned(usize);
impl Owned { fn new() -> Owned { Owned(NEXT.fetch_add(1, Ordering::SeqCst)) } }
impl Drop for Owned {
    fn drop(&mut self) {
        let mut seen = SEEN.lock().unwrap();
        assert!(!seen.contains(&self.0));
        seen.push(self.0);
    }
}
fn consume(_: Owned) {}
fn run(count: usize) {
    let mut value: Owned;
    for index in 0..count {
        value = Owned::new();
        if index % 2 == 0 { consume(value); }
    }
}
fn main() {
    for count in 0..12 { run(count); }
    let mut seen = SEEN.lock().unwrap();
    seen.sort();
    assert_eq!(seen.len(), NEXT.load(Ordering::SeqCst));
    for (index, actual) in seen.iter().enumerate() { assert_eq!(index, *actual); }
}
