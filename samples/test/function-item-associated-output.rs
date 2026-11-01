// compile-flags: --edition 2021
use std::ops::{BitAnd, BitOr};
fn apply(f: fn(u64, u64) -> u64, a: u64, b: u64) -> u64 { f(a, b) }
const FUNCTIONS: [fn(u64, u64) -> u64; 2] = [
    |a, b| apply(BitAnd::bitand, a, b),
    |a, b| apply(BitOr::bitor, a, b),
];
fn main() {
    assert_eq!(FUNCTIONS[0](6, 3), 2);
    assert_eq!(FUNCTIONS[1](6, 3), 7);
    println!("function item associated output coercions pass");
}
