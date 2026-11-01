// compile-flags: --edition 2021
use std::ops::{Deref, DerefMut};
struct Guard<T>(T);
impl<T> Deref for Guard<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
impl<T> DerefMut for Guard<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}
fn increment(value: &mut u32) { *value += 1; }
fn main() {
    let mut value = 0;
    {
        let mut outer = Guard(&mut value);
        let mut inner = Guard(&mut outer);
        increment(***inner);
        increment(***inner);
    }
    assert_eq!(value, 2);
    println!("nested dereference reborrows pass");
}
