// compile-flags: --edition 2021
mod outer {
    mod core { pub struct Parser(pub usize); }
    pub use core::Parser;
    pub fn value() -> usize {
        let _: ::core::option::Option<usize> = None;
        Parser(9).0
    }
}
fn main() {
    assert_eq!(outer::value(), 9);
    println!("local module shadows implicit crate");
}
