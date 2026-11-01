// compile-flags: --edition 2021
mod outer {
    mod definitions { pub mod core { pub struct Parser(pub usize); } }
    use self::definitions::core;
    pub use core::Parser;
    pub fn value() -> usize {
        let _: ::core::option::Option<usize> = None;
        Parser(11).0
    }
}
mod ordinary {
    use core;
    use core::option::Option;
    pub fn value() -> Option<u32> { Some(7) }
}
mod block_scope {
    mod core { pub struct Parser(pub usize); }
    pub fn value() -> usize { use core::Parser; Parser(13).0 }
}
mod value_namespace {
    fn core() -> u32 { 17 }
    use core::option::Option;
    pub fn value() -> Option<u32> { Some(core()) }
}
fn main() {
    assert_eq!(outer::value(), 11);
    assert_eq!(block_scope::value(), 13);
    assert_eq!(value_namespace::value(), Some(17));
    assert_eq!(ordinary::value(), Some(7));
}
