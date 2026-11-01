// compile-flags: --edition 2021
use std::cmp::Ordering;
const fn classify(value: Ordering) -> i32 {
    match value { Ordering::Less => -1, Ordering::Equal => 0, Ordering::Greater => 1 }
}
const LESS: i32 = classify(Ordering::Less);
const EQUAL: i32 = classify(Ordering::Equal);
const GREATER: i32 = classify(Ordering::Greater);
fn main() {
    assert_eq!((LESS, EQUAL, GREATER), (-1, 0, 1));
    println!("signed constant enum tags pass");
}
