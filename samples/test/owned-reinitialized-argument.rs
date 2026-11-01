// compile-flags: --edition 2021
use std::sync::atomic::{AtomicUsize, Ordering};
static DROPS: AtomicUsize = AtomicUsize::new(0);
struct Owned;
impl Drop for Owned {
    fn drop(&mut self) { DROPS.fetch_add(1, Ordering::SeqCst); }
}
fn run(mut value: Owned) {
    for index in 0..4 {
        value = Owned;
        if index % 2 == 0 { drop(value); }
        if index < 3 { continue; }
        return;
    }
}
fn run_local(initial: Owned) {
    let mut value = initial;
    for index in 0..4 {
        value = Owned;
        if index % 2 == 0 { drop(value); }
        if index < 3 { continue; }
        break;
    }
}
fn make() -> Owned { Owned }
fn with_call_result(mut value: Owned) {
    for index in 0..5 {
        value = make();
        if index != 4 { drop(value); continue; }
        return;
    }
}
fn main() {
    run(Owned);
    assert_eq!(DROPS.load(Ordering::SeqCst), 5);
    run_local(Owned);
    assert_eq!(DROPS.load(Ordering::SeqCst), 10);
    with_call_result(Owned);
    assert_eq!(DROPS.load(Ordering::SeqCst), 16);
    println!("reinitialized argument drops pass");
}
