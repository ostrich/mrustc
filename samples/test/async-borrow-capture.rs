// compile-flags: --edition 2021
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
fn check(label: &str, values: &mut dyn Iterator<Item = usize>) {
    let mut future = Box::pin(async {
        let mut total = label.len();
        for value in values { total += value; }
        total
    });
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(Future::poll(Pin::as_mut(&mut future), &mut context), Poll::Ready(10));
}
fn main() {
    check("capture", &mut [1, 2].iter().copied());
    println!("async reference capture passes");
}
