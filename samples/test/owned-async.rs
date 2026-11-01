// compile-flags: --edition 2021
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Poll, Wake, Waker};
static CREATED: AtomicUsize = AtomicUsize::new(0);
static DROPPED: AtomicUsize = AtomicUsize::new(0);
struct Owned;
impl Owned { fn new() -> Owned { CREATED.fetch_add(1, Ordering::SeqCst); Owned } }
impl Drop for Owned { fn drop(&mut self) { DROPPED.fetch_add(1, Ordering::SeqCst); } }
struct Yield(bool);
impl Future for Yield {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        if self.0 { Poll::Ready(()) } else { self.0 = true; Poll::Pending }
    }
}
struct Noop;
impl Wake for Noop { fn wake(self: Arc<Self>) {} }
async fn task() {
    let value = Owned::new();
    Yield(false).await;
    drop(value);
    let second = Owned::new();
    Yield(false).await;
    drop(second);
}
fn drive<F: Future<Output = ()>>(future: F, stop: usize, context: &mut Context<'_>) {
    let mut future = Box::pin(future);
    for _ in 0..stop {
        if let Poll::Ready(()) = Future::poll(future.as_mut(), context) { break; }
    }
}
fn main() {
    let waker = Waker::from(Arc::new(Noop));
    let mut context = Context::from_waker(&waker);
    for stop in 0..6 {
        drive(task(), stop, &mut context);
        assert_eq!(CREATED.load(Ordering::SeqCst), DROPPED.load(Ordering::SeqCst));
    }
}
